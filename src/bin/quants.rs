use tec2hdf5::tecplot_parser::{
    create_mesh_from_tecplot,
};

use clap::{Arg, Command};

use std::error::Error;
use csv::Writer;

struct CliArgs {
    tecplot_file: String,
    output_file: String,
}

fn parse_args() -> CliArgs {
    let matches = Command::new("quants")
        .version("1.0")
        .author("Les Nagy <lesleisnagy@gmail.com>")
        .about("A tool to read tecplot files and produce micromagnetic field quantities")
        .arg(
            Arg::new("tecplot_file")
                .help("The input tecplot file")
                .value_name("TECPLOT")
                .required(true),
        )
        .arg(
            Arg::new("output_file")
                .help("The output csv file computes micromagnetic quantitites per field")
                .value_name("OUTPUT")
                .required(true),
        )
        .get_matches();

    CliArgs {
        tecplot_file: matches.get_one::<String>("tecplot_file").unwrap().to_string(),
        output_file: matches.get_one::<String>("output_file").unwrap().to_string(),
    }

}

#[derive(serde::Serialize)]
struct CSVOutputRow<'a> {
    index: &'a usize,
    mom_x: &'a f64,
    mom_y: &'a f64,
    mom_z: &'a f64,
}

fn main() {

    // Parse command line arguments
    let args = parse_args();

    println!("tecplot file: {}", args.tecplot_file);
    println!("output file: {}", args.output_file);

    // We read the mesh along with the zones.
    let mut mesh = create_mesh_from_tecplot(&args.tecplot_file);
    mesh.compute_volume();
    mesh.compute_net_moments();

    let mesh = mesh;
    println!("Mesh data");
    println!("No. of vertices: {}", mesh.vertices.len());
    println!("No. of elements: {}", mesh.elements.len());
    println!("Volume:          {}", mesh.volume.unwrap());
    println!("Computing quantities");

    let mut csv_writer = Writer::from_path(args.output_file).unwrap();

    for (index, moment) in mesh.net_moments.unwrap().iter().enumerate() {
        let out_index = index + 1;
        csv_writer.serialize(
            CSVOutputRow{
                index: &out_index,
                mom_x: &moment[0],
                mom_y: &moment[1],
                mom_z: &moment[2],
            }
        ).unwrap()
    }

    println!("Done");

}
