use bfkit::optimizer;
use bfkit::{compiler, parser};
use clap::{crate_authors, crate_description, crate_name, App, Arg};
use std::fs;
use std::io::{stdin, stdout, Write};
use std::process::exit;

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
        repl();
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

fn repl() {
    let stdin = stdin();

    println!("Welcome to bfkit! Type `help` for more information.");
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();
        let parts = buffer.split(" ");

        match buffer {
            "help" | "h" => {
                println!("Commands:");
                println!("    help (h)");
                println!("    quit (q)");
                println!("    run (r)");
                println!("    break (b)");
                println!("    step (s)");
                println!("    print (p)");
                println!("    set (s)");
            }
            "quit" | "q" => {
                exit(0);
            }
            "run" | "r" => {
                // TODO
            }
            "break" | "b" => {
                // TODO
            }
            "step" | "s" => {
                // TODO
            }
            "print" | "p" => {
                // TODO
            }
            "set" | "s" => {
                // TODO
            }
            _ => {
                eprintln!("Unrecognized command: {}", buffer);
            }
        }
    }
}
