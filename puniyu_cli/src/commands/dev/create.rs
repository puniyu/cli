use crate::TEMPLATE_DIR;
use crate::template::Options;
use inquire::Text;
use inquire::validator::Validation;
use std::fs;
use std::path::Path;

pub struct Command;

impl Command {
    pub fn run() {
        let project_name = Text::new("请输入您的项目名称?")
            .with_validator(|input: &str| {
                if input.is_empty() {
                    Ok(Validation::Invalid("项目名称不能为空".into()))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt();
        match project_name {
            Ok(name) => match Self::create_project(&name) {
                Ok(()) => println!("项目 {} 创建成功!", name),
                Err(e) => println!("创建项目失败: {}", e),
            },
            Err(e) => println!("输入错误: {}", e),
        }
    }

    fn create_project(name: &str) -> std::io::Result<()> {
        let project_path = Path::new(name);
        fs::create_dir_all(project_path.join("src"))?;

        let options = Options {
            name: name.to_string(),
        };
        let template = options.build();
        let cargo_toml = template.render();
        fs::write(project_path.join("Cargo.toml"), cargo_toml)?;
        let source_dir = project_path.join("src");

        fs::write(
            source_dir.join("lib.rs"),
            TEMPLATE_DIR
                .get_file("rust/src/lib.rs")
                .unwrap()
                .contents_utf8()
                .unwrap(),
        )?;
        fs::write(
            source_dir.join("command.rs"),
            TEMPLATE_DIR
                .get_file("rust/src/command.rs")
                .unwrap()
                .contents_utf8()
                .unwrap(),
        )?;

        Ok(())
    }
}
