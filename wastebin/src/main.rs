// Imports
use std::env;
use std::process::exit;

#[derive(Debug)]
struct Flags {
    force: bool,
    prompt: bool,
    weak_prompt: bool,
    interactive: bool,
    one_file_system: bool,
    no_preserve_root: bool,
    preserve_root: bool,
    recursive: bool,
    empty_dir: bool,
    verbose: bool,
    help: bool,
    version: bool,
}

fn error_out(num: u32, str_arg: String) {
    if num == 1 {
        let err_str = format!("Error: Unknown flag -{}. Use --help to display all flags and usage.", str_arg);
        println!("{}", err_str);
    }
      
    exit(-1);
}

// Start of Main
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut user_flags = Flags {
        force: false,
        prompt: false,
        weak_prompt: false,
        interactive: false,
        one_file_system: false,
        no_preserve_root: false,
        preserve_root: false,
        recursive: false,
        empty_dir: false,
        verbose: false,
        help: false,
        version: false,
    };

    // Parse Arguments
    for item in args {
        if item.starts_with("-") {
           if item.eq("--interactive") {
               user_flags.interactive = true;
           }
           else if item.eq("--one-file-system") {
               user_flags.one_file_system = true;
           }
           else if item.eq("--no-preserve-root") {
               user_flags.no_preserve_root = true;
           }
           else if item.eq("--preserve-root") {
              user_flags.preserve_root = true;
           }
           else if item.eq("--force") {
              user_flags.force = true;
           }
           else if item.eq("--dir") {
              user_flags.empty_dir = true;
           }
           else if item.eq("--verbose") {
               user_flags.verbose = true;
           }
           else if item.contains("--help") {
               user_flags.help = true;
           }
           else if item.contains("--version") {
               user_flags.version = true;
           }

           for ch in item.chars() {
               match ch {
                   'f' => user_flags.force = true,
                   'i' => user_flags.prompt = true,
                   'I' => user_flags.weak_prompt = true,
                   'r' => user_flags.recursive = true,
                   'R' => user_flags.recursive = true,
                   'd' => user_flags.empty_dir = true,
                   'v' => user_flags.verbose = true,
                   '-' => continue,
                    _  => error_out(1, ch.to_string()),
               };
           }
        }
    }

    println!("{:?}", user_flags);

}
