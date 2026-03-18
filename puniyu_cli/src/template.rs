use derive_builder::Builder;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize, Builder)]
#[builder(setter(into))]
pub struct Template {
    name: String,
}

impl Template {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    #[allow(unused)]
    pub fn builder() -> TemplateBuilder {
        TemplateBuilder::default()
    }

    pub fn render(&self) -> String {
        let mut handlebars = Handlebars::new();
        let template_str = crate::TEMPLATE_DIR
            .get_file("rust/Cargo.toml.template")
            .unwrap()
            .contents_utf8()
            .unwrap();
        handlebars
            .register_template_string("Cargo.toml", template_str)
            .unwrap();
        handlebars.render("Cargo.toml", &self).unwrap()
    }
}
