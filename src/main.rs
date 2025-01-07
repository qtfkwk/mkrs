use {
    anstream::{eprint, print},
    anyhow::{anyhow, Result},
    clap::{builder::Styles, ArgAction::Count, Parser},
    dep_graph::{DepGraph, Node},
    expanduser::expanduser,
    glob::glob,
    globset::{Glob, GlobMatcher},
    indexmap::IndexMap,
    lazy_static::lazy_static,
    owo_colors::{OwoColorize, Style},
    pulldown_cmark as pd,
    regex::Regex,
    sprint::{style, ColorOverride, Command, Pipe, Shell},
    std::{
        collections::HashSet,
        path::{Path, PathBuf},
    },
};

#[cfg(unix)]
use pager::Pager;

//--------------------------------------------------------------------------------------------------

macro_rules! cprint {
    ($style:expr, $($x:tt)*) => {
        print!("{}", format!($($x)*).style($style));
    };
}

macro_rules! ecprint {
    ($style:expr, $($x:tt)*) => {
        eprint!("{}", format!($($x)*).style($style));
    };
}

macro_rules! error {
    ($code:expr, $($x:tt)*) => {
        ecprint!(*ERROR, $($x)*);
        std::process::exit($code);
    };
}

//--------------------------------------------------------------------------------------------------

lazy_static! {
    static ref BULLET: Style = style("#888888").expect("style");
    static ref CONFIGURATION: Style = style("#FFFF22+bold").expect("style");
    static ref ERROR: Style = style("red+bold").expect("style");
    static ref FENCE: Style = style("#555555").expect("style");
    static ref FILE_TARGET: Style = style("#44FFFF+bold").expect("style");
    static ref TARGET: Style = style("#FF22FF+bold").expect("style");
    static ref UP_TO_DATE: Style = style("#00FF00+italic").expect("style");
}

fn print_file_target(name: &str) {
    cprint!(*TARGET, "# `{name}`\n\n");
}

fn print_target(name: &str) {
    cprint!(*TARGET, "# {name}\n\n");
}

fn print_list_file_target(name: &str, level: usize) {
    print_bullet(level);
    cprint!(*FILE_TARGET, "`{name}`\n");
}

fn print_bullet(level: usize) {
    print_indent(level);
    cprint!(*BULLET, "* ");
}

fn print_indent(level: usize) {
    if level > 0 {
        print!("{}", " ".repeat(level * 4));
    }
}

fn print_list_target(name: &str, level: usize) {
    print_bullet(level);
    println!("{name}");
}

fn print_up_to_date() {
    cprint!(*UP_TO_DATE, "*Up to date*\n");
}

fn print_fence() {
    cprint!(*FENCE, "```");
}

fn print_end_fence() {
    print_fence();
    println!("\n");
}

fn print_list_file_targets(target: &str, targets: &IndexMap<String, Target>, level: usize) {
    let target = targets.get(target).unwrap();
    if target.dtg.is_some() {
        print_list_file_target(&target.name, level);
    } else {
        print_list_target(&target.name, level);
    }
    for dep in &target.dependencies {
        print_list_file_targets(dep, targets, level + 1);
    }
}

//--------------------------------------------------------------------------------------------------

const STYLES: Styles = Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Debug, Parser)]
#[command(about, version, max_term_width = 80, styles = STYLES)]
struct Cli {
    /// List targets/dependencies
    #[arg(short = 'l')]
    list_targets: bool,

    /// Force processing
    #[arg(short = 'B')]
    force_processing: bool,

    /// Dry run
    #[arg(short = 'n')]
    dry_run: bool,

    /// Script mode
    #[arg(short)]
    script_mode: bool,

    /// Verbose
    #[arg(short, action = Count)]
    verbose: u8,

    /// Quiet
    #[arg(short, conflicts_with = "verbose")]
    quiet: bool,

    /// Change directory
    #[arg(short = 'C', value_name = "PATH")]
    change_directory: Option<PathBuf>,

    /// Configuration file(s)
    #[arg(short = 'f', default_value = "Makefile.md", value_name = "PATH")]
    config_files: Vec<PathBuf>,

    /// Generate Makefile.md content [styles: rust]
    #[arg(short = 'g', value_name = "STYLE")]
    generate: Option<String>,

    /// Force enable/disable terminal colors
    #[arg(long, value_enum, global = true, default_value = "auto")]
    color: ColorOverride,

    /// Print readme
    #[arg(short)]
    readme: bool,

    /// Target(s)
    #[arg(value_name = "NAME")]
    targets: Vec<String>,
}

//--------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    if cfg!(windows) {
        error!(255, "ERROR: Windows is not a supported operating system!");
    }

    let cli = Cli::parse();

    cli.color.init();

    // Print the readme (`-r`)
    if cli.readme {
        #[cfg(unix)]
        Pager::with_pager("bat -pl md").setup();

        print!("{}", include_str!("../README.md"));
        std::process::exit(0);
    }

    // Generate default Makefile.md content (`-g STYLE`)
    if let Some(style) = cli.generate {
        match style.as_str() {
            "rust" => {
                print!("{}", include_str!("../styles/Makefile.rust.md"));
            }
            _ => {
                error!(6, "ERROR: Invalid style: `{style}`!");
            }
        }
        std::process::exit(0);
    }

    // Change directory (`-C`)
    if let Some(dir) = &cli.change_directory {
        std::env::set_current_dir(dir)?;
    }

    // Print CLI configuration
    if cli.verbose >= 3 {
        cprint!(*CONFIGURATION, "# Configuration\n\n");
        print_fence();
        println!("\n{cli:#?}");
        print_end_fence();
    }

    // Process targets
    Config::from(&cli.config_files)?.process(&cli)?;

    Ok(())
}

//--------------------------------------------------------------------------------------------------

fn add_node_and_deps(
    target: &str,
    cfg: &Config,
    nodes: &mut Vec<Node<String>>,
    processed: &mut HashSet<String>,
    force_processing: bool,
    prev_dep: Option<String>,
) {
    let target = target.to_string();
    let mut node = Node::new(target.clone());
    if let Some(prev_dep) = prev_dep {
        node.add_dep(prev_dep);
    }
    if let Some(t) = cfg.targets.get(&target) {
        // If a file target, only add its dependencies if it is needed
        let add_deps = if let Some(ts) = t.dtg.as_ref() {
            let file_does_not_exist = !Path::new(&t.name).exists();
            force_processing || file_does_not_exist || t.outdated(ts, &cfg.targets)
        } else {
            true
        };
        if add_deps {
            let mut prev_dep = None;
            for dependency in &t.dependencies {
                node.add_dep(dependency.to_owned());
                add_node_and_deps(
                    dependency,
                    cfg,
                    nodes,
                    processed,
                    force_processing,
                    prev_dep,
                );
                prev_dep = Some(dependency.to_owned());
            }
        }

        // Deduplicate nodes
        if !processed.contains(&target) {
            nodes.push(node);
            processed.insert(target);
        }
    } else {
        error!(5, "ERROR: Invalid target: `{target}`!");
    }
}

fn process_target(
    target: &str,
    targets: &IndexMap<String, Target>,
    dry_run: bool,
    force_processing: bool,
    verbose: u8,
    quiet: bool,
    script_mode: bool,
) {
    let target = target.to_owned();
    let target = targets.get(&target).unwrap();
    if let Some(ts) = target.dtg.as_ref() {
        // File target...
        let file_does_not_exist = !Path::new(&target.name).exists();
        if target.recipes.is_empty() {
            // Try wildcard target
            for t in targets.values() {
                if let Some(glob) = t.glob.as_ref() {
                    if glob.is_match(&target.name) {
                        let re = Regex::new(&format!("{}$", &t.name[2..])).expect("regex");
                        let extension = &t.dependencies[0][2..];
                        let dependency = re.replace(&target.name, extension).to_string();
                        let target_does_not_exist = !Path::new(&target.name).exists();
                        if force_processing
                            || target_does_not_exist
                            || outdated(&dependency, &target.name)
                        {
                            Target::new(
                                &target.name,
                                true,
                                None,
                                &[dependency.clone()],
                                t.recipes
                                    .iter()
                                    .map(|x| x.fix(&target.name, &dependency))
                                    .collect(),
                            )
                            .run(dry_run, verbose, quiet, script_mode);
                            return;
                        }
                    }
                }
            }

            if file_does_not_exist {
                // File dependency (without commands) must exist
                error!(3, "ERROR: File `{}` does not exist!", target.name);
            }
            // Otherwise, file dependency exists so don't print or do anything
        } else if force_processing || file_does_not_exist || target.outdated(ts, targets) {
            // Process the target if `-B`, target has commands & file doesn't exist, or target is
            // outdated
            target.run(dry_run, verbose, quiet, script_mode);
        } else if verbose >= 2 {
            // Otherwise, don't process the target
            target.print_heading();
            print_up_to_date();
        }
    } else {
        // "Phony" target
        target.run(dry_run, verbose, quiet, script_mode);
    }
}

fn run(command: &str, dry_run: bool, quiet: bool) {
    let results = Shell {
        dry_run,
        print: !quiet,
        ..Default::default()
    }
    .run(&[Command::new(command)]);

    for result in &results {
        exit_if_failed(result, dry_run);
    }
}

fn run_script(script: &str, dry_run: bool, verbose: u8, quiet: bool, shell: Option<String>) {
    let command = if let Some(command) = &shell {
        command
    } else if verbose >= 1 {
        "bash -xeo pipefail"
    } else {
        "bash -eo pipefail"
    };

    let result = Shell {
        dry_run,
        print: !quiet,
        ..Default::default()
    }
    .core(&Command {
        command: command.to_string(),
        stdin: Pipe::String(Some(script.to_string())),
        ..Default::default()
    });

    exit_if_failed(&result, dry_run);
}

fn exit_if_failed(result: &Command, dry_run: bool) {
    if let Some(code) = &result.code {
        if !result.codes.contains(code) {
            std::process::exit(*code);
        }
    } else if !dry_run {
        std::process::exit(1);
    }
}

//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Config {
    targets: IndexMap<String, Target>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            targets: IndexMap::new(),
        }
    }
}

impl Config {
    fn from(config_files: &[PathBuf]) -> Result<Config> {
        let mut r = Config::default();
        let dirname = std::env::current_dir()?
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        for config_file in config_files {
            r.load(config_file, &dirname)?;
        }
        Ok(r)
    }

    fn load(&mut self, config_file: &Path, dirname: &str) -> Result<()> {
        if config_file.exists() {
            match std::fs::read_to_string(config_file) {
                Ok(s) => {
                    self.load_markdown(&s, dirname);
                    Ok(())
                }
                Err(e) => Err(anyhow!("{e}")),
            }
        } else {
            Err(anyhow!(
                "Configuration file `{}` does not exist!",
                config_file.display(),
            ))
        }
    }

    fn load_markdown(&mut self, s: &str, dirname: &str) {
        let mut in_h1 = false;
        let mut in_dependencies = false;
        let mut in_recipe = None;
        let mut is_file = false;
        let mut is_glob = false;
        let mut name: Option<String> = None;
        let mut dependencies = vec![];
        let mut recipes = vec![];
        for event in pd::Parser::new_ext(s, pd::Options::all()) {
            match event {
                pd::Event::Start(pd::Tag::Heading {
                    level: pd::HeadingLevel::H1,
                    ..
                }) => {
                    if let Some(n) = name.take() {
                        // Push staged target
                        let target = Target::new(
                            &n,
                            is_file,
                            glob_matcher(&n, is_glob),
                            &dependencies,
                            std::mem::take(&mut recipes),
                        );
                        self.targets.insert(n, target);

                        // Reset
                        name = None;
                        is_file = false;
                        is_glob = false;
                        dependencies = vec![];
                    }
                    in_h1 = true;
                }
                pd::Event::Code(s) => {
                    let s = s.replace("{dirname}", dirname);
                    if in_h1 {
                        if s.starts_with("*.") && s.len() > 2 {
                            is_glob = true;
                            name = Some(s);
                        } else {
                            is_file = true;
                            name = Some(s);
                        }
                    } else if in_dependencies {
                        let s = expanduser(&s).unwrap().display().to_string();
                        let mut globbed = glob(&s)
                            .expect("glob")
                            .filter_map(|x| x.map(|x| x.display().to_string()).ok())
                            .collect::<Vec<_>>();
                        if globbed.is_empty() || is_glob {
                            dependencies.push(s.to_string());
                        } else {
                            dependencies.append(&mut globbed);
                        }
                    }
                }
                pd::Event::Text(s) => {
                    if in_h1 {
                        is_file = false;
                        name = Some(s.to_string());
                    } else if in_dependencies {
                        dependencies.push(s.to_string());
                    } else if let Some(shell) = in_recipe.take() {
                        let s = if is_glob {
                            s.trim().to_string()
                        } else {
                            s.trim()
                                .replace("{target}", name.as_ref().unwrap())
                                .replace("{dirname}", dirname)
                        };

                        let s = if dependencies.is_empty() || is_glob {
                            s
                        } else {
                            s.replace("{0}", &dependencies[0])
                        };

                        if let Some(shell) = shell {
                            recipes.push(Recipe::new(Some(shell), vec![s]));
                        } else {
                            recipes.push(Recipe::new(
                                None,
                                s.replace("\\\n", "")
                                    .lines()
                                    .filter_map(|x| {
                                        if x.is_empty() || x.starts_with('#') {
                                            None
                                        } else {
                                            Some(x.to_string())
                                        }
                                    })
                                    .collect(),
                            ));
                        }
                    }
                }
                pd::Event::End(pd::TagEnd::Heading(pd::HeadingLevel::H1, ..)) => {
                    in_h1 = false;
                }
                pd::Event::Start(pd::Tag::List(None)) => {
                    in_dependencies = true;
                }
                pd::Event::End(pd::TagEnd::List(false)) => {
                    in_dependencies = false;
                }
                pd::Event::Start(pd::Tag::CodeBlock(pd::CodeBlockKind::Fenced(info))) => {
                    let info = info.to_string();
                    in_recipe = if info.is_empty() {
                        Some(None)
                    } else {
                        Some(Some(info))
                    };
                }
                pd::Event::End(pd::TagEnd::CodeBlock) => {
                    in_recipe = None;
                }
                _ => {}
            }
        }

        // Add the last target
        if let Some(n) = name.take() {
            let target = Target::new(
                &n,
                is_file,
                glob_matcher(&n, is_glob),
                &dependencies,
                recipes,
            );
            self.targets.insert(n, target);
        }

        // Add files mentioned as dependencies but not targets in configuration
        let mut file_targets = vec![];
        for target in self.targets.values() {
            for dependency in &target.dependencies {
                if !self.targets.contains_key(dependency) {
                    file_targets.push((
                        dependency.clone(),
                        Target::new(dependency, true, None, &[], vec![]),
                    ));
                }
            }
        }
        for (name, target) in file_targets {
            self.targets.insert(name, target);
        }
    }

    fn process(&mut self, cli: &Cli) -> Result<()> {
        if cli.verbose >= 3 {
            print_fence();
            println!("\n{self:#?}");
            print_end_fence();
        }

        // List targets (`-l`)
        if cli.list_targets {
            if cli.targets.is_empty() {
                for target in self.targets.values() {
                    if target.dtg.is_none()
                        || !target.dependencies.is_empty()
                        || !target.recipes.is_empty()
                    {
                        if target.dtg.is_some() {
                            print_list_file_target(&target.name, 0);
                        } else {
                            print_list_target(&target.name, 0);
                        }
                    }
                }
            } else {
                for target in &cli.targets {
                    if !self.targets.contains_key(target) {
                        error!(5, "ERROR: Invalid target: `{target}`!");
                    }
                }
                for target in &cli.targets {
                    print_list_file_targets(target, &self.targets, 0);
                }
            }
            println!();
            return Ok(());
        }

        // Which target(s) are we processing?
        let targets = if cli.targets.is_empty() {
            // First target in `Makefile.md`
            vec![self.targets[0].name.clone()]
        } else {
            // Target(s) specified on the command line
            cli.targets.clone()
        };

        // Process the target(s)
        let mut processed = HashSet::new();
        for target in &targets {
            // Generate target from wildcard/glob target
            if !self.targets.contains_key(target) {
                for (_, t) in &self.targets {
                    if let Some(glob) = t.glob.as_ref() {
                        if glob.is_match(target) {
                            let re = Regex::new(&format!("{}$", &t.name[2..])).expect("regex");
                            let extension = &t.dependencies[0][2..];
                            let dependency = re.replace(target, extension).to_string();
                            let target_does_not_exist = !Path::new(target).exists();
                            if cli.force_processing
                                || target_does_not_exist
                                || outdated(&dependency, target)
                            {
                                let t = Target::new(
                                    target,
                                    true,
                                    None,
                                    &[dependency.clone()],
                                    t.recipes
                                        .iter()
                                        .map(|x| x.fix(target, &dependency))
                                        .collect(),
                                );
                                self.targets.insert(target.clone(), t);
                            }
                            break;
                        }
                    }
                }
            }

            let mut nodes = vec![];
            add_node_and_deps(
                target,
                self,
                &mut nodes,
                &mut processed,
                cli.force_processing,
                None,
            );
            let num_nodes = nodes.len();
            if num_nodes > 1 {
                DepGraph::new(&nodes).into_iter().for_each(|x| {
                    process_target(
                        &x,
                        &self.targets,
                        cli.dry_run,
                        cli.force_processing,
                        cli.verbose,
                        cli.quiet,
                        cli.script_mode,
                    );
                });
            } else if num_nodes > 0 {
                process_target(
                    nodes[0].id(),
                    &self.targets,
                    cli.dry_run,
                    cli.force_processing,
                    cli.verbose,
                    cli.quiet,
                    cli.script_mode,
                );
            }
        }

        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Recipe {
    shell: Option<String>,
    commands: Vec<String>,
}

impl Recipe {
    fn new(shell: Option<String>, commands: Vec<String>) -> Recipe {
        Recipe { shell, commands }
    }

    fn run(&self, dry_run: bool, verbose: u8, quiet: bool, script_mode: bool) {
        if let Some(shell) = &self.shell {
            run_script(
                &self.commands.join("\n"),
                dry_run,
                verbose,
                quiet,
                Some(shell.clone()),
            );
        } else if script_mode {
            run_script(&self.commands.join("\n"), dry_run, verbose, quiet, None);
        } else {
            for command in &self.commands {
                run(command, dry_run, quiet);
            }
        }
    }

    fn fix(&self, target: &str, dependency: &str) -> Recipe {
        Recipe {
            shell: self.shell.clone(),
            commands: self
                .commands
                .iter()
                .map(|x| x.replace("{0}", dependency).replace("{target}", target))
                .collect(),
        }
    }
}

//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Target {
    name: String,
    glob: Option<GlobMatcher>,
    dtg: Option<std::time::SystemTime>,
    dependencies: Vec<String>,
    recipes: Vec<Recipe>,
}

impl Target {
    fn new(
        name: &str,
        is_file: bool,
        glob: Option<GlobMatcher>,
        dependencies: &[String],
        recipes: Vec<Recipe>,
    ) -> Target {
        Target {
            name: name.to_owned(),
            glob,
            dtg: is_file.then(|| mtime(name)),
            dependencies: dependencies.to_owned(),
            recipes,
        }
    }

    fn outdated(
        &self,
        reference: &std::time::SystemTime,
        targets: &IndexMap<String, Target>,
    ) -> bool {
        if let Some(ts) = self.dtg.as_ref() {
            if ts > reference {
                true
            } else {
                self.dependencies
                    .iter()
                    .any(|x| targets.get(x).unwrap().outdated(reference, targets))
            }
        } else {
            false
        }
    }

    fn print_heading(&self) {
        if self.dtg.is_some() {
            print_file_target(&self.name);
        } else {
            print_target(&self.name);
        }
    }

    fn run(&self, dry_run: bool, verbose: u8, quiet: bool, script_mode: bool) {
        if !quiet && (!self.recipes.is_empty() || verbose >= 2) {
            self.print_heading();
        }
        for recipe in &self.recipes {
            recipe.run(dry_run, verbose, quiet, script_mode);
        }
    }
}

//--------------------------------------------------------------------------------------------------

fn glob_matcher(n: &str, is_glob: bool) -> Option<GlobMatcher> {
    is_glob.then(|| Glob::new(n).expect("glob").compile_matcher())
}

/// Get the modified time of a file
fn mtime(file: &str) -> std::time::SystemTime {
    match std::fs::metadata(file) {
        Ok(m) => m.modified().expect("modified"),
        Err(_e) => std::time::SystemTime::UNIX_EPOCH,
    }
}

/// Return true if the reference file is newer than the file
fn outdated(ref_file: &str, file: &str) -> bool {
    mtime(ref_file) > mtime(file)
}
