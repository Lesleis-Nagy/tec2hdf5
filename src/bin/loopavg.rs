use std::fs;
use std::path::Path;
use regex::Regex;
use clap::{Arg, Command};

struct CliArgs {
    base_directory: String,
    loop_file_match: String,
    output_file: String,
}

fn parse_args() -> CliArgs {
    let matches = Command::new("loopavg")
        .version("1.0")
        .author("Les Nagy <lesleisnagy@gmail.com>")
        .about("A simple tool to average over a set of hysteresis loop files")
        .arg(
            Arg::new("base_directory")
                .help("The base directory containing the loop files")
                .value_name("BASE_DIR")
                .required(true),
        )
        .arg(
            Arg::new("loop_file_match")
                .help("A regular expression to match the loop files")
                .value_name("LOOP_FILE_MATCH")
                .required(true),
        )
        .arg(
            Arg::new("output_file")
                .help("The output file")
                .value_name("OUTPUT_FILE")
                .required(true),
        )
        .get_matches();

    CliArgs {
        base_directory: matches.get_one::<String>("base_directory").unwrap().to_string(),
        loop_file_match: matches.get_one::<String>("loop_file_match").unwrap().to_string(),
        output_file: matches.get_one::<String>("output_file").unwrap().to_string(),
    }

}

fn main() {
    let args = parse_args();
    let mut loop_files = Vec::new();
    match fs::read_dir(args.base_directory) {
        Ok(entries) => {
            for entry in entries {
                let path = entry.unwrap().path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let re = Regex::new(&args.loop_file_match).unwrap();
                    if re.is_match(file_name) {
                        loop_files.push(path);
                    }
                }
            }
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }

    loop_files.sort();

    println!("Processing {} loop files", loop_files.len());
    for loop_file in loop_files {
        println!("\t{}", loop_file.to_str().unwrap());
    }
}
