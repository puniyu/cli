mod commands;
mod template;

use clap::{Parser, Subcommand};
use include_dir::{Dir, include_dir};

pub(crate) static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/template");

pub(crate) const HELP_TEMPLATE: &str = "{about-with-newline}\n使用方法:\n  {usage}\n\n命令:\n{subcommands}\n选项:\n  -h, --help  显示帮助信息\n";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[command(
    disable_help_subcommand = true,
    help_template = "{about-with-newline}\n使用方法:\n  {usage}\n\n命令:\n{subcommands}\n选项:\n  -h, --help     显示帮助信息\n  -V, --version  显示版本信息\n"
)]
pub struct App {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 开发相关命令
    Dev(commands::dev::DevCommand),
}

impl App {
    pub fn run<I, T>(args: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let app = Self::parse_from(args);
        match app.command {
            Commands::Dev(dev_cmd) => dev_cmd.run()
        }
    }
}
