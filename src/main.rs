use std::{collections::HashMap, os::unix::process::CommandExt, process::Command};
use nix::{
    sys::{ptrace, signal::Signal, wait::{waitpid, WaitStatus}},
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
    
    // --- Lista de Syscalls ---
    let syscalls_json: serde_json::Value = serde_json::from_str(include_str!("syscalls.json"))?;
        let syscall_t: HashMap<u64, String> = syscalls_json["aaData"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| (item[0].as_u64().unwrap(), item[1].as_str().unwrap().to_owned()))
            .collect();
    
    // --- Preparacion de la Llamada al Sistema ---
    let prog: &String = &args[1];
    let prog_args: Vec<String> = args[1..].to_vec();

    let mut command: Command = Command::new(prog);

    prog_args[1..].iter().for_each(|arg| {
        command.arg(arg);
    });

    unsafe {
        command.pre_exec(|| {
            use nix::sys::ptrace::traceme;
            traceme().map_err(|e: nix::errno::Errno| e.into())
        });
    }
    let child: std::process::Child = command.spawn()?;
    let child_pid: Pid = Pid::from_raw(child.id() as _);

    _ = waitpid(child_pid, None)?;

    // --- Rastreo ---
    let mut syscall_count: HashMap<String, usize> = HashMap::new();
    let mut is_sys_exit = false;

    loop {
        ptrace::syscall(child_pid, None)?;
        match waitpid(child_pid, None)? {
            WaitStatus::Exited(_, _) => break,
            WaitStatus::Stopped(pid, Signal::SIGTRAP) => {
                if is_sys_exit {
                    let regs: nix::libc::user_regs_struct = ptrace::getregs(pid)?;
                    let unknown_value = &format!("unknown({})", regs.orig_rax);
                    let name: &String = syscall_t
                        .get(&regs.orig_rax)
                        .unwrap_or(unknown_value);

                    // acumular conteo
                    *syscall_count.entry(name.clone()).or_insert(0) += 1;

                    // si verbose, imprimir
                    if !matches!(_verbose, Verbosity::None) {
                        println!(
                            "{}({:x}, {:x}, {:x}, ...) = {:x}",
                            name.green(),
                            regs.rdi.blue(),
                            regs.rsi.blue(),
                            regs.rdx.blue(),
                            regs.rax.yellow(),
                        );
                        if matches!(_verbose, Verbosity::Paused) {
                            use std::io::{stdin, Read};
                            let _ = stdin().bytes().next();
                        }
                    }
                }
                is_sys_exit = !is_sys_exit;
            }
            _ => {}
        }
    }


    // --- Tabla final ---
    println!("\nResumen de Syscalls:");
    for (name, count) in &syscall_count {
        println!("{:<20} {}", name, count);
    }

    Ok(())
}
