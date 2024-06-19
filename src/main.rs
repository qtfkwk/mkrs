use anyhow::Result;
use clap::{ArgAction::Count, Parser};
use dep_graph::{DepGraph, Node};
use glob::glob;
use indexmap::IndexMap;
use pulldown_cmark as pd;
use sprint::{Command, Pipe, Shell};
use std::{
    collections::HashSet,
    io::IsTerminal,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use pager::Pager;

//--------------------------------------------------------------------------------------------------

macro_rules! error {
    ($code:expr, $($x:tt)*) => {
        bunt::eprintln!("{$red+bold}{}{/$}", format!($($x)*));
        std::process::exit($code);
    };
}

//--------------------------------------------------------------------------------------------------

fn print_file_target(name: &str) {
    bunt::println!("{$#ff22ff+bold}# `{}`{/$}\n", name);
}

fn print_target(name: &str) {
    bunt::println!("{$#ff22ff+bold}# {}{/$}\n", name);
}

fn print_list_file_target(name: &str, level: usize) {
    print_indent(level);
    bunt::println!("{$#888888}*{/$} {$#44ffff}`{}`{/$}", name);
}

fn print_indent(level: usize) {
    if level > 0 {
        print!("{}", " ".repeat(level * 4));
    }
}

fn print_list_target(name: &str, level: usize) {
    print_indent(level);
    bunt::println!("{$#888888}*{/$} {}", name);
}

fn print_up_to_date() {
    bunt::println!("{$#00ff00+italic}*Up to date*{/$}\n");
}

fn print_end_fence() {
    bunt::println!("{$#555555}```{/$}\n");
}

fn print_fence() {
    bunt::println!("{$#555555}```{/$}");
}

fn set_terminal_colors() {
    // stdout
    bunt::set_stdout_color_choice(if std::io::stdout().is_terminal() {
        bunt::termcolor::ColorChoice::Always
    } else {
        bunt::termcolor::ColorChoice::Never
    });

    // stderr
    bunt::set_stderr_color_choice(if std::io::stderr().is_terminal() {
        bunt::termcolor::ColorChoice::Always
    } else {
        bunt::termcolor::ColorChoice::Never
    });
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

#[derive(Debug, Parser)]
#[command(about, version, max_term_width = 80)]
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

    /// Configuration file
    #[arg(short = 'f', default_value = "Makefile.md", value_name = "PATH")]
    config_file: PathBuf,

    /// Generate Makefile.md content [styles: rust]
    #[arg(short = 'g', value_name = "STYLE")]
    generate: Option<String>,

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

    // Set terminal colors
    set_terminal_colors();

    // Print configuration
    if cli.verbose >= 3 {
        bunt::println!("{$#ffff22+bold}# Configuration{/$}\n");
        print_fence();
        println!("{cli:#?}");
        print_end_fence();
    }

    // Process the configuration file (`Makefile.md` or `-f PATH`)
    if cli.config_file.exists() {
        match std::fs::read_to_string(&cli.config_file) {
            Ok(s) => {
                // Change to directory of configuration file
                std::env::set_current_dir(
                    cli.config_file.canonicalize().unwrap().parent().unwrap(),
                )?;

                // Parse the content to a `Config`
                let cfg = Config::from_markdown(&s);

                if cli.verbose >= 3 {
                    print_fence();
                    println!("{cfg:#?}");
                    print_end_fence();
                }

                // List targets (`-l`)
                if cli.list_targets {
                    if cli.targets.is_empty() {
                        for target in cfg.targets.values() {
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
                            if !cfg.targets.contains_key(target) {
                                error!(5, "ERROR: Invalid target: `{target}`!");
                            }
                        }
                        for target in &cli.targets {
                            print_list_file_targets(target, &cfg.targets, 0);
                        }
                    }
                    println!();
                    return Ok(());
                }

                // Which target(s) are we processing?
                let targets = if cli.targets.is_empty() {
                    // First target in `Makefile.md`
                    vec![cfg.targets[0].name.clone()]
                } else {
                    // Target(s) specified on the command line
                    cli.targets.clone()
                };

                // Process the target(s)
                let mut processed = HashSet::new();
                for target in &targets {
                    let mut nodes = vec![];
                    add_node_and_deps(
                        target,
                        &cfg,
                        &mut nodes,
                        &mut processed,
                        cli.force_processing,
                        &cfg.targets,
                        None,
                    );
                    let num_nodes = nodes.len();
                    if num_nodes > 1 {
                        DepGraph::new(&nodes).into_iter().for_each(|x| {
                            process_target(
                                &x,
                                &cfg.targets,
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
                            &cfg.targets,
                            cli.dry_run,
                            cli.force_processing,
                            cli.verbose,
                            cli.quiet,
                            cli.script_mode,
                        );
                    }
                }
            }
            Err(e) => {
                error!(2, "ERROR: {e}!");
            }
        }
    } else {
        error!(1, "ERROR: Please create a `{}`!", cli.config_file.display());
    }

    Ok(())
}

//--------------------------------------------------------------------------------------------------

fn add_node_and_deps(
    target: &str,
    cfg: &Config,
    nodes: &mut Vec<Node<String>>,
    processed: &mut HashSet<String>,
    force_processing: bool,
    targets: &IndexMap<String, Target>,
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
            force_processing || file_does_not_exist || t.outdated(ts, targets)
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
                    targets,
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

impl Config {
    fn from_markdown(s: &str) -> Config {
        let mut targets = IndexMap::new();
        let mut in_h1 = false;
        let mut in_dependencies = false;
        let mut in_recipe = None;
        let mut is_file = false;
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
                        let target =
                            Target::new(&n, is_file, &dependencies, std::mem::take(&mut recipes));
                        targets.insert(n, target);
                        name = None;
                        is_file = false;
                        dependencies = vec![];
                    }
                    in_h1 = true;
                }
                pd::Event::Code(s) => {
                    if in_h1 {
                        is_file = true;
                        name = Some(s.to_string());
                    } else if in_dependencies {
                        let mut globbed = glob(&s)
                            .expect("glob")
                            .filter_map(|x| match x {
                                Ok(p) => Some(p.display().to_string()),
                                Err(_e) => None,
                            })
                            .collect::<Vec<_>>();
                        if globbed.is_empty() {
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
                        if let Some(shell) = shell {
                            recipes.push(Recipe::new(
                                Some(shell),
                                vec![if dependencies.is_empty() {
                                    s.trim().replace("{target}", name.as_ref().unwrap())
                                } else {
                                    s.trim()
                                        .replace("{target}", name.as_ref().unwrap())
                                        .replace("{0}", &dependencies[0])
                                }],
                            ));
                        } else {
                            recipes.push(Recipe::new(
                                None,
                                s.replace("\\\n", "")
                                    .lines()
                                    .filter_map(|x| {
                                        let command = x
                                            .trim()
                                            .replace(
                                                "{0}",
                                                if dependencies.is_empty() {
                                                    "{0}"
                                                } else {
                                                    &dependencies[0]
                                                },
                                            )
                                            .replace("{target}", name.as_ref().unwrap());
                                        if command.is_empty() || command.starts_with('#') {
                                            None
                                        } else {
                                            Some(command)
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
            let target = Target::new(&n, is_file, &dependencies, recipes);
            targets.insert(n, target);
        }

        // Add files mentioned as dependencies but not targets in configuration
        let mut file_targets = vec![];
        for target in targets.values() {
            for dependency in &target.dependencies {
                if !targets.contains_key(dependency) {
                    file_targets.push((
                        dependency.clone(),
                        Target::new(dependency, true, &[], vec![]),
                    ));
                }
            }
        }
        for (name, target) in file_targets {
            targets.insert(name, target);
        }

        Config { targets }
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
}

//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Target {
    name: String,
    dtg: Option<std::time::SystemTime>,
    dependencies: Vec<String>,
    recipes: Vec<Recipe>,
}

impl Target {
    fn new(name: &str, is_file: bool, dependencies: &[String], recipes: Vec<Recipe>) -> Target {
        Target {
            name: name.to_owned(),
            dtg: if is_file {
                match std::fs::metadata(name) {
                    Ok(m) => m.modified().ok(),
                    Err(_e) => Some(std::time::SystemTime::UNIX_EPOCH),
                }
            } else {
                None
            },
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
