// use cargs::are_cargo_args_valid;
use std::{
    env::{self},
    io::{stdin, BufReader, Stdin},
};
// use std::fs::File;
mod args;
mod cargo;

fn main() {
    const USAGE: &str = "[-h] [-c|-v] [-p INDENT]\n \
   -h       Help: displays this help menu.\n \
   -v       Validate: the program reads from standard input and checks whether\n \
            it is syntactically correct JSON.  If there is any error, then a message\n \
            describing the error is printed to standard error before termination.\n \
            No other output is produced.\n \
   -c       Canonicalize: once the input has been read and validated, it is\n \
            re-emitted to standard output in 'canonical form'.  Unless -p has been\n \
            specified, the canonicalized output contains no whitespace (except within\n \
            strings that contain whitespace characters).\n \
   -p       Pretty-print:  This option is only permissible if -c has also been specified.\n \
            In that case, newlines and spaces are used to format the canonical output\n \
            in a more human-friendly way.  For the precise requirements on where this\n \
            whitespace must appear, see the assignment handout.\n \
            The INDENT is an optional nonnegative integer argument that specifies the\n \
            number of additional spaces to be output at the beginning of a line for each\n \
            for each increase in indentation level.  If no value is specified, then a\n \
            default value of 4 is used.\n";
    let mut indent_level: i32 = 4;
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();
    let reader: BufReader<Stdin> = BufReader::new(stdin());
    dbg!(argv);
    let is_valid: bool = args::are_cargo_args_valid(argc, argv.clone());
    if !is_valid {
        println!("{}", USAGE);
    }
}
