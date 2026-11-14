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

#[test]
fn origin_is_in_set() {
    let c = Complex { re: 0.0, im: 0.0 };
    assert_eq!(escape_time(c, 100), None);
}

#[test]
fn two_escapes_fast() {
    let c = Complex { re: 2.0, im: 0.0 };
    assert_eq!(escape_time(c, 100), Some(2));
}

#[test]
fn three_escapes_one() {
    let c = Complex { re: 3.0, im: 0.0 };
    assert_eq!(escape_time(c, 100), Some(1));
}
