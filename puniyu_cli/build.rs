use fs_extra::dir::{CopyOptions, copy};
use std::{env, path::Path, process::Command};

fn main() {
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Failed to get workspace dir");
    Command::new("git")
        .args(["submodule", "update", "--init", "--remote", "--recursive"])
        .status()
        .unwrap();
    let template_dir = workspace_dir.join("template");
    if template_dir.exists() {
        let out_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let out_dir = out_dir.join("template").join("rust");

        std::fs::create_dir_all(&out_dir).unwrap();

        let rust_template_dir = template_dir.join("rust");
        rust_template(&rust_template_dir, &out_dir);
    }
    println!("cargo:rerun-if-changed={}", template_dir.to_string_lossy());
}

fn rust_template<P: AsRef<Path>>(template_dir: P, out_dir: P) {
    use toml_edit::DocumentMut;
    let options = CopyOptions::new().overwrite(true);
    let source_dir = template_dir.as_ref().join("src");
    let cargo_path = template_dir.as_ref().join("Cargo.toml");
    let readme_path = template_dir.as_ref().join("README.md");

    copy(&source_dir, &out_dir, &options).unwrap();

    std::fs::copy(&cargo_path, out_dir.as_ref().join("Cargo.toml.template")).unwrap();

    let cargo_file = std::fs::read_to_string(out_dir.as_ref().join("Cargo.toml.template")).unwrap();
    let mut doc = cargo_file.parse::<DocumentMut>().unwrap();
    doc["package"]["name"] = toml_edit::value("{{name}}");
    doc["package"]
        .as_table_mut()
        .map(|t| t.remove("description"));

    std::fs::write(
        out_dir.as_ref().join("Cargo.toml.template"),
        doc.to_string(),
    )
    .unwrap();

    std::fs::copy(&readme_path, out_dir.as_ref().join("README.md")).unwrap();
}
