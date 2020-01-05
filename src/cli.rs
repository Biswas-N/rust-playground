// CLI - Command line interface using StructOps of Rust(Which is based on clap)
// CLAP - Command Line Argument Parser

mod print;
mod variables;
mod dtypes;
mod strings;
mod tuples;
mod arrays;
mod vector;
mod conditionals;
mod loops;
mod functions;
mod pointers;
mod structs;
mod enums;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Rust Basics", about = "Cli app to understand rust basics")]
struct Cli {
    /// Commands can be [Arrays, Conditions, Dtypes, Enums, Functions, Loops, Pointers, Print, Strings, Structs, Tuples, Variables, Vectors]
    #[structopt(short = "c", long = "command")]
    command: String
}

pub fn run() {
    let args = Cli::from_args();
    let command = String::from(args.command).to_lowercase();

    match command.as_ref() {
        "arrays" => arrays::run(),
        "conditions" => conditionals::run(),
        "dtypes" => dtypes::run(),
        "enums" => enums::run(),
        "functions" => functions::run(),
        "loops" => loops::run(),
        "pointers" => pointers::run(),
        "print" => print::run(),
        "strings" => strings::run(),
        "structs" => structs::run(),
        "tuples" => tuples::run(),
        "variables" => variables::run(),
        "vectors" => vector::run(),
        _ => eprintln!("Not a vaild command\n=> Try \"playground --help\"")
    }
}