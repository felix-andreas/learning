use ndarray::prelude::*;

fn main() {
    let n = 1_000;
    let x: Array2<f64> = Array::linspace(0., 1., n * n).into_shape((n, n)).unwrap();

    println!("{}", x.dot(&x).sum());
}
