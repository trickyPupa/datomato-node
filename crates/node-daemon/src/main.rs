use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 {
        if args[0] == "--help" {
            println!(
                "datomato-node — stage 0 skeleton\nUsage: datomato-node --help | --version\nNo storage operations are implemented."
            );
            return ExitCode::SUCCESS;
        }
        if args[0] == "--version" {
            println!(
                "datomato-node {} (protocol {})",
                env!("CARGO_PKG_VERSION"),
                datomato_node_core::protocol_version()
            );
            return ExitCode::SUCCESS;
        }
    }
    eprintln!("Usage: datomato-node --help | --version");
    ExitCode::from(2)
}
