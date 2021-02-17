use std::env;
use std::process;

mod md2;
mod utility;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        utility::usage_no_args(&args[0]);
        process::exit(1);
    }

    let switch = args[1].clone();

    if switch != "f" && switch != "s" {
        utility::usage_no_args(&args[0]);
        process::exit(1);
    }

    if args.len() == 2 {
        utility::usage_with_switch(&args[0], &args[1]);
        process::exit(1);
    }

    if args.len() == 3 && switch == "f" {
        utility::hash_file(args[2]);
        process::exit(0);
    }

    if args.len() >= 3 && switch == "s" {
        utility::hash_string(&args[2..].iter().cloned().collect());
        process::exit(0);
    }
}
