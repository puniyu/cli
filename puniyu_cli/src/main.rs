fn main() {
    let mut args: Vec<_> = std::env::args_os().collect();
    if args.len() > 1 && args[1] == "puniyu" {
        args.remove(1);
    }
    puniyu_cli::App::run(args);
}
