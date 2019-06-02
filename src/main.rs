use clap::{crate_authors, crate_description, crate_name, App, Arg};
use bfkit::optimizer;
use bfkit::{compiler, parser};
use std::fs;

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
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
    let code = optimizer::optimize(parser::parse_str(source), 10);

    let compiled = compiler::compile(code);

    if let Some(output) = matches.value_of("output") {
        fs::write(output, compiled).unwrap();
    } else {
        println!("{}", compiled);
    }
}
