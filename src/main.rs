#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    im: f64,
}

fn main() {
    let c = Complex { real: 3.0, im: 2.0 };
    println!("{:#?}", c);
}
