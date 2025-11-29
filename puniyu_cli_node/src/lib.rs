use napi_derive::napi;

#[napi]
pub fn run(args: Vec<String>) {
    puniyu_cli::App::run(args)
}