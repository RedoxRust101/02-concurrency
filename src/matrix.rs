use anyhow::{anyhow, Result};
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
};

// [[1, 2], [3, 4], [5, 6]] => [1, 2, 3, 4, 5, 6]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix multiply error: a.cols != b.rows"));
    }
    let mut data = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }
    Ok(Matrix {
        rows: a.rows,
        cols: b.cols,
        data,
    })
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: impl Into<Vec<T>>) -> Self {
        Self {
            rows,
            cols,
            data: data.into(),
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "{}", if i == 0 { "{" } else { " " })?;
            for j in 0..self.cols {
                write!(
                    f,
                    "{}{}",
                    if j == 0 { "" } else { " " },
                    self.data[i * self.cols + j]
                )?;
            }
            write!(f, "{}", if i == self.rows - 1 { "}" } else { "," })?;
        }
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix[{}x{}]: {}", self.rows, self.cols, self)
    }
}

#[cfg(test)]
mod tests {
    // test matrix multiply
    #[test]
    fn test_matrix_multiply() {
        use super::*;
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
        let c = multiply(&a, &b).unwrap();
        assert_eq!(format!("{:?}", c), "Matrix[2x2]: {22 28, 49 64}");
    }

    // test matrix display
    #[test]
    fn test_matrix_display() {
        use super::*;
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(format!("{}", a), "{1 2 3, 4 5 6}");
    }
}
