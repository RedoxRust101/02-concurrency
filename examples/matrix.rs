use anyhow::{Ok, Result};
use concurrency::Matrix;
fn main() -> Result<()> {
    let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
    let b = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
    println!("a:{}", a);
    println!("b:{}", b);
    println!("a*b: {:?}", a * b);
    Ok(())
}
