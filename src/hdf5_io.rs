use crate::mesh::{
    Mesh,
    Field
};

use hdf5::{
    File,
    Result,
    types::FixedAscii
};

use ndarray::{
    Array2,
};

/// Writes a `Mesh` object to an HDF5 file.
///
/// The function saves the mesh's vertices, elements, submesh indices, and fields data to the
/// specified HDF5 file. It organizes the data into the following groups:
/// - `/mesh/vertices`: Stores the vertex positions as Nx3 `f64` array.
/// - `/mesh/elements`: Stores the element connectivity as Mx4 `usize` array.
/// - `/mesh/submesh`: Stores the submesh indices as Nx1 `usize` array.
/// - `/fields/field{}/vectors`: Stores the vector data for each field as Px3 `f64` array.
/// - `/fields/labels`: Stores the labels for each field as fixed-length ASCII strings.
///
/// # Parameters
/// - `mesh`: A reference to the `Mesh` object to be written to the file.
/// - `filename`: The name of the HDF5 file to create and write data to.
///
/// # Returns
/// - `Result<()>`: Returns `Ok(())` on success or an error if the file could not be created or data
///   could not be written.
///
/// # Errors
/// This function will return an error if:
/// - The HDF5 file cannot be created or accessed.
/// - There is an issue with writing data to the datasets.
/// - Any of the field labels exceed the maximum allowable length for a fixed-width ASCII string
///   (64 characters by default).
///
/// # Panics
/// Panics if a field label cannot be truncated or converted into fixed-width ASCII format.
///
/// # Example
/// ```
/// use crate::mesh::{Mesh, Field};
///
/// let fields: Vec<Field> = vec![
///     Field {
///         label: String::from("field1"),
///         vectors: vec![
///             [0.0, 0.0, 0.0],
///             [1.0, 0.0, 0.0],
///             [0.0, 1.0, 0.0],
///             [0.0, 0.0, 1.0]
///         ]
///     }
/// ];
///
/// let mesh = Mesh {
///     label: String::from("My mesh"),
///     vertices: vec![
///         [0.0, 0.0, 0.0],
///         [1.0, 0.0, 0.0],
///         [0.0, 1.0, 0.0],
///         [0.0, 0.0, 1.0]
///     ],
///     elements: vec![
///         [0, 1, 2, 3]
///     ],
///     submesh_indices: vec![1],
///     fields,
///     volume: None,
///     net_moments: None
/// };
///
/// write_mesh_to_hdf5(&mesh, "test.h5").unwrap();
/// ```
pub fn write_mesh_to_hdf5(mesh: &Mesh, filename: &str) -> Result<()> {
    const MAX_STR_LEN: usize = 64;
    let file = File::create(filename)?;

    let n_verts = mesh.vertices.len();
    let vertices: Array2<f64> = Array2::from_shape_vec(
        (n_verts, 3),
        mesh.vertices.iter().flatten().copied().collect()
    )?;

    let n_elems = mesh.elements.len();
    let elements: Array2<usize> = Array2::from_shape_vec(
        (n_elems, 4),
        mesh.elements.iter().flatten().copied().collect()
    )?;

    file.new_dataset::<f64>()
        .shape((n_verts, 3))
        .create("/mesh/vertices")?
        .write(&vertices)?;

    file.new_dataset::<usize>()
        .shape((n_elems, 4))
        .create("/mesh/elements")?
        .write(&elements)?;

    file.new_dataset::<usize>()
        .shape(n_elems,)
        .create("/mesh/submesh")?
        .write(&mesh.submesh_indices)?;

    for (field_index, field) in mesh.fields.iter().enumerate() {
        let field_name = format!("/fields/field{}/vectors", field_index);
        let field_data: Array2<f64> = Array2::from_shape_vec(
            (field.vectors.len(), 3),
            field.vectors.iter().flatten().copied().collect()
        )?;

        file.new_dataset::<f64>()
            .shape((n_verts, 3))
            .create(field_name.as_str())?
            .write(&field_data)?;
    }

    let fixed_width_field_labels: Vec<FixedAscii<MAX_STR_LEN>> = mesh
        .fields
        .iter()
        .map(
            |field| {
                let mut field_label = field.label.clone();
                field_label.truncate(MAX_STR_LEN);
                FixedAscii::<MAX_STR_LEN>::from_ascii(field_label.as_str()).unwrap_or_else(
                    |_| {
                        panic!("Field label too long")
                    })
            }
        )
        .collect();

    file.new_dataset::<FixedAscii<MAX_STR_LEN>>()
        .shape((fixed_width_field_labels.len(),))
        .create("/fields/labels")?
        .write(&fixed_width_field_labels)?;

    Ok(())
}

pub fn read_mesh_from_hdf5(filename: &str) -> Result<Mesh> {

}

#[cfg(test)]
mod tests {

    use super::*;

    //......................................................................................//
    //. write_mesh_to_hdf5()                                                               .//
    //......................................................................................//

    #[test]
    fn test_write_mesh_to_hdf5() {
        let fields: Vec<Field> = vec![
            Field {
                label: String::from("field1"),
                vectors: vec![
                    [0.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0]
                ]
            },
            Field {
                label: String::from("field2"),
                vectors: vec![
                    [1.0, 2.0, 3.0],
                    [4.0, 5.0, 6.0],
                    [7.0, 8.0, 9.0],
                    [10.0, 11.0, 12.0]
                ]
            }
        ];
        let mesh = Mesh {
            label: String::from("My mesh"),
            vertices: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ],
            elements: vec![
                [0, 1, 2, 3]
            ],
            submesh_indices: vec![1],
            fields,
            volume: None,
            net_moments: None
        };

        write_mesh_to_hdf5(&mesh, "test.h5").unwrap();

        assert!(true);
    }

}