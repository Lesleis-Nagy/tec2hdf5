use crate::geometry::{
    tet_volume,
    tet_lin_vec_integral
};

/// Represents a vector field associated with a mesh, containing a label and a collection of 3D
/// vectors.
///
/// Fields are typically used to store additional information linked to the mesh,
/// such as vector fields, physical quantities, or other data associated with the model.
///
/// # Fields
///
/// * `label` - A string identifier for the field.
/// * `vectors` - A vector of 3D points (represented as arrays of three `f64` values),
///               holding the information associated with the field.
pub struct Field {
    pub label: String,
    pub vectors: Vec<[f64; 3]>,
}

///
/// Represents a 3D mesh model, including its geometry, connectivity, and associated data fields.
///
/// # Fields
///
/// * `label` - A string identifier for the mesh.
/// * `vertices` - A vector of 3D points (represented as arrays of three `f64` values),
///                representing the geometry of the mesh.
/// * `elements` - A vector of elements (represented as arrays of four `usize` values),
///                where each element connects four vertices, defining the connectivity of the mesh.
/// * `submesh_indices` - A vector of indices indicating subdivisions of the mesh into submeshes.
/// * `fields` - A vector of `Field` structs holding additional information associated with the mesh,
///              such as vector fields or other data.
/// * `volume` - An optional precomputed volume of the mesh (if available). If `None`,
///              the volume can be computed using the appropriate method.
///
/// The `Mesh` struct provides methods for creating new meshes, as well as computing derived
/// quantities such as volume.
///
pub struct Mesh {
    pub label: String,
    pub vertices: Vec<[f64; 3]>,
    pub elements: Vec<[usize; 4]>,
    pub submesh_indices: Vec<usize>,
    pub fields: Vec<Field>,
    pub volume: Option<f64>,
    pub net_moments: Option<Vec<[f64; 3]>>,
}

impl Mesh {
    ///
    /// Creates a new `Mesh` instance with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `label` - A string identifier for the mesh.
    /// * `vertices` - A vector of 3D points (represented as arrays of three `f64` values),
    ///                representing the geometry of the mesh.
    /// * `elements` - A vector of elements (represented as arrays of four `usize` values),
    ///                where each element connects four vertices, defining the connectivity of the mesh.
    /// * `submesh_indices` - A vector of indices indicating subdivisions of the mesh into submeshes.
    /// * `fields` - A vector of `Field` structs holding additional information associated with the mesh,
    ///              such as vector fields or other data.
    ///
    /// # Returns
    ///
    /// A newly created `Mesh` instance with the specified properties.
    ///
    /// The `volume` field of the returned `Mesh` is initialized to `None`. It can be computed
    /// later using the `volume` method.
    pub fn new(
        label: String,
        vertices: Vec<[f64; 3]>,
        elements: Vec<[usize; 4]>,
        submesh_indices: Vec<usize>,
        fields: Vec<Field>,
    ) -> Mesh {
        Mesh {
            label,
            vertices,
            elements,
            submesh_indices,
            fields,
            volume: None,
            net_moments: None,
        }
    }

    ///
    /// Computes the volume of the mesh using the mesh's elements and vertices.
    ///
    /// # Returns
    ///
    /// Returns the computed volume of the mesh as an `f64`.
    ///
    /// The volume is calculated by summing up the contributions from each element of the mesh,
    /// assuming that the elements form tetrahedra in 3D space. The vertices of each tetrahedron
    /// are used in the computation to determine its signed volume.
    ///
    /// The result is cached in the `volume` field for future use.
    ///
    /// # Note
    ///
    /// The method assumes that the mesh is well-formed and that indices in the `elements` field
    /// correctly reference valid vertices in the `vertices` field. If the mesh is invalid or
    /// not manifold, the behavior of this method is undefined.
    ///
    /// # Example
    ///
    /// ```
    /// let mut mesh = Mesh::new(
    ///     String::from("example"),
    ///     vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    ///     vec![[0, 1, 2, 3]],
    ///     vec![],
    ///     vec![],
    /// );
    /// let volume = mesh.compute_volume();
    /// println!("Volume: {}", volume);
    /// ```
    ///
    pub fn compute_volume(&mut self) {
        let mut volume: f64 = 0.0;
        for element in &self.elements {
            let v0: &[f64; 3] = &self.vertices[element[0]];
            let v1: &[f64; 3] = &self.vertices[element[1]];
            let v2: &[f64; 3] = &self.vertices[element[2]];
            let v3: &[f64; 3] = &self.vertices[element[3]];
            volume += tet_volume(*v0, *v1, *v2, *v3);
        }
        self.volume = Some(volume);
    }

    pub fn compute_net_moments(&mut self)  {
        let mut net_moments: Vec<[f64; 3]> = Vec::new();
        for field in &self.fields {
            let mut net_moment: [f64; 3] = [0.0, 0.0, 0.0];
            for element in &self.elements {
                let v0: &[f64; 3] = &self.vertices[element[0]];
                let v1: &[f64; 3] = &self.vertices[element[1]];
                let v2: &[f64; 3] = &self.vertices[element[2]];
                let v3: &[f64; 3] = &self.vertices[element[3]];
                let f0: &[f64; 3] = &field.vectors[element[0]];
                let f1: &[f64; 3] = &field.vectors[element[1]];
                let f2: &[f64; 3] = &field.vectors[element[2]];
                let f3: &[f64; 3] = &field.vectors[element[3]];
                let moment = tet_lin_vec_integral(*v0, *v1, *v2, *v3, *f0, *f1, *f2, *f3);
                net_moment[0] += moment[0];
                net_moment[1] += moment[1];
                net_moment[2] += moment[2];
            }
            net_moments.push(net_moment);
        }
        self.net_moments = Some(net_moments);
    }
}
