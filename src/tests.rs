use super::*; //bring everything from parent module

#[test]

fn square_works() {
    let z = Complex { re: 1.0, im: 2.0 };

    let s = z.square();
    assert_eq!(s.re, -3.0);
    assert_eq!(s.im, 4.0);
}

#[test]
fn add_works() {
    let z = Complex { re: 1.0, im: 2.0 };
    let s = z.add(z);
    assert_eq!(s.re, 2.0);
    assert_eq!(s.im, 4.0);
}

#[test]
fn norm_sqr_works() {
    let z = Complex { re: 1.0, im: 2.0 };
    assert_eq!(z.norm_sqr(), 5.0);
}
