use std::process::Command;

#[test]
fn only_documented_information_flags_succeed() {
    let binary = env!("CARGO_BIN_EXE_datomato-node");
    for (flag, expected) in [
        ("--help", "Usage:"),
        ("--version", "datomato-node 0.1.0 (protocol 1)"),
    ] {
        let output = Command::new(binary).arg(flag).output().unwrap();
        assert!(output.status.success());
        assert!(String::from_utf8(output.stdout).unwrap().contains(expected));
        assert!(output.stderr.is_empty());
    }
    for args in [
        vec![],
        vec!["upload"],
        vec!["--help", "upload"],
        vec!["--unknown"],
    ] {
        let output = Command::new(binary).args(args).output().unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
    }
}
