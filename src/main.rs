mod lib;

use lib::{
    run,
    FileOptions::{Bytes, Chars, None},
};

fn main() {
    let files = std::env::args().skip(1).collect::<Vec<_>>();

    // parse the user input from the cli manually
    // Usage: wc [Flags] <files...>
    // Flags:
    // -b | --bytes // Listing bytes count of the file or files
    // -c | --chars // Listing character count of the file or files
    // -h | --help  // Help

    if files.len() == 0 {
        println!("You are now using the your Standard Input.");
        // an empty vector is passed to satisfy the function run.
        lib::run(&vec![], None);
    } else if files.len() == 1
        && (files[0].clone().contains("-") || files[0].clone().contains("--"))
    {
        println!("When you use a flag, give a file also. Usage: wc -b temp.txt");
        std::process::exit(1);
    }

    match files[0].clone().as_str() {
        "-b" | "--bytes" => run(&files, Bytes),
        "-c" | "--chars" => run(&files, Chars),
        "-h" | "--help" => println!("Usage: wc <flags> <file or files>"),
        _ => lib::run(&files, None),
    }
}
