extern crate num;
use num::Complex;

fn main() {
    complex_square_add_loop(Complex {re: 0.5, im: 0.1});
}


#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    println!("{}", c);
    let mut z = Complex {re: 0.0, im: 0.0};
    loop {
        z = z * z + c;
    }
}