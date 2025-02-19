use crate::linalg::Determinant;

/// Calculates the volume of a tetrahedron defined by four vertices in 3D space.
///
/// # Parameters
///
/// - `v0`: A 3D point representing the first vertex of the tetrahedron, given as `[f64; 3]`.
/// - `v1`: A 3D point representing the second vertex of the tetrahedron, given as `[f64; 3]`.
/// - `v2`: A 3D point representing the third vertex of the tetrahedron, given as `[f64; 3]`.
/// - `v3`: A 3D point representing the fourth vertex of the tetrahedron, given as `[f64; 3]`.
///
/// # Returns
///
/// The volume of the tetrahedron as a `f64`. The result is always positive.
///
/// # Formula
///
/// The formula used to calculate the tetrahedron volume is:
/// ```
/// Volume = |det| / 6
/// ```
/// where `det` is the determinant of a 4x4 matrix constructed using the
/// vertices with an additional column of ones.
///
/// # Examples
///
/// ```
/// use crate::tet_volume;
///
/// let v0 = [0.0, 0.0, 0.0];
/// let v1 = [1.0, 0.0, 0.0];
/// let v2 = [0.0, 1.0, 0.0];
/// let v3 = [0.0, 0.0, 1.0];
///
/// let volume = tet_volume(v0, v1, v2, v3);
/// assert_eq!(volume, 1.0 / 6.0);
/// ```
pub fn tet_volume(v0: [f64; 3], v1: [f64; 3], v2: [f64; 3], v3: [f64; 3]) -> f64 {
    (1.0 / 6.0)
        * [
            [v0[0], v0[1], v0[2], 1.0],
            [v1[0], v1[1], v1[2], 1.0],
            [v2[0], v2[1], v2[2], 1.0],
            [v3[0], v3[1], v3[2], 1.0],
        ]
        .determinant()
}

///
/// Calculates the integral of a linear scalar field over a tetrahedron in 3D space.
///
/// # Parameters
///
/// - `v0`, `v1`, `v2`, `v3`: Vertices of the tetrahedron as 3D points, each given as `[f64; 3]`.
/// - `s0`, `s1`, `s2`, `s3`: Values of the scalar field at the respective vertices of the
///                           tetrahedron.
///
/// # Returns
///
/// The integral of the linear scalar field over the tetrahedron as an `f64`.
///
/// # Formula
///
/// The integral is calculated as:
/// ```
/// Integral = Volume * (s0 + s1 + s2 + s3) / 4.0
/// ```
/// where `Volume` is the absolute value of the tetrahedron's volume.
///
/// # Examples
///
/// ```
/// use crate::tet_lin_scal_integral;
///
/// let v0 = [0.0, 0.0, 0.0];
/// let v1 = [1.0, 0.0, 0.0];
/// let v2 = [0.0, 1.0, 0.0];
/// let v3 = [0.0, 0.0, 1.0];
///
/// let s0 = 1.0;
/// let s1 = 2.0;
/// let s2 = 3.0;
/// let s3 = 4.0;
///
/// let integral = tet_lin_scal_integral(v0, v1, v2, v3, s0, s1, s2, s3);
/// assert_eq!(integral, (1.0 / 6.0) * (s0 + s1 + s2 + s3) / 4.0);
/// ```
pub fn tet_lin_scal_integral(
    v0: [f64; 3],
    v1: [f64; 3],
    v2: [f64; 3],
    v3: [f64; 3],
    s0: f64,
    s1: f64,
    s2: f64,
    s3: f64,
) -> f64 {
    let v: f64 = tet_volume(v0, v1, v2, v3).abs();
    v * (s0 + s1 + s2 + s3) / 4.0
}

///
/// Calculates the integral of a linearly interpolated vector field over a tetrahedron in 3D space.
///
/// # Parameters
///
/// - `v0`, `v1`, `v2`, `v3`: Vertices of the tetrahedron, each represented as a 3D point `[f64; 3]`.
/// - `f0`, `f1`, `f2`, `f3`: Values of the vector field at the corresponding vertices of the tetrahedron,
///                           each represented as a 3D vector `[f64; 3]`.
///
/// # Returns
///
/// The integral of the vector field over the tetrahedron as a 3D vector `[f64; 3]`.
///
/// # Formula
///
/// The integral is calculated as:
/// ```
/// Integral[i] = Volume * (f0[i] + f1[i] + f2[i] + f3[i]) / 4.0
/// ```
/// for each component `i` (x, y, z), where `Volume` is the absolute value of the tetrahedron's volume.
///
/// # Examples
///
/// ```
/// use crate::tet_lin_vec_integral;
///
/// let v0 = [0.0, 0.0, 0.0];
/// let v1 = [1.0, 0.0, 0.0];
/// let v2 = [0.0, 1.0, 0.0];
/// let v3 = [0.0, 0.0, 1.0];
///
/// let f0 = [1.0, 0.0, 0.0];
/// let f1 = [0.0, 1.0, 0.0];
/// let f2 = [0.0, 0.0, 1.0];
/// let f3 = [1.0, 1.0, 1.0];
///
/// let integral = tet_lin_vec_integral(v0, v1, v2, v3, f0, f1, f2, f3);
/// assert_eq!(integral, [1.0 / 24.0, 1.0 / 24.0, 1.0 / 24.0]);
/// ```
pub fn tet_lin_vec_integral(
    v0: [f64; 3],
    v1: [f64; 3],
    v2: [f64; 3],
    v3: [f64; 3],
    f0: [f64; 3],
    f1: [f64; 3],
    f2: [f64; 3],
    f3: [f64; 3],
) -> [f64; 3] {
    let v: f64 = tet_volume(v0, v1, v2, v3).abs();
    let mut integral = [0.0; 3];
    for i in 0..3 {
        integral[i] = v * (f0[i] + f1[i] + f2[i] + f3[i]) / 4.0;
    }
    integral
}

//----------------------------------------------------------------------------------------------//
//- Tests                                                                                      -//
//----------------------------------------------------------------------------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    //..........................................................................................//
    //. tet_volume()                                                                           .//
    //..........................................................................................//

    #[test]
    fn test_tet_volume_simple_case() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let v3 = [0.0, 0.0, 1.0];

        let volume = tet_volume(v0, v1, v2, v3);
        assert!((volume - (-1.0 / 6.0)).abs() < 1e-9);
    }

    #[test]
    fn test_tet_volume_flat_tetrahedron() {
        // All points are in the same plane, volume should be zero
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let v3 = [1.0, 1.0, 0.0];

        let volume = tet_volume(v0, v1, v2, v3);
        assert!((volume - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_tet_volume_large_coordinates() {
        let v0 = [100.0, 0.0, 0.0];
        let v1 = [0.0, 100.0, 0.0];
        let v2 = [0.0, 0.0, 100.0];
        let v3 = [100.0, 100.0, 100.0];

        let volume = tet_volume(v0, v1, v2, v3);
        assert!((volume - (-333333.3333333333)).abs() < 1e-9); // 100^3 / 6
    }

    #[test]
    fn test_tet_volume_negative_coordinates() {
        let v0 = [-1.0, -1.0, -1.0];
        let v1 = [1.0, -1.0, -1.0];
        let v2 = [-1.0, 1.0, -1.0];
        let v3 = [-1.0, -1.0, 1.0];

        let volume = tet_volume(v0, v1, v2, v3);
        assert!((volume - (-8.0 / 6.0)).abs() < 1e-9);
    }

    #[test]
    fn test_tet_volume_degenerate_case() {
        // All points are the same, volume should be zero
        let v0 = [1.0, 1.0, 1.0];
        let v1 = [1.0, 1.0, 1.0];
        let v2 = [1.0, 1.0, 1.0];
        let v3 = [1.0, 1.0, 1.0];

        let volume = tet_volume(v0, v1, v2, v3);
        assert!((volume - 0.0).abs() < 1e-9);
    }

    //..........................................................................................//
    //. tet_lin_scal_integral()                                                                .//
    //..........................................................................................//

    #[test]
    fn test_tet_lin_scal_integral() {
        // Define a standard tetrahedron
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let v3 = [0.0, 0.0, 1.0];

        // Define scalar field values at vertices
        let s0 = 1.0;
        let s1 = 2.0;
        let s2 = 3.0;
        let s3 = 4.0;

        // Compute integral
        let integral = tet_lin_scal_integral(v0, v1, v2, v3, s0, s1, s2, s3);

        // Expected volume of a unit tetrahedron
        let volume = 1.0 / 6.0;

        // Expected integral: V * (s0 + s1 + s2 + s3) / 4
        let expected_integral = volume * (s0 + s1 + s2 + s3) / 4.0;

        // Allow small numerical error
        let tolerance = 1e-10;
        assert!((integral - expected_integral).abs() < tolerance,
                "Expected {}, got {}", expected_integral, integral);
    }

    //..........................................................................................//
    //. tet_lin_vec_integral()                                                                 .//
    //..........................................................................................//

    #[test]
    fn test_tet_lin_vec_integral() {
        // Define a standard tetrahedron
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];
        let v3 = [0.0, 0.0, 1.0];

        // Define vector field values at vertices
        let f0 = [1.0, 2.0, 3.0];
        let f1 = [2.0, 3.0, 4.0];
        let f2 = [3.0, 4.0, 5.0];
        let f3 = [4.0, 5.0, 6.0];

        // Compute integral
        let integral = tet_lin_vec_integral(v0, v1, v2, v3, f0, f1, f2, f3);

        // Expected volume of the unit tetrahedron
        let volume = 1.0 / 6.0;

        // Compute expected integral component-wise
        let expected_integral = [
            volume * (f0[0] + f1[0] + f2[0] + f3[0]) / 4.0,
            volume * (f0[1] + f1[1] + f2[1] + f3[1]) / 4.0,
            volume * (f0[2] + f1[2] + f2[2] + f3[2]) / 4.0,
        ];

        // Allow a small numerical error
        let tolerance = 1e-10;
        for i in 0..3 {
            assert!(
                (integral[i] - expected_integral[i]).abs() < tolerance,
                "Component {}: expected {}, got {}", i, expected_integral[i], integral[i]
            );
        }
    }

}
