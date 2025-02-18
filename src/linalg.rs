/// Calculates the determinant of a square matrix.
///
/// This method is implemented for square matrices of sizes 2x2, 3x3, and 4x4.
/// The determinant represents a scalar value that can indicate properties
/// such as whether a matrix is invertible and the scaling factor of
/// transformation described by the matrix.
///
/// # Returns
///
/// A `f64` representing the determinant value of the matrix.
///
/// # Examples
///
/// ## 2x2 Matrix
/// ```rust
/// use crate::linalg::Determinant; // Adjust the crate/module import as needed.
///
/// let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
/// assert_eq!(matrix.determinant(), -2.0);
/// ```
///
/// ## 3x3 Matrix
/// ```rust
/// use crate::linalg::Determinant; // Adjust the crate/module import as needed.
///
/// let matrix: [[f64; 3]; 3] = [
///     [6.0, 1.0, 1.0],
///     [4.0, -2.0, 5.0],
///     [2.0, 8.0, 7.0]
/// ];
/// assert_eq!(matrix.determinant(), -306.0);
/// ```
///
/// ## 4x4 Matrix
/// ```rust
/// use your_crate::Determinant; // Adjust the crate/module import as needed.
///
/// let matrix: [[f64; 4]; 4] = [
///     [3.0, 2.0, -1.0, 4.0],
///     [2.0, 1.0,  5.0, 7.0],
///     [0.0, 5.0,  2.0, -6.0],
///     [1.0, 2.0,  3.0, 0.0]
/// ];
/// assert_eq!(matrix.determinant(), 142.0);
/// ```
pub trait Determinant {
    fn determinant(&self) -> f64;
}

impl Determinant for [[f64; 2]; 2] {
    fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Determinant for [[f64; 3]; 3] {
    fn determinant(&self) -> f64 {
        self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
            - self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0])
            + self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }
}

impl Determinant for [[f64; 4]; 4] {
    fn determinant(&self) -> f64 {
        let m1 = self[1][1] * (self[2][2] * self[3][3] - self[2][3] * self[3][2])
            - self[1][2] * (self[2][1] * self[3][3] - self[2][3] * self[3][1])
            + self[1][3] * (self[2][1] * self[3][2] - self[2][2] * self[3][1]);

        let m2 = self[1][0] * (self[2][2] * self[3][3] - self[2][3] * self[3][2])
            - self[1][2] * (self[2][0] * self[3][3] - self[2][3] * self[3][0])
            + self[1][3] * (self[2][0] * self[3][2] - self[2][2] * self[3][0]);

        let m3 = self[1][0] * (self[2][1] * self[3][3] - self[2][3] * self[3][1])
            - self[1][1] * (self[2][0] * self[3][3] - self[2][3] * self[3][0])
            + self[1][3] * (self[2][0] * self[3][1] - self[2][1] * self[3][0]);

        let m4 = self[1][0] * (self[2][1] * self[3][2] - self[2][2] * self[3][1])
            - self[1][1] * (self[2][0] * self[3][2] - self[2][2] * self[3][0])
            + self[1][2] * (self[2][0] * self[3][1] - self[2][1] * self[3][0]);

        self[0][0] * m1 - self[0][1] * m2 + self[0][2] * m3 - self[0][3] * m4
    }
}

/// The `Adjugate` trait provides a method to compute the adjugate (also known as the adjoint) 
/// of a square matrix. The adjugate is the transpose of the cofactor matrix of the given matrix. 
///
/// This trait is implemented for 2x2, 3x3, and 4x4 matrices represented as fixed-size Rust arrays.
///
/// # Examples
///
/// ## 2x2 Matrix
/// ```rust
/// use your_crate::Adjugate; // Adjust the crate/module import as needed.
///
/// let matrix: [[f64; 2]; 2] = [
///     [4.0, 3.0],
///     [3.0, 2.0]
/// ];
/// let adjugate = matrix.adj();
/// assert_eq!(adjugate, [
///     [2.0, -3.0],
///     [-3.0, 4.0]
/// ]);
/// ```
///
/// ## 3x3 Matrix
/// ```rust
/// use your_crate::Adjugate; // Adjust the crate/module import as needed.
///
/// let matrix: [[f64; 3]; 3] = [
///     [1.0, 2.0, 3.0],
///     [0.0, 1.0, 4.0],
///     [5.0, 6.0, 0.0]
/// ];
/// let adjugate = matrix.adj();
/// assert_eq!(adjugate, [
///     [-24.0,  18.0,  5.0],
///     [ 20.0, -15.0, -4.0],
///     [ -5.0,   4.0,  1.0]
/// ]);
/// ```
///
/// ## 4x4 Matrix
/// ```rust
/// use your_crate::Adjugate; // Adjust the crate/module import as needed.
///
/// let matrix: [[f64; 4]; 4] = [
///     [3.0, 2.0, -1.0, 4.0],
///     [2.0, 1.0,  5.0, 7.0],
///     [0.0, 5.0,  2.0, -6.0],
///     [1.0, 2.0,  3.0, 0.0]
/// ];
/// let adjugate = matrix.adj();
/// // Result is the transpose of the cofactor matrix.
/// assert_eq!(adjugate, [
///     [142.0, -124.0, 36.0, 4.0],
///     [-80.0, 56.0, -16.0, 0.0],
///     [58.0, -28.0, 4.0, -9.0],
///     [-48.0, 20.0, -6.0, 26.0]
/// ]);
/// ```
pub trait Adjugate: Sized {
    fn adj(&self) -> Self;
}

impl Adjugate for [[f64; 2]; 2] {
    fn adj(&self) -> [[f64; 2]; 2] {
        let m = self;
        [[m[1][1], -m[0][1]], [-m[1][0], m[0][0]]]
    }
}

impl Adjugate for [[f64; 3]; 3] {
    fn adj(&self) -> [[f64; 3]; 3] {
        let m = self;
        [
            [
                -m[1][2] * m[2][1] + m[1][1] * m[2][2],
                m[0][2] * m[2][1] - m[0][1] * m[2][2],
                -m[0][2] * m[1][1] + m[0][1] * m[1][2],
            ],
            [
                m[1][2] * m[2][0] - m[1][0] * m[2][2],
                -m[0][2] * m[2][0] + m[0][0] * m[2][2],
                m[0][2] * m[1][0] - m[0][0] * m[1][2],
            ],
            [
                -m[1][1] * m[2][0] + m[1][0] * m[2][1],
                m[0][1] * m[2][0] - m[0][0] * m[2][1],
                -m[0][1] * m[1][0] + m[0][0] * m[1][1],
            ],
        ]
    }
}

impl Adjugate for [[f64; 4]; 4] {
    fn adj(&self) -> [[f64; 4]; 4] {
        let m = self;
        [
            [
                -m[1][3] * m[2][2] * m[3][1]
                    + m[1][2] * m[2][3] * m[3][1]
                    + m[1][3] * m[2][1] * m[3][2]
                    - m[1][1] * m[2][3] * m[3][2]
                    - m[1][2] * m[2][1] * m[3][3]
                    + m[1][1] * m[2][2] * m[3][3],
                m[0][3] * m[2][2] * m[3][1]
                    - m[0][2] * m[2][3] * m[3][1]
                    - m[0][3] * m[2][1] * m[3][2]
                    + m[0][1] * m[2][3] * m[3][2]
                    + m[0][2] * m[2][1] * m[3][3]
                    - m[0][1] * m[2][2] * m[3][3],
                -m[0][3] * m[1][2] * m[3][1]
                    + m[0][2] * m[1][3] * m[3][1]
                    + m[0][3] * m[1][1] * m[3][2]
                    - m[0][1] * m[1][3] * m[3][2]
                    - m[0][2] * m[1][1] * m[3][3]
                    + m[0][1] * m[1][2] * m[3][3],
                m[0][3] * m[1][2] * m[2][1]
                    - m[0][2] * m[1][3] * m[2][1]
                    - m[0][3] * m[1][1] * m[2][2]
                    + m[0][1] * m[1][3] * m[2][2]
                    + m[0][2] * m[1][1] * m[2][3]
                    - m[0][1] * m[1][2] * m[2][3],
            ],
            [
                m[1][3] * m[2][2] * m[3][0]
                    - m[1][2] * m[2][3] * m[3][0]
                    - m[1][3] * m[2][0] * m[3][2]
                    + m[1][0] * m[2][3] * m[3][2]
                    + m[1][2] * m[2][0] * m[3][3]
                    - m[1][0] * m[2][2] * m[3][3],
                -m[0][3] * m[2][2] * m[3][0]
                    + m[0][2] * m[2][3] * m[3][0]
                    + m[0][3] * m[2][0] * m[3][2]
                    - m[0][0] * m[2][3] * m[3][2]
                    - m[0][2] * m[2][0] * m[3][3]
                    + m[0][0] * m[2][2] * m[3][3],
                m[0][3] * m[1][2] * m[3][0]
                    - m[0][2] * m[1][3] * m[3][0]
                    - m[0][3] * m[1][0] * m[3][2]
                    + m[0][0] * m[1][3] * m[3][2]
                    + m[0][2] * m[1][0] * m[3][3]
                    - m[0][0] * m[1][2] * m[3][3],
                -m[0][3] * m[1][2] * m[2][0]
                    + m[0][2] * m[1][3] * m[2][0]
                    + m[0][3] * m[1][0] * m[2][2]
                    - m[0][0] * m[1][3] * m[2][2]
                    - m[0][2] * m[1][0] * m[2][3]
                    + m[0][0] * m[1][2] * m[2][3],
            ],
            [
                -m[1][3] * m[2][1] * m[3][0]
                    + m[1][1] * m[2][3] * m[3][0]
                    + m[1][3] * m[2][0] * m[3][1]
                    - m[1][0] * m[2][3] * m[3][1]
                    - m[1][1] * m[2][0] * m[3][3]
                    + m[1][0] * m[2][1] * m[3][3],
                m[0][3] * m[2][1] * m[3][0]
                    - m[0][1] * m[2][3] * m[3][0]
                    - m[0][3] * m[2][0] * m[3][1]
                    + m[0][0] * m[2][3] * m[3][1]
                    + m[0][1] * m[2][0] * m[3][3]
                    - m[0][0] * m[2][1] * m[3][3],
                -m[0][3] * m[1][1] * m[3][0]
                    + m[0][1] * m[1][3] * m[3][0]
                    + m[0][3] * m[1][0] * m[3][1]
                    - m[0][0] * m[1][3] * m[3][1]
                    - m[0][1] * m[1][0] * m[3][3]
                    + m[0][0] * m[1][1] * m[3][3],
                m[0][3] * m[1][1] * m[2][0]
                    - m[0][1] * m[1][3] * m[2][0]
                    - m[0][3] * m[1][0] * m[2][1]
                    + m[0][0] * m[1][3] * m[2][1]
                    + m[0][1] * m[1][0] * m[2][3]
                    - m[0][0] * m[1][1] * m[2][3],
            ],
            [
                m[1][2] * m[2][1] * m[3][0]
                    - m[1][1] * m[2][2] * m[3][0]
                    - m[1][2] * m[2][0] * m[3][1]
                    + m[1][0] * m[2][2] * m[3][1]
                    + m[1][1] * m[2][0] * m[3][2]
                    - m[1][0] * m[2][1] * m[3][2],
                -m[0][2] * m[2][1] * m[3][0]
                    + m[0][1] * m[2][2] * m[3][0]
                    + m[0][2] * m[2][0] * m[3][1]
                    - m[0][0] * m[2][2] * m[3][1]
                    - m[0][1] * m[2][0] * m[3][2]
                    + m[0][0] * m[2][1] * m[3][2],
                m[0][2] * m[1][1] * m[3][0]
                    - m[0][1] * m[1][2] * m[3][0]
                    - m[0][2] * m[1][0] * m[3][1]
                    + m[0][0] * m[1][2] * m[3][1]
                    + m[0][1] * m[1][0] * m[3][2]
                    - m[0][0] * m[1][1] * m[3][2],
                -m[0][2] * m[1][1] * m[2][0]
                    + m[0][1] * m[1][2] * m[2][0]
                    + m[0][2] * m[1][0] * m[2][1]
                    - m[0][0] * m[1][2] * m[2][1]
                    - m[0][1] * m[1][0] * m[2][2]
                    + m[0][0] * m[1][1] * m[2][2],
            ],
        ]
    }
}


/// A trait that provides a method for calculating the inverse of a square matrix.
///
/// # Requirements
/// The matrix must be square with dimensions `N x N`, and its determinant
/// should not be near zero (e.g., to avoid numerical instability).
///
/// This trait is implemented for matrices of sizes 2x2, 3x3, and 4x4, using 
/// floating-point numbers (`f64`) as the element type.
///
/// # Provided Method
/// - `inv`: Computes the inverse of the matrix if it exists.
///
/// # Example Usage
/// ```
/// use your_crate::Inverse; // Replace `your_crate` with the actual crate name.
///
/// let matrix_2x2 = [[4.0, 7.0], [2.0, 6.0]];
/// if let Some(inverse) = matrix_2x2.inv() {
///     println!("Inverse: {:?}", inverse);
/// } else {
///     println!("Matrix is not invertible");
/// }
/// ```
///
/// For larger matrices (e.g., 3x3 or 4x4), the same approach applies.
pub trait Inverse: Sized {
    fn inv(&self) -> Option<Self>;
}

impl Inverse for [[f64; 2]; 2] {
    fn inv(&self) -> Option<[[f64; 2]; 2]> {
        let det = self.determinant();
        if det.abs() < 1e-14 {
            return None;
        }

        let inv_det = 1.0 / det;
        let adj = self.adj();

        Some([
            [adj[0][0] * inv_det, adj[0][1] * inv_det],
            [adj[1][0] * inv_det, adj[1][1] * inv_det],
        ])
    }
}

impl Inverse for [[f64; 3]; 3] {
    fn inv(&self) -> Option<[[f64; 3]; 3]> {
        let det = self.determinant();
        if det.abs() < 1e-14 {
            return None;
        }

        let inv_det = 1.0 / det;
        let adj = self.adj();

        Some([
            [
                adj[0][0] * inv_det,
                adj[0][1] * inv_det,
                adj[0][2] * inv_det,
            ],
            [
                adj[1][0] * inv_det,
                adj[1][1] * inv_det,
                adj[1][2] * inv_det,
            ],
            [
                adj[2][0] * inv_det,
                adj[2][1] * inv_det,
                adj[2][2] * inv_det,
            ],
        ])
    }
}

impl Inverse for [[f64; 4]; 4] {
    fn inv(&self) -> Option<[[f64; 4]; 4]> {
        let det = self.determinant();
        if det.abs() < 1e-14 {
            return None;
        }

        let inv_det = 1.0 / det;
        let adj = self.adj();

        Some([
            [
                adj[0][0] * inv_det,
                adj[0][1] * inv_det,
                adj[0][2] * inv_det,
                adj[0][3] * inv_det,
            ],
            [
                adj[1][0] * inv_det,
                adj[1][1] * inv_det,
                adj[1][2] * inv_det,
                adj[1][3] * inv_det,
            ],
            [
                adj[2][0] * inv_det,
                adj[2][1] * inv_det,
                adj[2][2] * inv_det,
                adj[2][3] * inv_det,
            ],
            [
                adj[3][0] * inv_det,
                adj[3][1] * inv_det,
                adj[3][2] * inv_det,
                adj[3][3] * inv_det,
            ],
        ])
    }
}

///
/// Given four vectors <v0[0], v0[1], v0[2]>, <v1[0], v1[1], v1[2]>, <v2[0], v2[1], v2[2]> and
/// <v3[0], v3[1], v3[2]>, compute the edge-matrix:
///
///     +-                                               -+
///     | v1[0] - v0[0]    v2[0] - v0[0]    v3[0] - v0[0] |
///     | v1[1] - v0[1]    v2[1] - v0[1]    v3[1] - v0[1] |
///     | v1[2] - v0[2]    v2[2] - v0[2]    v3[2] - v0[2] |
///     +-                                               -+
///
fn edge_matrix(v0: [f64; 3], v1: [f64; 3], v2: [f64; 3], v3: [f64; 3]) -> [[f64; 3]; 3] {
    [
        [v1[0] - v0[0], v2[0] - v0[0], v3[0] - v0[0]],
        [v1[1] - v0[1], v2[1] - v0[1], v3[1] - v0[1]],
        [v1[2] - v0[2], v2[2] - v0[2], v3[2] - v0[2]],
    ]
}

//----------------------------------------------------------------------------------------------//
//- Tests                                                                                      -//
//----------------------------------------------------------------------------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    //..........................................................................................//
    //. test_determinant_2x2()                                                                 .//
    //..........................................................................................//

    #[test]
    fn test_determinant_2x2() {
        let matrix1: [[f64; 2]; 2] = [[3.0, 8.0], [4.0, 6.0]];
        assert!((matrix1.determinant() - (-14.0)).abs() < 1e-14);

        let matrix2: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
        assert!((matrix2.determinant() - (-2.0)).abs() < 1e-14);

        let identity: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
        assert!((identity.determinant() - (1.0)).abs() < 1e-14);

        let zero_matrix: [[f64; 2]; 2] = [[0.0, 0.0], [0.0, 0.0]];
        assert!((zero_matrix.determinant() - (0.0)).abs() < 1e-14);
    }

    //..........................................................................................//
    //. test_determinant_3x3()                                                                 .//
    //..........................................................................................//

    #[test]
    fn test_determinant_3x3() {
        let matrix1: [[f64; 3]; 3] = [[6.0, 1.0, 1.0], [4.0, -2.0, 5.0], [2.0, 8.0, 7.0]];
        assert!((matrix1.determinant() - (-306.0)).abs() < 1e-14);

        let identity: [[f64; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        assert!((identity.determinant() - (1.0)).abs() < 1e-14);

        let singular_matrix: [[f64; 3]; 3] = [[2.0, 4.0, 6.0], [1.0, 2.0, 3.0], [3.0, 6.0, 9.0]];
        assert!((singular_matrix.determinant() - (0.0)).abs() < 1e-14);

        let neg_det_matrix: [[f64; 3]; 3] = [[-3.0, 2.0, 1.0], [1.0, 1.0, -1.0], [2.0, 1.0, 3.0]];
        assert!((neg_det_matrix.determinant() - (-23.0)).abs() < 1e-14);
    }

    //..........................................................................................//
    //. test_determinant_4x4()                                                                 .//
    //..........................................................................................//

    #[test]
    fn test_determinant_4x4() {
        let matrix1: [[f64; 4]; 4] = [
            [3.0, 2.0, -1.0, 4.0],
            [2.0, 1.0, 5.0, 7.0],
            [0.0, 5.0, 2.0, -6.0],
            [1.0, 2.0, 3.0, 0.0],
        ];
        assert!((matrix1.determinant() - 142.0).abs() < 1e-14);

        let identity: [[f64; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        assert!((identity.determinant() - 1.0).abs() < 1e-14);

        let singular_matrix: [[f64; 4]; 4] = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 6.0, 8.0],
            [3.0, 6.0, 9.0, 12.0],
            [4.0, 8.0, 12.0, 16.0],
        ];
        assert!((singular_matrix.determinant() - 0.0).abs() < 1e-14);

        let neg_det_matrix: [[f64; 4]; 4] = [
            [-2.0, -1.0, 3.0, 1.0],
            [1.0, 3.0, -1.0, 4.0],
            [0.0, 2.0, 1.0, -2.0],
            [1.0, -2.0, 3.0, 2.0],
        ];
        assert!((neg_det_matrix.determinant() - (-161.0)).abs() < 1e-14);
    }

    //..........................................................................................//
    //. test_adjugate_2x2()                                                                    .//
    //..........................................................................................//

    #[test]
    fn test_adjugate_2x2_singular() {
        let m = [[1.0, 1.0], [2.0, 2.0]];
        let m_adj_exp = [[2.0, -1.0], [-2.0, 1.0]];
        let m_adj = m.adj();
        for i in 0..2 {
            for j in 0..2 {
                assert!(
                    (m_adj[i][j] - m_adj_exp[i][j]).abs() < 1e-14,
                    "Expected adjugate @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_adj_exp[i][j],
                    m_adj[i][j]
                );
            }
        }
    }

    #[test]
    fn test_adjugate_2x2() {
        let m = [[1.0, 2.0], [0.0, 1.0]];
        let m_adj_exp = [[1.0, -2.0], [0.0, 1.0]];
        let m_adj = m.adj();
        for i in 0..2 {
            for j in 0..2 {
                assert!(
                    (m_adj[i][j] - m_adj_exp[i][j]).abs() < 1e-14,
                    "Expected adjugate @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_adj_exp[i][j],
                    m_adj[i][j]
                );
            }
        }
    }

    //..........................................................................................//
    //. test_adjugate_3x3()                                                                    .//
    //..........................................................................................//

    #[test]
    fn test_adjugate_3x3_singular() {
        let m = [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]];
        let m_adj_exp = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
        let m_adj = m.adj();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (m_adj[i][j] - m_adj_exp[i][j]).abs() < 1e-14,
                    "Expected adjugate @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_adj_exp[i][j],
                    m_adj[i][j]
                );
            }
        }
    }

    #[test]
    fn test_adjugate_3x3() {
        let m = [[1.0, 0.0, 1.0], [0.0, 3.0, 2.0], [2.0, 0.0, 0.0]];
        let m_adj_exp = [[0.0, 0.0, -3.0], [4.0, -2.0, -2.0], [-6.0, 0.0, 3.0]];
        let m_adj = m.adj();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (m_adj[i][j] - m_adj_exp[i][j]).abs() < 1e-14,
                    "Expected adjugate @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_adj_exp[i][j],
                    m_adj[i][j]
                );
            }
        }
    }

    //..........................................................................................//
    //. test_adjugate_4x4()                                                                    .//
    //..........................................................................................//

    #[test]
    fn test_adjugate_4x4_singular() {
        let m = [
            [1.0, 1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0, 2.0],
            [3.0, 3.0, 3.0, 3.0],
            [4.0, 4.0, 4.0, 4.0],
        ];
        let m_adj_exp = [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        let m_adj = m.adj();
        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    (m_adj[i][j] - m_adj_exp[i][j]).abs() < 1e-14,
                    "Expected adjugate @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_adj_exp[i][j],
                    m_adj[i][j]
                );
            }
        }
    }

    #[test]
    fn test_adjugate_4x4() {
        let m = [
            [1.0, 0.0, 1.0, 1.0],
            [2.0, 0.0, 0.0, 1.0],
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 1.0, 2.0, 0.0],
        ];
        let m_adj_exp = [
            [1.0, -5.0, 1.0, -2.0],
            [14.0, -6.0, -2.0, -4.0],
            [-7.0, 3.0, 1.0, -2.0],
            [-2.0, 2.0, -2.0, 4.0],
        ];
        let m_adj = m.adj();
        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    (m_adj[i][j] - m_adj_exp[i][j]).abs() < 1e-14,
                    "Expected adjugate @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_adj_exp[i][j],
                    m_adj[i][j]
                );
            }
        }
    }

    //..........................................................................................//
    //. test_inverse_2x2()                                                                     .//
    //..........................................................................................//

    #[test]
    fn test_inverse_2x2_singular() {
        let m = [[1.0, 1.0], [2.0, 2.0]];
        let m_inv = m.inv();
        assert!(m_inv.is_none());
    }

    #[test]
    fn test_inverse_2x2() {
        let m = [[1.0, 2.0], [0.0, 1.0]];
        let m_inv_exp = [[1.0, -2.0], [0.0, 1.0]];
        let m_inv = m.inv().unwrap();
        for i in 0..2 {
            for j in 0..2 {
                assert!(
                    (m_inv[i][j] - m_inv_exp[i][j]).abs() < 1e-14,
                    "Expected inverse @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_inv_exp[i][j],
                    m_inv[i][j]
                )
            }
        }
    }

    //..........................................................................................//
    //. test_inverse_3x3()                                                                     .//
    //..........................................................................................//

    #[test]
    fn test_inverse_3x3_singular() {
        let m = [[1.0, 1.0, 1.0], [2.0, 2.0, 2.0], [3.0, 3.0, 3.0]];
        let m_inv = m.inv();
        assert!(m_inv.is_none());
    }

    #[test]
    fn test_inverse_3x3() {
        let m = [[1.0, 0.0, 1.0], [0.0, 3.0, 2.0], [2.0, 0.0, 0.0]];
        let m_inv_exp = [
            [0.0, 0.0, 1.0 / 2.0],
            [-2.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0],
            [1.0, 0.0, -1.0 / 2.0],
        ];
        let m_inv = m.inv().unwrap();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (m_inv[i][j] - m_inv_exp[i][j]).abs() < 1e-14,
                    "Expected inverse @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_inv_exp[i][j],
                    m_inv[i][j]
                )
            }
        }
    }

    //..........................................................................................//
    //. test_inverse_4x4()                                                                     .//
    //..........................................................................................//

    #[test]
    fn test_inverse_4x4_singular() {
        let m = [
            [1.0, 1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0, 2.0],
            [3.0, 3.0, 3.0, 2.0],
            [4.0, 4.0, 4.0, 4.0],
        ];
        let m_inv = m.inv();
        assert!(m_inv.is_none());
    }

    #[test]
    fn test_inverse_4x4() {
        let m = [
            [1.0, 0.0, 1.0, 1.0],
            [2.0, 0.0, 0.0, 1.0],
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 1.0, 2.0, 0.0],
        ];
        let m_inv_exp = [
            [-1.0 / 8.0, 5.0 / 8.0, -1.0 / 8.0, 1.0 / 4.0],
            [-7.0 / 4.0, 3.0 / 4.0, 1.0 / 4.0, 1.0 / 2.0],
            [7.0 / 8.0, -3.0 / 8.0, -1.0 / 8.0, 1.0 / 4.0],
            [1.0 / 4.0, -1.0 / 4.0, 1.0 / 4.0, -1.0 / 2.0],
        ];
        let m_inv = m.inv().unwrap();
        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    (m_inv[i][j] - m_inv_exp[i][j]).abs() < 1e-14,
                    "Expected inverse @ {}, {} to be {} but was {}",
                    i,
                    j,
                    m_inv_exp[i][j],
                    m_inv[i][j]
                )
            }
        }
    }
}
