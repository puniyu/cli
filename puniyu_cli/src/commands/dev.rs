mod create;
use crate::HELP_TEMPLATE;
use clap::{Args, Subcommand};

#[derive(Args)]
#[command(
    about = "开发相关命令",
    help_template = HELP_TEMPLATE
)]
pub struct DevCommand {
    #[command(subcommand)]
    command: Subcommands,
}

impl DevCommand {
    pub fn run(&self) {
        match &self.command {
            Subcommands::Create => create::Command::run(),
        }
    }
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// 创建一个新的插件项目
    Create,
}
