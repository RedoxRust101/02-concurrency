use crate::{dot_product, Vector};
use anyhow::{anyhow, Result};
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_THREADS: usize = 4;

// [[1, 2], [3, 4], [5, 6]] => [1, 2, 3, 4, 5, 6]
pub struct Matrix<T> {
    pub row: usize,
    pub col: usize,
    pub data: Vec<T>,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    // sender to send the result back
    sender: oneshot::Sender<MsgOutput<T>>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.cols != b.rows"));
    }
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprint!("Sender error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // generate 4 threads which receive msg and do dot product
    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    // map/reduce: map phase
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col: Vector<T> = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(rx);
        }
    }

    // map/reduce:reduce
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }

    Ok(Matrix {
        row: a.row,
        col: b.col,
        data,
    })
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(row: usize, col: usize, data: impl Into<Vec<T>>) -> Self {
        Self {
            row,
            col,
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
        for i in 0..self.row {
            write!(f, "{}", if i == 0 { "{" } else { " " })?;
            for j in 0..self.col {
                write!(
                    f,
                    "{}{}",
                    if j == 0 { "" } else { " " },
                    self.data[i * self.col + j]
                )?;
            }
            write!(f, "{}", if i == self.row - 1 { "}" } else { "," })?;
        }
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix[{}x{}]: {}", self.row, self.col, self)
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiply error")
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

    // test a can not multiply b
    #[test]
    fn test_matrix_multiply_error() {
        use super::*;
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let c = multiply(&a, &b);
        assert!(c.is_err());
    }

    // test a can not multiply b panic
    #[test]
    #[should_panic]
    fn test_matrix_multiply_error_panic() {
        use super::*;
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let _c = a * b;
    }
}
