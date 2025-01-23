mod tecplot_parser;

use clap::{Arg, Command};

struct CliArgs {
    tecplot_file: String,
    output_basename: String,
    with_xdmf: bool
}

fn parse_args() -> CliArgs {

    let matches = Command::new("tec2hdf5")
        .version("1.0")
        .author("Lesleis Nagy <lesnagy@liverpool.ac.uk>")
        .about("A simple tool to read tecplot files and produce MERRILL compatible HDF5 files")
        .arg(
            Arg::new("tecplot_file")
                .help("The input tecplot file")
                .value_name("TECPLOT")
                .required(true),
        )
        .arg(
            Arg::new("output_basename")
                .help("The base name for the output, the file '<BASENAME>.h5' is produced.")
                .value_name("BASENAME")
                .required(true),
        )
        .arg(
            Arg::new("with_xdmf")
                .help("Flag indicates if accompanying '<BASENAME>.xdmf' ought to be produced.")
                .long("with-xdmf")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    CliArgs {
        tecplot_file: matches.get_one::<String>("tecplot_file").unwrap().to_string(),
        output_basename: matches.get_one::<String>("output_basename").unwrap().to_string(),
        with_xdmf: matches.get_one::<bool>("with_xdmf").copied().unwrap_or(false),
    }

}

fn main() {

    // Parse command line arguments.
    let args = parse_args();

    println!("tecplot file: {}", args.tecplot_file);
    println!("output basename: {}", args.output_basename);
    if args.with_xdmf {
        println!("output xdmf: true");
    } else {
        println!("output xdmf: false");
    }

}
