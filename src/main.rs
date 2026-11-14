#[derive(Debug, Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    //cartesian form (a+bi), z^2 = (a^2 - b^2) + (2ab)i
    fn square(self) -> Complex {
        Complex {
            re: self.re * self.re - self.im * self.im,
            im: 2.0 * self.re * self.im,
        }
    }

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

fn escape_time(c: Complex, max_iter: u32) -> Option<u32> {
    //Maybe a value, either Some(number) or None
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..max_iter {
        if z.norm_sqr() > 4.0 {
            return Some(i); //exit function early, escaped the set
        }
        z = z.square().add(c);
    }
    None //did not exit function so is in the mandelbrt set
}

fn main() {}

#[cfg(test)] // only compile this when running cargo test
mod tests;
