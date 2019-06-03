use bfkit::{compiler, parser, optimizer, repl, ir};
use clap::{crate_authors, crate_description, crate_name, App, Arg};
use std::fs;

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
            Arg::with_name("output-type")
                .short("t")
                .help("The output format [c, ir]")
                .takes_value(true)
                .possible_values(&["c", "ir"])
                .default_value("c")
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .takes_value(true)
                .value_name("OUTPUT")
                .help("The output file to be written"),
        )
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .value_name("FILE")
                .required(true)
                .help("Brainfuck source file to be compiled or executed"),
        )
        .get_matches();

    let source = fs::read_to_string(matches.value_of("file").unwrap()).unwrap();

    if matches.is_present("interactive") {
        repl::repl(source);
    } else {
        let code = optimizer::optimize(parser::parse_str(source), 10);

        let result = match matches.value_of("output-type").unwrap() {
            "c" => compiler::compile(code),
            "ir" => ir::ir_to_string(code),
            _ => unreachable!()
        };

        if let Some(output) = matches.value_of("output") {
            fs::write(output, result).unwrap();
        } else {
            println!("{}", result);
        }
    }
}