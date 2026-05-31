use std::env;
use std::path::PathBuf;

fn main() {
    let root = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let findings = vtrace::run_checks(&root);
    let display_root = root.canonicalize().unwrap_or(root);

    for finding in &findings {
        println!("{}", finding.render(&display_root));
    }

    if findings.is_empty() {
        println!("VTRACE validation passed");
    } else {
        println!("VTRACE validation failed: {} finding(s)", findings.len());
        std::process::exit(1);
    }
}
