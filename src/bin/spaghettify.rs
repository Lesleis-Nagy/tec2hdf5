///
/// Spaghettify utility - this utility allows the user to paste a block of floating point numbers
/// in to the terminal, these will then be split into a single column list of the same
/// numbers, for example:
/// 1.0 2.0 3.0 4.0         1.0
/// 5.0 6.0           ==>   2.0
/// 7.0 8.0 9.0             3.0
///                         4.0
///                         5.0
///                         6.0
///                         7.0
///                         8.0
///                         9.0
///

use std::fs;
use std::env;
use std::io::{self, Read};

///
/// Function to read the floating point numbers from an input string and produce a single vec
/// of the same values (thus spaghettifying the input).
/// @param input the input string of floats
/// @return a Vec<f64> object.
///

fn read_floats_from_input(input: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    // Split on whitespace treat each token as float
    let floats = input
        .split_whitespace() // Split on spaces, tabs, and newlines
        .map(|s| s.parse::<f64>()) // Attempt to parse each token as a float
        .collect::<Result<Vec<_>, _>>()?; // Collect into a Vec<f64> or return an error

    Ok(floats)
}

///
/// Function to read the integers numbers from an input string and produce a single vec
/// of the same values (thus spaghettifying the input).
/// @param input the input string of floats
/// @return a Vec<f64> object.
///

fn read_integers_from_input(input: &str) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    // Split on whitespace treat each token as int
    let integers = input
        .split_whitespace() // Split on spaces, tabs, and newlines
        .map(|s| s.parse::<i64>()) // Attempt to parse each token as an integer
        .collect::<Result<Vec<_>, _>>()?; // Collect into a Vec<i64> or return an error

    Ok(integers)
}

fn main() -> io::Result<()> {
    // Get the output file path and optional flag from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_file> [--integers]", args[0]);
        return Ok(());
    }
    let output_file = &args[1];
    let parse_as_integers = args.get(2).map_or(false, |flag| flag == "--integers");

    println!(
        "Paste whitespace-delimited {} (including newlines) & press Ctrl+D (Ctrl+Z on Windows)",
             if parse_as_integers { "integers" } else { "floats" }
    );

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    if parse_as_integers {
        match read_integers_from_input(&input) {
            Ok(integers) => {
                println!("Total integers: {}", integers.len());

                // Write integers to file
                let contents = integers
                    .iter()
                    .map(|i| i.to_string()) // Convert to int
                    .collect::<Vec<_>>()
                    .join("\n"); // Join with newlines
                fs::write(output_file, contents)?;

                println!("Integers successfully written to {}", output_file);
            }
            Err(e) => eprintln!("Error reading integers: {}", e),
        }
    } else {
        match read_floats_from_input(&input) {
            Ok(floats) => {
                println!("Total floats: {}", floats.len());

                // Write floats to file
                let contents = floats
                    .iter()
                    .map(|f| format!("{:.7E}", f)) // sci. notation formatting (7 d.p.)
                    .collect::<Vec<_>>()
                    .join("\n"); // Join with newlines
                fs::write(output_file, contents)?;

                println!("Floats successfully written to {}", output_file);
            }
            Err(e) => eprintln!("Error reading floats: {}", e),
        }
    }

    Ok(())
}
