use std::{collections::HashMap, os::unix::process::CommandExt, process::Command};
use nix::{
    sys::{ptrace, wait::waitpid},
    unistd::Pid,
};
use owo_colors::OwoColorize;

enum Verbosity {
    None,
    Normal,
    Paused
}

macro_rules! verbosity_type {
    ($arg:expr) => {
        match $arg.as_str() {
            "-v" => Verbosity::Normal,
            "-V" => Verbosity::Paused,
            _ => Verbosity::None
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // --- Parseo de argumentos ---
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso correcto: rastreador [-v | -V ] Prog [args...]");
        std::process::exit(1);
    }
    let _verbose: Verbosity = verbosity_type!(args[1]);
    if args[1] == "-v" || args[1] == "-V" {
        args.remove(1);
    }
    
    Ok(())
}
