use bfkit::optimizer;
use bfkit::{compiler, parser};
use clap::{crate_authors, crate_description, crate_name, App, Arg};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use bfkit::interp::{Interpreter, StopReason};

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .help("Run an interactive REPL"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .value_name("OUTPUT")
                .help("The C source file to be written"),
        )
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .value_name("FILE")
                .required(true)
                .help("Brainfuck source file to be executed"),
        )
        .get_matches();

    let source = fs::read_to_string(matches.value_of("file").unwrap()).unwrap();

    if matches.is_present("interactive") {
        repl(source);
    } else {
        let code = optimizer::optimize(parser::parse_str(source), 10);

        let compiled = compiler::compile(code);

        if let Some(output) = matches.value_of("output") {
            fs::write(output, compiled).unwrap();
        } else {
            println!("{}", compiled);
        }
    }
}

fn repl(source: String) {
    let code = parser::parse_str(source);
    let mut interp = Interpreter::new(code.clone());

    let stdin = stdin();

    println!("Welcome to bfkit! Type `help` for more information.");
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();
        let parts: Vec<&str> = buffer.split(" ").collect();

        match parts[0] {
            "help" | "h" => {
                println!("Commands:");
                println!("    help (h)");
                println!("    quit (q)");
                println!("    run (r)");
                println!("    break (b)");
                println!("    delete (d)");
                println!("    step (s)");
                println!("    print (p)");
                println!("    assign (a)");
                println!("    jump (j)");
                println!("    select");
            }
            "quit" | "q" => {
                println!("OK");
                exit(0);
            }
            "run" | "r" => {
                match interp.run() {
                    StopReason::Breakpoint(address) => println!("Hit breakpoint at {} ({:?})", address, code[address]),
                    StopReason::Done => println!("OK")
                }
            }
            "break" | "b" => {
                if parts.len() != 2 {
                    eprintln!("Invalid syntax!");
                } else {
                    match parts[1].parse::<usize>() {
                        Ok(address) => match interp.set_breakpoint(address) {
                            Ok(_) => println!("OK"),
                            Err(e) => eprintln!("{}", e)
                        },
                        Err(_) => eprintln!("Invalid address: {}", parts[1])
                    }
                }
            },
            "delete" | "d" => {
                if parts.len() != 2 {
                    eprintln!("Invalid syntax!");
                } else {
                    match parts[1].parse::<usize>() {
                        Ok(address) => {
                            interp.delete_breakpoint(address);
                            println!("OK");
                        },
                        Err(_) => eprintln!("Invalid address: {}", parts[1])
                    }
                }
            },
            "step" | "s" => {
                interp.step();
            },
            "print" | "p" => {
                if parts.len() != 2 {
                    eprintln!("Invalid syntax!");
                } else {
                    match parts[1].parse::<usize>() {
                        Ok(address) => match interp.get(address) {
                            Ok(value) => println!("{}", value),
                            Err(e) => eprintln!("{}", e)
                        },
                        Err(_) => eprintln!("Invalid address: {}", parts[1])
                    }
                }
            },
            "assign" | "a" => {
                if parts.len() != 3 {
                    eprintln!("Invalid syntax!");
                } else {
                    match parts[1].parse::<usize>() {
                        Ok(address) => match parts[2].parse::<u8>() {
                            Ok(value) => match interp.set(address, value) {
                                Ok(_) => println!("OK"),
                                Err(e) => eprintln!("{}", e)
                            },
                            Err(_) => eprintln!("Invalid byte: {}", parts[2])
                        },
                        Err(_) => eprintln!("Invalid address: {}", parts[1])
                    }
                }
            },
            "jump" | "j" => {
                if parts.len() != 2 {
                    eprintln!("Invalid syntax!");
                } else {
                    match parts[1].parse::<usize>() {
                        Ok(address) => match interp.jump(address) {
                            Ok(_) => println!("OK"),
                            Err(e) => eprintln!("{}", e)
                        },
                        Err(_) => eprintln!("Invalid address: {}", parts[1])
                    }
                }
            },
            "select" => {
                if parts.len() != 2 {
                    eprintln!("Invalid syntax!");
                } else {
                    match parts[1].parse::<usize>() {
                        Ok(address) => match interp.select(address) {
                            Ok(_) => println!("OK"),
                            Err(e) => eprintln!("{}", e)
                        },
                        Err(_) => eprintln!("Invalid address: {}", parts[1])
                    }
                }
            },
            _ => {
                eprintln!("Unrecognized command: {}", buffer);
            }
        }
    }
}
