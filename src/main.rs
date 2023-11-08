use anyhow::Result;
use clap::Parser;
use dep_graph::{DepGraph, Node};
use indexmap::IndexMap;
use is_terminal::IsTerminal;
use pulldown_cmark as pd;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

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

#[derive(Parser)]
#[command(about, version, max_term_width = 80)]
struct Cli {
    /// List available targets
    #[arg(short = 'l')]
    list_targets: bool,

    /// Force processing
    #[arg(short = 'B')]
    force_processing: bool,

    /// Dry run
    #[arg(short = 'n')]
    dry_run: bool,

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
    let cli = Cli::parse();

    if cli.readme {
        #[cfg(unix)]
        Pager::with_pager("bat -pl md").setup();

        print!("{}", include_str!("../README.md"));
        std::process::exit(0);
    }

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

    // Change directory
    if let Some(dir) = cli.change_directory {
        std::env::set_current_dir(dir)?;
    }

    if cli.config_file.exists() {
        match std::fs::read_to_string(&cli.config_file) {
            Ok(s) => {
                // Change to directory of configuration file
                std::env::set_current_dir(
                    cli.config_file.canonicalize().unwrap().parent().unwrap(),
                )?;

                // Process the configuration file
                let cfg = Config::from_markdown(&s);

                // Set color
                let color = if std::io::stdout().is_terminal() {
                    bunt::termcolor::ColorChoice::Always
                } else {
                    bunt::termcolor::ColorChoice::Never
                };
                bunt::set_stdout_color_choice(color);
                bunt::set_stderr_color_choice(color);

                // List targets
                if cli.list_targets {
                    bunt::println!("{$#ffa500}# Targets{/$}\n");
                    for target in cfg.targets.values() {
                        if target.dtg.is_none()
                            || !target.dependencies.is_empty()
                            || !target.commands.is_empty()
                        {
                            if target.dtg.is_some() {
                                bunt::println!("{$#888888}*{/$} {$#44ffff}`{}`{/$}", target.name);
                            } else {
                                bunt::println!("{$#888888}*{/$} {}", target.name);
                            }
                        }
                    }
                    println!();
                    return Ok(());
                }

                // Which targets are we processing?
                let targets = if cli.targets.is_empty() {
                    // Default / first target
                    vec![cfg.targets[0].name.clone()]
                } else {
                    // Target(s) specified on the command line
                    cli.targets.clone()
                };

                // Process the targets
                let mut processed = HashSet::new();
                for target in &targets {
                    let mut nodes = vec![];
                    add_node_and_deps(target, &cfg, &mut nodes, &mut processed);
                    DepGraph::new(&nodes).into_iter().for_each(|x| {
                        process_target(&x, &cfg.targets, cli.dry_run, cli.force_processing);
                    });
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
) {
    let target = target.to_string();
    let mut node = Node::new(target.clone());
    if let Some(t) = cfg.targets.get(&target) {
        for dependency in &t.dependencies {
            node.add_dep(dependency.to_owned());
            add_node_and_deps(dependency, cfg, nodes, processed);
        }
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
) {
    let target = target.to_owned();
    let target = targets.get(&target).unwrap();
    if let Some(ts) = target.dtg.as_ref() {
        // File target...
        let file_does_not_exist = !Path::new(&target.name).exists();
        if target.commands.is_empty() {
            if file_does_not_exist {
                // File dependency (without commands) must exist
                error!(3, "ERROR: File `{}` does not exist!", target.name);
            }
            // Otherwise, file dependency exists so don't print or do anything
        } else if force_processing || file_does_not_exist || target.outdated(ts, targets) {
            // Process the target if any of:
            // * `-B`
            // * file doesn't exist & target has commands
            // * target is outdated
            target.print_heading();
            target.run(dry_run);
        } else {
            // Otherwise, don't process the target
            target.print_heading();
            bunt::println!("{$#00ff00+italic}*Up to date*{/$}\n");
        }
    } else {
        // "Phony" target
        target.print_heading();
        target.run(dry_run);
    }
}

fn run(command: &str, dry_run: bool) {
    bunt::println!("{$#555555}```text\n${/$} {$#00ffff+bold}{}{/$}", command);
    if !dry_run
        && std::process::Command::new("sh")
            .args(["-c", command])
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .code()
            != Some(0)
    {
        bunt::println!("{$#555555}```{/$}\n");
        error!(4, "ERROR: The command failed!");
    }
    bunt::println!("{$#555555}```{/$}\n");
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
        let mut in_commands = false;
        let mut is_file = false;
        let mut name: Option<String> = None;
        let mut dependencies = vec![];
        let mut commands = vec![];
        for event in pd::Parser::new_ext(s, pd::Options::all()) {
            match event {
                pd::Event::Start(pd::Tag::Heading(pd::HeadingLevel::H1, ..)) => {
                    if let Some(n) = name.take() {
                        let target = Target::new(&n, is_file, &dependencies, &commands);
                        targets.insert(n, target);
                        name = None;
                        is_file = false;
                        dependencies = vec![];
                        commands = vec![];
                    }
                    in_h1 = true;
                }
                pd::Event::Code(s) => {
                    if in_h1 {
                        is_file = true;
                        name = Some(s.to_string());
                    } else if in_dependencies {
                        dependencies.push(s.to_string());
                    }
                }
                pd::Event::Text(s) => {
                    if in_h1 {
                        is_file = false;
                        name = Some(s.to_string());
                    } else if in_dependencies {
                        dependencies.push(s.to_string());
                    } else if in_commands {
                        commands = s
                            .replace("\\\n", "")
                            .lines()
                            .filter_map(|x| {
                                let command = x
                                    .replace(
                                        "{0}",
                                        if dependencies.is_empty() {
                                            "{0}"
                                        } else {
                                            &dependencies[0]
                                        },
                                    )
                                    .replace("{target}", name.as_ref().unwrap());
                                if command.is_empty() {
                                    None
                                } else {
                                    Some(command)
                                }
                            })
                            .collect();
                    }
                }
                pd::Event::End(pd::Tag::Heading(pd::HeadingLevel::H1, ..)) => {
                    in_h1 = false;
                }
                pd::Event::Start(pd::Tag::List(None)) => {
                    in_dependencies = true;
                }
                pd::Event::End(pd::Tag::List(None)) => {
                    in_dependencies = false;
                }
                pd::Event::Start(pd::Tag::CodeBlock(pd::CodeBlockKind::Fenced(_info))) => {
                    in_commands = true;
                }
                pd::Event::End(pd::Tag::CodeBlock(pd::CodeBlockKind::Fenced(_info))) => {
                    in_commands = false;
                }
                _ => {}
            }
        }

        // Add the last target
        if let Some(n) = name.take() {
            let target = Target::new(&n, is_file, &dependencies, &commands);
            targets.insert(n, target);
        }

        // Add files mentioned as dependencies but not targets in configuration
        let mut file_targets = vec![];
        for target in targets.values() {
            for dependency in &target.dependencies {
                if !targets.contains_key(dependency) {
                    file_targets
                        .push((dependency.clone(), Target::new(dependency, true, &[], &[])));
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
struct Target {
    name: String,
    dtg: Option<std::time::SystemTime>,
    dependencies: Vec<String>,
    commands: Vec<String>,
}

impl Target {
    fn new(name: &str, is_file: bool, dependencies: &[String], commands: &[String]) -> Target {
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
            commands: commands.to_owned(),
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
            true
        }
    }

    fn print_heading(&self) {
        if self.dtg.is_some() {
            bunt::println!("{$#ff22ff+bold}# `{}`{/$}\n", self.name);
        } else {
            bunt::println!("{$#ff22ff+bold}# {}{/$}\n", self.name);
        }
    }

    fn run(&self, dry_run: bool) {
        for command in &self.commands {
            run(command, dry_run);
        }
    }
}
