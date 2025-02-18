//
// file: tecplot_parser.rs
// author: L. Nagy
//

use std::fs;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub tecplot);

use crate::mesh::{Field, Mesh};

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
    pub no_of_vertices: usize,

    /// The number of elements parsed from the zone's header.
    pub no_of_elements: usize,

    /// The list of floating point values found in the zone -- this is just a large list of
    /// floating point values which must be processed further to represent vertices and field vector
    /// values.
    pub float_list: Vec<f64>,

    /// The list of integers -- this is a large list of integers which must be processed further to
    /// represent element and sub-mesh indices.
    pub integer_list: Vec<usize>,
}

///
/// A subsequent zone (if present) contains just field vector values.
///
pub struct Zone {
    /// The zone's title.
    pub title: String,

    /// The number of vertices parsed from the zone's header.
    pub no_of_vertices: usize,

    /// The number of elements parsed from the zone's header.
    pub no_of_elements: usize,

    // The list of floating point values found in the zone -- this is just a large list of floating
    // point values which must be processed further to represent field vector values.
    pub float_list: Vec<f64>,
}

///
/// Create a Mesh object from a tecplot file.
///
fn create_mesh_from_tecplot(file: &str) -> Mesh {
    let file_contents = fs::read_to_string(file).expect("Failed to read tecplot file");

    let mut document = tecplot::DocumentParser::new()
        .parse(&file_contents)
        .expect("Failed to parse tecplot file");

    let nvert = document.first_zone.no_of_vertices;
    let nelem = document.first_zone.no_of_elements;

    // Extract vertices.
    let mut xs = document
        .first_zone
        .float_list
        .drain(..nvert)
        .collect::<Vec<f64>>();
    let mut ys = document
        .first_zone
        .float_list
        .drain(..nvert)
        .collect::<Vec<f64>>();
    let mut zs = document
        .first_zone
        .float_list
        .drain(..nvert)
        .collect::<Vec<f64>>();

    let mut vertices: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]; nvert];
    for i in 0..nvert {
        vertices[i][0] = xs.remove(0);
        vertices[i][1] = ys.remove(0);
        vertices[i][2] = zs.remove(0);
    }

    // Extract the submesh indices
    let submesh_indices: Vec<usize> = document.first_zone.integer_list
        .drain(..nelem)
        .collect::<Vec<usize>>();

    // Extract the tetrahedron indices.
    let mut elements: Vec<[usize; 4]> = vec![[0, 0, 0, 0]; nelem];
    for i in 0..nelem {
        elements[i][0] = document.first_zone.integer_list.remove(0) - 1;
        elements[i][1] = document.first_zone.integer_list.remove(0) - 1;
        elements[i][2] = document.first_zone.integer_list.remove(0) - 1;
        elements[i][3] = document.first_zone.integer_list.remove(0) - 1;
    }

    // All integers in the first zone should be drained.
    assert_eq!(document.first_zone.integer_list.len(), 0);

    let mut fields: Vec<Field> = Vec::new();

    // Add the first field.
    let mut vx = document
        .first_zone
        .float_list
        .drain(..nvert)
        .collect::<Vec<f64>>();
    let mut vy = document
        .first_zone
        .float_list
        .drain(..nvert)
        .collect::<Vec<f64>>();
    let mut vz = document
        .first_zone
        .float_list
        .drain(..nvert)
        .collect::<Vec<f64>>();

    // All floats in the first zone should be drained.
    assert_eq!(document.first_zone.float_list.len(), 0);

    // Extract the field components in the first zone.
    let mut vs: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]; nvert];
    for i in 0..nvert {
        vs[i][0] = vx.remove(0);
        vs[i][1] = vy.remove(0);
        vs[i][2] = vz.remove(0);
    }

    fields.push(Field {
        label: document.first_zone.title,
        vectors: vs,
    });

    // Extract the field components in the subsequent zones.
    for mut zone in document.zones.unwrap() {
        let mut vx = zone.float_list.drain(..nvert).collect::<Vec<f64>>();
        let mut vy = zone.float_list.drain(..nvert).collect::<Vec<f64>>();
        let mut vz = zone.float_list.drain(..nvert).collect::<Vec<f64>>();

        // All floats should be drained.
        assert_eq!(zone.float_list.len(), 0);

        let mut vs: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]; nvert];

        // Extract the field components in the zone.
        for i in 0..nvert {
            vs[i][0] = vx.remove(0);
            vs[i][1] = vy.remove(0);
            vs[i][2] = vz.remove(0);
        }

        fields.push(Field {
            label: zone.title,
            vectors: vs,
        });
    }

    Mesh {
        label: document.title,
        vertices: vertices,
        elements: elements,
        submesh_indices: submesh_indices,
        fields: fields,
        volume: None,
    }
}

#[cfg(test)]
mod tests {

    use std::fs;
    use std::path::Path;

    use crate::tecplot_parser::{create_mesh_from_tecplot, tecplot};

    #[test]
    fn test_title() {
        let result = tecplot::TitleParser::new()
            .parse("TITLE = \"My test title\"")
            .expect("Parse failed");
        let expected: String = "\"My test title\"".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_variables() {
        let result = tecplot::VariablesParser::new()
            .parse("VARIABLES = \"x\", \"y\", \"z\"")
            .expect("Parse failed");
        let expected = vec!["\"x\"", "\"y\"", "\"z\""];
        assert_eq!(result.len(), expected.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expected[i]);
        }
    }

    #[test]
    fn test_basic_one_zone() {
        let root_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("basic_one_zone");

        let golden_file = root_file_path.join("example.tec");
        let file_contents = fs::read_to_string(golden_file).expect("Failed to read tecplot file");
        let result = tecplot::DocumentParser::new()
            .parse(&file_contents)
            .expect("Parse failed");

        let expected_title: String = "\"sphere_0000\"".to_string();
        assert_eq!(result.title, expected_title);

        let expected_variables = vec![
            "\"X\"", "\"Y\"", "\"Z\"", "\"Mx\"", "\"My\"", "\"Mz\"", "\"SD\"",
        ];
        assert_eq!(result.variables.len(), expected_variables.len());
        for i in 0..result.variables.len() {
            assert_eq!(result.variables[i], expected_variables[i]);
        }

        let expected_floats_txt = root_file_path.join("expected_floats_zone_1.txt");
        let expected_floats = fs::read_to_string(expected_floats_txt)
            .expect("Failed to read x file")
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(result.first_zone.float_list, expected_floats);

        let expected_ints_txt = root_file_path.join("expected_ints_zone_1.txt");
        let expected_ints = fs::read_to_string(expected_ints_txt)
            .expect("Failed to read x file")
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(result.first_zone.integer_list, expected_ints);

        let expected_first_zone_title: String = "\"\"".to_string();
        assert_eq!(result.first_zone.title, expected_first_zone_title);

        let expected_first_zone_no_of_vertices: usize = 7025;
        assert_eq!(
            result.first_zone.no_of_vertices,
            expected_first_zone_no_of_vertices
        );

        let expected_first_zone_no_of_elements: usize = 37375;
        assert_eq!(
            result.first_zone.no_of_elements,
            expected_first_zone_no_of_elements
        );
    }

    #[test]
    fn test_histo_two_zone() {
        let root_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("histo_two_zones");

        let golden_file = root_file_path.join("example.tec");
        let file_contents = fs::read_to_string(golden_file).expect("Failed to read tecplot file");
        let result = tecplot::DocumentParser::new()
            .parse(&file_contents)
            .expect("Parse failed");

        let expected_title: String = "\"Histo two zones\"".to_string();
        assert_eq!(result.title, expected_title);

        let expected_variables = vec![
            "\"X\"", "\"Y\"", "\"Z\"", "\"Mx\"", "\"My\"", "\"Mz\"", "\"SD\"",
        ];
        assert_eq!(result.variables.len(), expected_variables.len());
        for i in 0..result.variables.len() {
            assert_eq!(result.variables[i], expected_variables[i]);
        }

        // First zone

        let expected_floats_zone_1_txt = root_file_path.join("expected_floats_zone_1.txt");
        let expected_floats_zone_1 = fs::read_to_string(expected_floats_zone_1_txt)
            .expect("Failed to read floats zone 1 file")
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(result.first_zone.float_list, expected_floats_zone_1);

        let expected_ints_zone_1_txt = root_file_path.join("expected_ints_zone_1.txt");
        let expected_ints_zone_1 = fs::read_to_string(expected_ints_zone_1_txt)
            .expect("Failed to read integers zone 1 file")
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(result.first_zone.integer_list, expected_ints_zone_1);

        let expected_first_zone_title: String = "\"400.0000 mT\"".to_string();
        assert_eq!(result.first_zone.title, expected_first_zone_title);

        let expected_first_zone_no_of_vertices: usize = 70;
        assert_eq!(
            result.first_zone.no_of_vertices,
            expected_first_zone_no_of_vertices
        );

        let expected_first_zone_no_of_elements: usize = 200;
        assert_eq!(
            result.first_zone.no_of_elements,
            expected_first_zone_no_of_elements
        );

        // Second zone

        let zones = &result.zones.unwrap();

        let expected_floats_zone_2_txt = root_file_path.join("expected_floats_zone_2.txt");
        let expected_floats_zone_2 = fs::read_to_string(expected_floats_zone_2_txt)
            .expect("Failed to read floats zone 1 file")
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(zones[0].float_list, expected_floats_zone_2);

        let expected_second_zone_title: String = "\"380.0000 mT\"".to_string();
        assert_eq!(zones[0].title, expected_second_zone_title);

        let expected_second_zone_no_of_vertices: usize = 70;
        assert_eq!(zones[0].no_of_vertices, expected_second_zone_no_of_vertices);

        let expected_second_zone_no_of_elements: usize = 200;
        assert_eq!(zones[0].no_of_elements, expected_second_zone_no_of_elements);
    }

    #[test]
    fn test_create_mesh_from_tecplot() {
        let root_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("histo_two_zones");

        let golden_file = root_file_path.join("example.tec");

        let mesh = create_mesh_from_tecplot(golden_file.to_str().unwrap());

        println!("{:?}", mesh.vertices);
        println!("{:?}", mesh.submesh_indices);
        println!("{:?}", mesh.elements);

        for field in mesh.fields.iter() {
            println!("{:?}", field.label);
            println!("{:?}", field.vectors);
        }
    }
}
