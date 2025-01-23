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
    // Split the input string by whitespace and parse each token as a float
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
    // Split the input string by whitespace and parse each token as an integer
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

    println!("Please paste a block of whitespace-delimited {} (you can include newlines) and press Ctrl+D (or Ctrl+Z on Windows) when done:",
             if parse_as_integers { "integers" } else { "floats" });

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?; // Read multi-line user input

    if parse_as_integers {
        match read_integers_from_input(&input) {
            Ok(integers) => {
                // println!("Parsed integers: {:?}", integers);
                println!("Total integers: {}", integers.len());

                // Write the integers to the specified file
                let contents = integers
                    .iter()
                    .map(|i| i.to_string()) // Convert each integer to a string
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
                // println!("Parsed floats: {:?}", floats);
                println!("Total floats: {}", floats.len());

                // Write the floats to the specified file
                let contents = floats
                    .iter()
                    .map(|f| format!("{:.7E}", f)) // Format each float in scientific notation with 7 decimals
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
