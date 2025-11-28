mod command;
pub mod template;



use clap::{Parser, Subcommand};
use include_dir::{include_dir, Dir};

pub(crate) static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/template");

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct App {
	#[command(subcommand)]
	dev: Commands
}

#[derive(Subcommand)]
enum Commands  {
	Create,
	Dev
}

impl App {
	pub fn run() {
		let app = Self::parse();
		match app.dev {
			Commands::Create => command::create::Command::run(),
			Commands::Dev => command::dev::Command::run(),
		}
	}
}
