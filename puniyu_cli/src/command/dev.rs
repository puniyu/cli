use std::fs;
use std::path::Path;

use inquire::Text;

use crate::template::Options;
use crate::TEMPLATE_DIR;

pub struct Command;

impl Command {
    pub fn run() {
        let project_name = Text::new("请输入您的项目名称?").prompt();
        
        match project_name {
            Ok(name) => {
                if let Err(e) = Self::create_project(&name) {
                    println!("创建项目失败: {}", e);
                } else {
                    println!("项目 {} 创建成功!", name);
                }
            }
            Err(_) => println!("输入错误"),
        }
    }

    fn create_project(name: &str) -> std::io::Result<()> {
        let project_path = Path::new(name);
        
        // 创建项目目录
        fs::create_dir_all(project_path.join("src"))?;
        
        // 渲染并写入 Cargo.toml
        let options = Options { name: name.to_string() };
        let template = options.build();
        let cargo_toml = template.render();
        fs::write(project_path.join("Cargo.toml"), cargo_toml)?;
        
        // 复制 src/lib.rs
        if let Some(lib_rs) = TEMPLATE_DIR.get_file("src/lib.rs") {
            fs::write(project_path.join("src/lib.rs"), lib_rs.contents())?;
        }
        
        Ok(())
    }
}
