fn main() {
    let mut args: Vec<_> = std::env::args_os().collect();
    if args.get(1).is_some_and(|arg| arg == "puniyu") {
        args.remove(1);
    }
    puniyu_cli::App::run(args);
}
