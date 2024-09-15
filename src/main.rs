use bevy::prelude::*;
use clap::{CommandFactory, Parser};
use std::path::Path;

pub mod config;
pub mod unit;

#[derive(Debug, clap::Subcommand)]
enum SubCmd {
    DefaultConfig,
    Run,
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: SubCmd,
    #[arg(short, long, default_value = "config.ron")]
    config_file_path: String,
}

fn run(args: &Args) -> anyhow::Result<()> {
    // NOTE: Typically one would load the config from the asset server. But doing a blocking
    // load here simplifies the code and won't cause significant issues.
    let config = std::fs::read_to_string(&args.config_file_path)
        .inspect_err(|_| {
            eprint!(
                "Could not find/read config file at `{}`",
                &args.config_file_path
            );

            if Path::new(&args.config_file_path).is_relative() {
                if let Some(pwd) = std::env::current_dir().ok() {
                    eprint!(" (relative to `{}`)", pwd.display());
                }
            }
            eprintln!("\nWill continue with default game settings.");
        })
        .unwrap_or_default();
    let config = ron::from_str::<config::Config>(&config).unwrap();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(config);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse().inspect_err(|_| {
        eprintln!("{}", Args::command().render_help());
    })?;

    match args.cmd {
        SubCmd::DefaultConfig => {
            println!(
                "{}",
                ron::ser::to_string_pretty(
                    &config::Config::default(),
                    ron::ser::PrettyConfig::default()
                )
                .expect("Contact maintainer or submit a pull. Somebody made a mistake.")
            );
        }
        SubCmd::Run => run(&args)?,
    }
    Ok(())
}
