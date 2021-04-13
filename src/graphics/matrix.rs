use std::{fmt, ops::*, slice};

#[derive(Default, Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

// pub struct Matrix<const rows:usize, const cols:usize> {
//     data: Vec<f64>, not going to do generics
// }

#[allow(dead_code)]
impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len(), "Matrix must be filled completely");
        Self { rows, cols, data }
    }

    pub fn identity_matrix(size: usize) -> Self {
        let mut matrix: Matrix = Matrix::new(size, size, vec![0.0; size * size]);
        for i in 0..size {
            matrix.set(i, i, 1.0);
        }
        matrix
    }

    pub fn transpose(&self) -> Matrix {
        Matrix::new(self.cols, self.rows, self.data.clone())
    }

    pub fn to_identity(&mut self) {
        assert_eq!(self.rows, self.cols, "An identity matrix must be N x N");
        let cols = self.cols;
        // self.iter_mut()
        //     .enumerate()
        //     .map(|(i, d)| *d = if i / cols == i % cols { 1.0 } else { 0.0 });
        for (i, d) in self.iter_mut().enumerate() {
            *d = if i / cols == i % cols { 1.0 } else { 0.0 }
        }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        if self.rows >= self.cols {
            row * self.cols + col
        } else {
            col * self.rows + row
        }
    }

    pub fn fill(&mut self, n: f64) {
        self.data = vec![n; self.rows * self.cols]
    }

    pub fn swap_rows(&mut self, row_one: usize, row_two: usize) {
        let mut points = self.iter_by_point_mut();
        points
            .nth(row_one)
            .unwrap()
            .swap_with_slice(points.nth(row_two - row_one - 1).unwrap());
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        assert!(row < self.rows && col < self.cols, "Index out of bound");
        self.data[self.index(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, new_point: f64) {
        assert!(row < self.rows && col < self.cols, "Index out of bound");
        let i = self.index(row, col);
        self.data[i] = new_point;
    }

    // void matrix_mult(struct matrix *a, struct matrix *b);
    // void add_ege(struct matrix *a, );
}

// Iterator stuff
#[allow(dead_code)]
impl Matrix {
    pub fn for_each<F>(&mut self, function: F)
    where
        F: Fn(f64) -> f64,
    {
        self.iter_by_point_mut()
            .for_each(|point: &mut [f64]| point.iter_mut().for_each(|e| *e = function(*e)))
    }

    // pub fn from_iter(&self) -> impl IntoIterator<Item = &[f64]> {
    //     self.data.chunks_exact(self.cols)
    // }

    fn iter(&self) -> impl Iterator<Item = &f64> + '_ {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> + '_ {
        self.data.iter_mut()
    }

    pub fn iter_row(&self, r: usize) -> impl Iterator<Item = &f64> + '_ {
        let start = r * self.cols;
        self.data[start..self.cols + start].iter()
    }

    fn iter_row_mut(&mut self, r: usize) -> impl Iterator<Item = &mut f64> + '_ {
        let start = r * self.cols;
        self.data[start..self.cols + start].iter_mut()
    }

    pub fn iter_col(&self, c: usize) -> impl Iterator<Item = &f64> + '_ {
        self.iter().skip(c).step_by(self.cols)
    }

    pub fn iter_col_mut(&mut self, c: usize) -> impl Iterator<Item = &mut f64> + '_ {
        let col = self.cols;
        self.iter_mut().skip(c).step_by(col)
    }

    pub fn iter_by_point(&self) -> slice::ChunksExact<'_, f64> {
        self.data.chunks_exact(self.cols)
    }

    pub fn iter_by_point_mut(&mut self) -> slice::ChunksExactMut<'_, f64> {
        self.data.chunks_exact_mut(self.cols)
    }
}

// operators

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
            && self.cols == other.cols
            && self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// add + append
#[allow(dead_code)]
impl Matrix {
    // TODO: add z //
    pub fn add_point(&mut self, x: f64, y: f64) {
        self.data.push(x);
        self.data.push(y);
        self.data.push(0.0);
        self.data.push(1.0);
        self.rows += 1;
    }

    // add z, + 1
    pub fn add_edge(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        self.add_point(x0, y0);
        self.add_point(x1, y1);
    }

    pub fn append_row(&mut self, row: &mut Vec<f64>) {
        assert_eq!(
            self.cols,
            row.len() + 2,
            "self.cols and edge len are not equal"
        );
        // I don't implment z or 1 - for now
        row.push(0.0);
        row.push(1.0);
        self.data.append(row);
        self.rows += 1;
    }
}

// operators
impl Add for Matrix {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        assert!(
            self.cols == other.cols && self.rows == other.rows,
            "To add Matices must be the same size"
        );
        let new_data = self.iter().zip(other.iter()).map(|(a, b)| a + b).collect();
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Self) {
        assert!(
            self.cols == other.cols && self.rows == other.rows,
            "To add Matices must be the same size"
        );
        self.iter_mut().zip(other.iter()).for_each(|(a, b)| *a += b)
    }
}

impl AddAssign<f64> for Matrix {
    fn add_assign(&mut self, other: f64) {
        self.iter_mut().for_each(|e| *e += other)
    }
}

// change to three
impl AddAssign<[f64; 2]> for Matrix {
    fn add_assign(&mut self, other: [f64; 2]) {
        self.add_point(other[0], other[1])
    }
}

impl AddAssign<Vec<f64>> for Matrix {
    fn add_assign(&mut self, mut other: Vec<f64>) {
        self.append_row(&mut other)
    }
}

impl Sub for Matrix {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        assert!(
            self.cols == other.cols && self.rows == other.rows,
            "To subtract Matices must be the same size"
        );
        let new_data = self.iter().zip(other.iter()).map(|(a, b)| a - b).collect();
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, other: Self) {
        assert!(
            self.cols == other.cols && self.rows == other.rows,
            "To subtract Matices must be the same size"
        );
        self.iter_mut().zip(other.iter()).for_each(|(a, b)| *a -= b)
    }
}

impl SubAssign<f64> for Matrix {
    fn sub_assign(&mut self, other: f64) {
        self.iter_mut().for_each(|e| *e += other)
    }
}

//impl<const M: usize, const N: usize> ops::AddAssign<f32> for Matrix<M, N> {
// fn add_assign(&mut self, other: f32) {
//     self.body.iter_mut().for_each(|row| row.iter_mut().for_each(|e| *e += other));
// }
// }

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}\t", self.data[self.index(col, row)])?;
            }
            writeln!(f)?;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_matrix() {
        let nums: Vec<f64> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let edge: Matrix = Matrix::new(3, 3, nums);
        // println!("{}", edge);
        let ident = Matrix::identity_matrix(3);
        let bruh = edge.transpose();
        println!("{}", ident);
        println!("{:?}", ident);
        assert!(ident != bruh, "Not Equal")
        // assert_eq!(edge.data, bruh.data)
    }

    #[test]
    fn rows_and_cols() {
        let nums: Vec<f64> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let edge: Matrix = Matrix::new(3, 3, nums.clone());
        let row_one: Vec<f64> = edge.iter_row(1).copied().collect();
        let mut points = edge.iter_by_point();
        println!("{:?}", points.next());
        println!("{:?}", points.next());
        assert_eq!(row_one, nums[3..6])
    }

    #[test]
    fn swap() {
        let mut ident = Matrix::identity_matrix(3);
        ident.swap_rows(0, 2);
        // println!("ident:\n{}", ident);
        // println!("test:\n{}", test);
        assert_eq!(
            ident,
            Matrix::new(3, 3, vec![0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0])
        )
    }

    #[test]
    // #[should_panic]
    fn iterators() {
        let ident = Matrix::identity_matrix(4);
        let src: Vec<&[f64]> = ident.iter_by_point().collect();
        // println!("{:?}", src);
        println!("{:?}", src);
        assert_eq!(5 / ident.rows, 5 % ident.cols)
        // let src: Vec<String> = ident.iter().map(|x| format!("{}", x)).collect();
        // println!("{:?}",src)
        // src.map(|x| println!("{}", x))
        // let src = vec![1, 2, 3];
        // let mut dest: Vec<String> = src.iter()
    }

    #[test]
    #[should_panic]
    fn operators() {
        let mut matrix = Matrix::identity_matrix(2);
        let bruh = Matrix {
            rows: 2,
            cols: 2,
            data: [2.0, 0.0, 0.0, 2.0].to_vec(),
        };
        // let bruh = Matrix::identity_matrix(2);
        println!("{:?}", matrix);
        matrix -= bruh;
        println!("{:?}", matrix);
        // let new_matrix = matrix + bruh;
        // println!("{:?}", new_matrix);
        let test = Matrix {
            rows: 2,
            cols: 2,
            data: [2.0, 0.0, 0.0, 2.0].to_vec(),
        };
        assert_eq!(matrix, test);
    }

    #[test]
    // #[should_panic]
    fn add_points() {
        let mut matrix = Matrix::new(0, 4, Vec::with_capacity(8));
        let x = [0.0, 0.1];
        let y = vec![0.2, 0.3];
        matrix += x;
        matrix += y;
        println!("{}", matrix)
    }
}
