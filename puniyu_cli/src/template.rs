use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
pub struct Options {
	pub name: String,
}

impl Options {
	pub fn build(&self) -> Template {
		Template::new(Options {
			name: self.name.clone(),
		})
	}
}
pub struct Template {
    inner: Options
}

impl Template {
	pub fn new(options: Options) -> Self {
		Template { inner: options }
	}

	pub fn render(&self) -> String {
		let mut handlebars = Handlebars::new();
		let template_str = crate::TEMPLATE_DIR
			.get_file("Cargo.toml.template")
			.unwrap()
			.contents_utf8()
			.unwrap();
		handlebars.register_template_string("Cargo.toml", template_str).unwrap();
		handlebars.render("Cargo.toml", &self.inner).unwrap()
	}
}
