use anyhow::{Ok, Result};
use concurrency::{multiply, Matrix};
fn main() -> Result<()> {
    let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
    let b = Matrix::new(3, 2, vec![1, 2, 3, 4, 5, 6]);
    let c = multiply(&a, &b).unwrap();
    println!("a:{}", a);
    println!("b:{}", b);
    println!("c:{:?}", c);
    Ok(())
}
