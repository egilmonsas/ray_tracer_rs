use crate::tuple::VTuple;
use crate::zequality::ZEq;
use num_traits::Float;
use std::convert::From;
use std::ops::{self, AddAssign};

type Matrix4f = VMatrix<f64, 4>;
type Matrix3f = VMatrix<f64, 3>;
type Matrix2f = VMatrix<f64, 2>;

/*
_______________________________________ DxD generics _____________________________________________
*/
#[derive(Debug, Clone, Copy)]
pub struct VMatrix<T, const D: usize>
where
    T: Float,
{
    data: [[T; D]; D],
}
impl<T, const D: usize> From<[[T; D]; D]> for VMatrix<T, D>
where
    T: Float,
{
    fn from(data: [[T; D]; D]) -> Self {
        VMatrix { data }
    }
}
impl<T, const D: usize> VMatrix<T, D>
where
    T: Float,
{
    pub fn default() -> VMatrix<T, D> {
        VMatrix::from([[T::zero(); D]; D])
    }
}
impl<T, const D: usize> ops::Index<usize> for VMatrix<T, D>
where
    T: Float,
{
    type Output = [T; D];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T, const D: usize> ops::IndexMut<usize> for VMatrix<T, D>
where
    T: Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl<T, const D: usize> ops::Mul<VMatrix<T, D>> for VMatrix<T, D>
where
    T: Float,
    T: AddAssign,
{
    type Output = VMatrix<T, D>;
    fn mul(self, other: VMatrix<T, D>) -> Self::Output {
        let mut tgt: VMatrix<T, D> = VMatrix::default();
        for row in 0..D {
            for col in 0..D {
                for i in 0..D {
                    tgt[row][col] += self[row][i] * other[i][col]
                }
            }
        }
        tgt
    }
}
impl<T, const D: usize> ZEq<Self> for VMatrix<T, D>
where
    T: Float,
    T: ZEq<T>,
{
    fn zeq(&self, other: VMatrix<T, D>) -> bool {
        for i in 0..D {
            for j in 0..D {
                if !self[i][j].zeq(other[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }
}
/*
_______________________________________ 4x4 specifics _____________________________________________
*/
impl<T> VMatrix<T, 4>
where
    T: Float,
    T: AddAssign,
{
    pub fn identity() -> VMatrix<T, 4> {
        let mut result: VMatrix<T, 4> = VMatrix::default();
        for i in 0..4 {
            result[i][i] = T::one();
        }
        result
    }
    pub fn transpose(&mut self) {
        let copy = self.clone();
        for i in 0..4 {
            for j in 0..4 {
                self[i][j] = copy[j][i];
            }
        }
    }
    pub fn transposed(self) -> VMatrix<T, 4> {
        let mut result: VMatrix<T, 4> = VMatrix::default();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[j][i];
            }
        }
        result
    }
    pub fn is_invertable(self) -> bool {
        self.determinant().zneg(T::zero())
    }
    pub fn invert(&mut self) {
        if !self.is_invertable() {
            panic!("Matrix is not invertible, but inversion was called")
        }
        let copy = self.clone();

        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let cofactor: T = copy.cofactor(row, col);
                self[col][row] = cofactor / determinant; //stores transposed
            }
        }
    }
    pub fn inverted(self) -> VMatrix<T, 4> {
        if !self.is_invertable() {
            panic!("Matrix is not invertible, but inversion was called")
        }

        let mut tgt: VMatrix<T, 4> = VMatrix::default();
        let determinant: T = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                let cofactor: T = self.cofactor(row, col);
                tgt[col][row] = T::from(cofactor / determinant).unwrap(); //stores transposed
            }
        }
        tgt
    }
    pub fn submatrix(self, rmv_row: usize, rmv_col: usize) -> VMatrix<T, 3> {
        // @FIXME: Find a nicer algorithm for this garbage
        let mut tgt: VMatrix<T, 3> = VMatrix::default();

        let mut src_row: usize = 0;
        let mut src_col: usize = 0;
        let mut tgt_row: usize = 0;
        let mut tgt_col: usize = 0;

        while tgt_row < 3 {
            //reset src col index
            if src_row == rmv_row {
                //Skip remove row
                src_row += 1;
            }
            while tgt_col < 3 {
                if src_col == rmv_col {
                    //Skip remove col
                    src_col += 1;
                }
                tgt[tgt_row][tgt_col] = self[src_row][src_col];
                src_col += 1;
                tgt_col += 1;
            }
            src_row += 1;
            src_col = 0;
            tgt_row += 1;
            tgt_col = 0;
        }
        tgt
    }
    pub fn minor(self, rmv_row: usize, rmv_col: usize) -> T {
        T::from(self.submatrix(rmv_row, rmv_col).determinant()).unwrap()
    }
    pub fn cofactor(self, rmv_row: usize, rmv_col: usize) -> T {
        let minor = self.minor(rmv_row, rmv_col);
        let rmd: T = T::from(((rmv_col + rmv_row) % 2) as f64).unwrap();
        let sign: T = T::from(1.0 - 2.0 * rmd.to_f64().unwrap()).unwrap();
        return minor * sign;
    }
    pub fn determinant(self) -> T {
        let mut determinant: T = T::zero();

        for col in 0..4 {
            determinant = determinant + self[0][col] * self.cofactor(0, col);
        }
        determinant
    }
}

impl<T> ops::Mul<VTuple<T>> for VMatrix<T, 4>
where
    T: Float,
{
    type Output = VTuple<T>;
    fn mul(self, other: VTuple<T>) -> Self::Output {
        VTuple::new(
            self[0][0] * other.x
                + self[0][1] * other.y
                + self[0][2] * other.z
                + self[0][3] * other.w,
            self[1][0] * other.x
                + self[1][1] * other.y
                + self[1][2] * other.z
                + self[1][3] * other.w,
            self[2][0] * other.x
                + self[2][1] * other.y
                + self[2][2] * other.z
                + self[2][3] * other.w,
            self[3][0] * other.x
                + self[3][1] * other.y
                + self[3][2] * other.z
                + self[3][3] * other.w,
        )
    }
}
/*
_______________________________________ 3x3 specifics _____________________________________________
*/
impl<T> VMatrix<T, 3>
where
    T: Float,
    T: AddAssign,
{
    pub fn submatrix(self, rmv_row: usize, rmv_col: usize) -> VMatrix<T, 2> {
        // @FIXME: Find a nicer algorithm for this garbage
        let mut tgt: VMatrix<T, 2> = VMatrix::default();

        let mut src_row: usize = 0;
        let mut src_col: usize = 0;
        let mut tgt_row: usize = 0;
        let mut tgt_col: usize = 0;

        while tgt_row < 2 {
            //reset src col index
            if src_row == rmv_row {
                //Skip remove row
                src_row += 1;
            }
            while tgt_col < 2 {
                if src_col == rmv_col {
                    //Skip remove col
                    src_col += 1;
                }
                tgt[tgt_row][tgt_col] = self[src_row][src_col];
                src_col += 1;
                tgt_col += 1;
            }
            src_row += 1;
            src_col = 0;
            tgt_row += 1;
            tgt_col = 0;
        }
        tgt
    }
    pub fn minor(self, rmv_row: usize, rmv_col: usize) -> T {
        self.submatrix(rmv_row, rmv_col).determinant()
    }
    pub fn cofactor(self, rmv_row: usize, rmv_col: usize) -> T {
        let minor = self.minor(rmv_row, rmv_col);
        let rmd = (rmv_col + rmv_row) % 2;
        let sign = T::from(1.0 - 2.0 * rmd as f64).unwrap();
        return minor * sign;
    }
    pub fn determinant(self) -> T {
        let mut determinant: T = T::zero();

        for col in 0..3 {
            determinant += self[0][col] * self.cofactor(0, col);
        }
        determinant
    }
}
/*
_______________________________________ 2x2 specifics _____________________________________________
*/
impl<T> VMatrix<T, 2>
where
    T: Float,
{
    pub fn determinant(self) -> T {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}

#[cfg(test)]
mod test {
    use crate::tuple::VTuple;

    use super::*;

    #[test]
    fn construcing_and_inspecting_a_4x4_matrix() {
        let m: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[0][2], 3.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[1][3], 8.5);
        assert_eq!(m[2][0], 9.0);
        assert_eq!(m[2][1], 10.0);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[2][3], 12.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][1], 14.4);
        assert_eq!(m[3][2], 15.5);
        assert_eq!(m[3][3], 16.5);
    }
    #[test]
    fn construcing_and_inspecting_a_3x3_matrix() {
        let m: Matrix3f = Matrix3f::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[0][2], 3.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][0], 9.0);
        assert_eq!(m[2][1], 10.0);
        assert_eq!(m[2][2], 11.0);
    }
    #[test]
    fn construcing_and_inspecting_a_2x2_matrix() {
        let m = VMatrix::from([[1.0, 2.0], [5.5, 6.5]]);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
    }
    #[test]
    fn matrix_equality_with_identical_4x4_matrices() {
        let m1: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        let m2: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_equality_with_identical_3x3_matrices() {
        let m1: Matrix3f = Matrix3f::from([[2.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        let m2: Matrix3f = Matrix3f::from([[2.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_equality_with_identical_2x2_matrices() {
        let m1: Matrix2f = Matrix2f::from([[1.0, 2.0], [5.5, 6.5]]);
        let m2: Matrix2f = Matrix2f::from([[1.0, 2.0], [5.5, 6.5]]);
        assert_zeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_4x4_matrices() {
        let m1: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        let m2: Matrix4f = Matrix4f::from([
            [2.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.4, 15.5, 16.5],
        ]);
        assert_nzeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_3x3_matrices() {
        let m1: Matrix3f = Matrix3f::from([[1.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        let m2: Matrix3f = Matrix3f::from([[2.0, 2.0, 3.0], [5.5, 6.5, 7.5], [9.0, 10.0, 11.0]]);
        assert_nzeq!(m1, m2)
    }
    #[test]
    fn matrix_inequality_with_identical_2x2_matrices() {
        let m1: Matrix2f = Matrix2f::from([[1.0, 2.0], [5.5, 6.5]]);
        let m2: Matrix2f = Matrix2f::from([[2.0, 2.0], [5.5, 1.5]]);
        assert_nzeq!(m1, m2)
    }
    #[test]
    fn multiplying_two_4x4_matrices() {
        let m1: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix4f = Matrix4f::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let result: Matrix4f = m1 * m2;
        let expected_result: Matrix4f = Matrix4f::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn multiplying_4x4_matrix_by_the_identity_matrix() {
        let m1: Matrix4f = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2: Matrix4f = Matrix4f::identity();

        let result: Matrix4f = m1 * m2;
        let expected_result: Matrix4f = m1;

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn multiplying_4x4_matrix_by_point() {
        let m = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let v = VTuple::point(1.0, 2.0, 3.0);

        let result = m * v;
        let expected_result = VTuple::point(18.0, 24.0, 33.0);

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn transpose_a_4x4_matrix() {
        let mut m = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        m.transpose();

        let expected_result = Matrix4f::from([
            [1.0, 2.0, 8.0, 0.0],
            [2.0, 4.0, 6.0, 0.0],
            [3.0, 4.0, 4.0, 0.0],
            [4.0, 2.0, 1.0, 1.0],
        ]);
        assert_zeq!(m, expected_result)
    }
    #[test]
    fn transposed_mirror_a_4x4_matrix() {
        let m = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let result = m.transposed();
        let expected_result = Matrix4f::from([
            [1.0, 2.0, 8.0, 0.0],
            [2.0, 4.0, 6.0, 0.0],
            [3.0, 4.0, 4.0, 0.0],
            [4.0, 2.0, 1.0, 1.0],
        ]);

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn determinant_of_2x2_matrix() {
        let m = Matrix2f::from([[1.0, 2.0], [3.0, 4.0]]);

        let result = m.determinant();
        let expected_result = -2.0;

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn submatrix_of_4x4_matrix_is_a_3x3_matrix() {
        let m = Matrix4f::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let result = m.submatrix(0, 3);
        let expected_result = Matrix3f::from([[2.0, 4.0, 4.0], [8.0, 6.0, 4.0], [0.0, 0.0, 0.0]]);

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn submatrix_of_3x3_matrix_is_a_2x2_matrix() {
        let m = Matrix3f::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, 3.0]]);

        let result = m.submatrix(0, 2);
        let expected_result = Matrix2f::from([[-3.0, 2.0], [0.0, 6.0]]);

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn calculate_minor_of_a_3x3_matrix() {
        let m = Matrix3f::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let sub = m.submatrix(1, 0);
        let det = sub.determinant();
        let minor = m.minor(1, 0);

        assert_zeq!(det, 25.0);
        assert_zeq!(minor, 25.0)
    }
    #[test]
    fn calculating_the_cofactor_of_a_3x3_matrix() {
        let m = Matrix3f::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let minor1 = m.minor(0, 0);
        let minor2 = m.minor(1, 0);

        let cofactor1 = m.cofactor(0, 0);
        let cofactor2 = m.cofactor(1, 0);

        assert_zeq!(-12.0, minor1);
        assert_zeq!(-12.0, cofactor1);
        assert_zeq!(25.0, minor2);
        assert_zeq!(-25.0, cofactor2);
    }
    #[test]
    fn calculating_determinant_of_a_3x3_matrix() {
        let m = VMatrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        let cofactor00 = m.cofactor(0, 0);
        let cofactor01 = m.cofactor(0, 1);
        let cofactor02 = m.cofactor(0, 2);
        let determinant = m.determinant();

        assert_zeq!(56.0, cofactor00);
        assert_zeq!(12.0, cofactor01);
        assert_zeq!(-46.0, cofactor02);
        assert_zeq!(-196.0, determinant);
    }
    #[test]
    fn calculating_determinant_of_a_4x4_matrix() {
        let m = Matrix4f::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        let cofactor00 = m.cofactor(0, 0);
        let cofactor01 = m.cofactor(0, 1);
        let cofactor02 = m.cofactor(0, 2);
        let cofactor03 = m.cofactor(0, 3);
        let determinant = m.determinant();

        assert_zeq!(690.0, cofactor00);
        assert_zeq!(447.0, cofactor01);
        assert_zeq!(210.0, cofactor02);
        assert_zeq!(51.0, cofactor03);
        assert_zeq!(-4071.0, determinant);
    }
    #[test]
    fn testing_an_invertable_4x4_matrix_for_invertability() {
        let m = Matrix4f::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert_zeq!(m.determinant(), -2120.0);
        assert!(m.is_invertable());
    }
    #[test]
    fn testing_an_non_invertable_4x4_matrix_for_non_invertability() {
        let m = Matrix4f::from([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_zeq!(m.determinant(), 0.0);
        assert!(!m.is_invertable());
    }
    #[test]
    fn calculating_the_inverse_of_4x4_matrix() {
        let mut m = Matrix4f::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let determinant = m.determinant();
        let cofactor23 = m.cofactor(2, 3);
        let cofactor32 = m.cofactor(3, 2);

        let inverse = m.inverted();
        m.invert();

        let expected_result = Matrix4f::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_zeq!(532.0, determinant);
        assert_zeq!(-160.0, cofactor23);
        assert_zeq!(-160.0 / 532.0, inverse[3][2]);
        assert_zeq!(105.0, cofactor32);
        assert_zeq!(105.0 / 532.0, inverse[2][3]);

        assert_zeq!(inverse, m);
        assert_zeq!(inverse, expected_result);
    }
    #[test]
    fn multiplying_products_by_its_inverse_yields_identity_matrix() {
        let m = Matrix4f::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        assert_zeq!(m.inverted() * m, Matrix4f::identity())
    }
}
