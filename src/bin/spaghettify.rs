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

use std::env;
use std::fs;
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

fn main() -> io::Result<()> {
    // Get the output file path from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_file>", args[0]);
        return Ok(());
    }
    let output_file = &args[1];

    let message = "Please paste a block of whitespace-delimited floats (you can include newlines) \
                   and press Ctrl+D (or Ctrl+Z on Windows) when done:".to_string();

    println!("{}", message);

    let mut input = String::new();

    // Read multi-line user input.
    io::stdin().read_to_string(&mut input)?;

    match read_floats_from_input(&input) {
        Ok(floats) => {
            // Write the floats to the specified file
            let contents = floats
                .iter()
                .map(|f| format!("{:.7E}", f)) // Format floats in sci. notation with 7 d.p.
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(output_file, contents)?;

            println!("{} floats successfully written to {}", floats.len(), output_file);
        }
        Err(e) => eprintln!("Error reading floats: {}", e),
    }

    Ok(())
}
