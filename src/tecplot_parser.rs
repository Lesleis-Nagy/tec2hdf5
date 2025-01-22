//
// file: tecplot_parser.rs
// author: L. Nagy
//
use std::fs;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub tecplot);

///
/// A Document object holds information parsed from a tecplot document.
///
pub struct Document {
    /// The tecplot file title.
    pub title: String,

    /// The tecplot variables.
    pub variables: Vec<String>,

    /// The first zone found in the tecplot file.
    pub first_zone: FirstZone,

    /// Subsequent zones found in the tecplot file (if any).
    pub zones: Option<Vec<Zone>>,
}

///
/// The first zone in a tecplot file is special since it is both necessary and contains information
/// about the mesh geometry.
///
pub struct FirstZone {
    /// The zone's title.
    pub title: String,

    /// The number of vertices parsed from the zone's header.
    pub no_of_vertices: i64,

    /// The number of elements parsed from the zone's header.
    pub no_of_elements: i64,

    /// The list of floating point values found in the zone -- this is just a large list of
    /// floating point values which must be processed further to represent vertices and field vector
    /// values.
    pub float_list: Vec<f64>,

    /// The list of integers -- this is a large list of integers which must be processed further to
    /// represent element and sub-mesh indices.
    pub integer_list: Vec<i64>,
}

///
/// A subsequent zone (if present) contains just field vector values.
///
pub struct Zone {
    /// The zone's title.
    pub title: String,

    /// The number of vertices parsed from the zone's header.
    pub no_of_vertices: i64,

    /// The number of elements parsed from the zone's header.
    pub no_of_elements: i64,

    // The list of floating point values found in the zone -- this is just a large list of floating
    // point values which must be processed further to represent field vector values.
    pub float_list: Vec<f64>,
}

#[test]
fn test_title() {
    let result =  tecplot::TitleParser::new().parse("TITLE = \"My test title\"")
        .expect("Parse failed");
    let expected : String = "\"My test title\"".to_string();
    assert_eq!(result, expected);
}

#[test]
fn test_variables() {
    let result =  tecplot::VariablesParser::new().parse("VARIABLES = \"x\", \"y\", \"z\"")
        .expect("Parse failed");
    let expected = vec!["\"x\"", "\"y\"", "\"z\""];
    assert_eq!(result.len(), expected.len());
    for i in 0 .. result.len() {
        assert_eq!(result[i], expected[i]);
    }
}

#[test]
fn test_document() {
    let file_path = "/Users/lnagy2/Projects/tec2hdf5/examples/myhyst.tec";
    let file_contents = fs::read_to_string(file_path).expect("Failed to read tecplot file");
    let result = tecplot::DocumentParser::new().parse(&file_contents).expect("Parse failed");
    let expected_title: String = "\"myhyst\"".to_string();
    let expected_variables = vec!["\"X\"", "\"Y\"", "\"Z\"", "\"Mx\"", "\"My\"", "\"Mz\"", "\"SD\""];
    assert_eq!(result.title, expected_title);
    assert_eq!(result.variables.len(), expected_variables.len());
    for i in 0 .. result.variables.len() {
        assert_eq!(result.variables[i], expected_variables[i]);
    }
    println!("the title of the document: {}", result.title);
    println!("the title of the first zone: {}", result.first_zone.title);
    println!("no of vertices in the first zone: {}", result.first_zone.no_of_vertices);
    println!("no of elements in the first zone: {}", result.first_zone.no_of_elements);
    println!("number of floats in the first zone: {}", result.first_zone.float_list.len());
    println!("number of integers in the first zone: {}", result.first_zone.integer_list.len());

    match result.zones {
        Some(z) => {
            println!("There is {} additional zone", z.len());
            for (index, zone) in z.iter().enumerate() {
                println!("Zone {}: {}", index, zone.title);
                println!("no. of vertices is {}", zone.no_of_vertices);
                println!("no. of elements is {}", zone.no_of_elements);
                println!("no. of floats is {}", zone.float_list.len());
            }

        },
        None => {
            println!("There are no additional zones");
        },
    }
}
