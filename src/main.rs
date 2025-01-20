use clap::Parser;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub tecplot);

// A simple input / output.
#[derive(Parser)]
#[command(name="tec2hdf5")]
#[command(about="tec2hdf5 is a tool to convert tecplot files to hdf5 files.")]
struct Cli {
    // Activate generation of xdmf files.
    #[arg(short, long)]
    with_xdmf: bool,

    // Input file (required)
    input: String,

    // Output stub (required)
    stub: String,
}

fn main() {

    let cli = Cli::parse();

    println!("Input file: {}", cli.input);
    println!("Output stub: {}", cli.stub);
    println!(
        "XDMF: {}",
        if cli.with_xdmf { "yes" } else { "no" }
    )

}
