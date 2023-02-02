use clap::Parser;
use nametag::*;
use std::path::PathBuf;
use std::process;

/// Generate random usernames
#[derive(Parser, Debug)]
#[clap(version, name = "nametag")]
struct Args {
    /// Prints verbose output, including parameters as received
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    /// Provide a text file with a list of words to randomly generate username from
    #[clap(short = 'l', long = "list")]
    list_file_path: Option<PathBuf>,

    /// Set how many random usernames to output
    #[clap(short = 'n', long = "number", default_value = "10")]
    number_to_print: usize,

    /// Set maximum username length. Must be greater than 5.
    #[clap(short = 'm', long = "maximum", default_value = "50")]
    maximum_length: u8,

    /// Uses Title Case for words in generated usernames
    #[clap(short = 't', long = "title-case")]
    title_case: bool,
}

fn main() {
    let opt = Args::parse();
    if opt.verbose {
        eprintln!("Received options: {:?}", opt);
        match &opt.list_file_path {
            Some(file_path) => eprintln!("Will use word list at {:?}", file_path),
            None => eprintln!("Will use default word list"),
        }
    }
    if opt.maximum_length < 5 {
        eprintln!("Error. Maximum username length must be greater than 5.");
        process::exit(1);
    }
    let usernames = get_usernames(
        opt.list_file_path,
        opt.number_to_print,
        opt.maximum_length,
        opt.title_case,
    );
    for username in usernames {
        println!("{}", username);
    }
}
