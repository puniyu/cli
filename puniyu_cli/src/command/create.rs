use dialoguer::{theme::ColorfulTheme, Select};
pub struct Command;

impl Command {
	pub fn run() {
		let bot_type = Select::with_theme(&ColorfulTheme::default())
			.with_prompt("请选择创建Bot的类型")
			.item("Rust")
			.item("Node")
			.default(0)
			.interact()
			.unwrap();
	}
}
