use std::fmt;
use std::ops::*;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

trait AddInteger<T> {
    fn add(self, rhs: T) -> Complex;
}

impl Complex {
    fn new<T, K>(real: T, imag: K) -> Complex
    where
        f64: From<T>,
        f64: From<K>,
    {
        Complex {
            real: f64::from(real),
            imag: f64::from(imag),
        }
    }

    fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: (-1.0) * self.imag,
        }
    }
}

impl From<i32> for Complex {
    fn from(value: i32) -> Complex {
        Complex {
            real: value as f64,
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Complex {
        Complex {
            real: value,
            imag: 0.0,
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Add<i32> for Complex {
    type Output = Complex;

    fn add(self, rhs: i32) -> Self::Output {
        self + Complex::from(rhs)
    }
}

impl Add<f64> for Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        self + Complex::from(rhs)
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Sub<i32> for Complex {
    type Output = Complex;

    fn sub(self, rhs: i32) -> Self::Output {
        self - Complex::from(rhs)
    }
}

impl Sub<f64> for Complex {
    type Output = Complex;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Complex::from(rhs)
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl Mul<i32> for Complex {
    type Output = Complex;

    fn mul(self, rhs: i32) -> Self::Output {
        let rhs_complex = Complex::from(rhs);
        Complex {
            real: self.real * rhs_complex.real - self.imag * rhs_complex.imag,
            imag: self.real * rhs_complex.imag + self.imag * rhs_complex.real,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs_complex = Complex::from(rhs);
        Complex {
            real: self.real * rhs_complex.real - self.imag * rhs_complex.imag,
            imag: self.real * rhs_complex.imag + self.imag * rhs_complex.real,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            real: (-1.0) * self.real,
            imag: (-1.0) * self.imag,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.real == 0.0 && self.imag == 0.0 {
            write!(f, "0")?;
        } else if self.real != 0.0 {
            write!(f, "{}", self.real)?;
        }

        if self.imag != 0.0 {
            let op = if self.imag < 0.0 {
                ""
            } else if self.real != 0.0 {
                "+"
            } else {
                ""
            };

            write!(f, "{}{}i", op, self.imag)?;
        }

        Ok(())
    }
}

//---------------------------------------------bonus
impl AddAssign<Complex> for Complex {
    fn add_assign(&mut self, rhs: Complex) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl AddAssign<i32> for Complex {
    fn add_assign(&mut self, rhs: i32) {
        let rhs_complex = Complex::from(rhs);
        self.real += rhs_complex.real;
        self.imag += rhs_complex.imag;
    }
}

impl AddAssign<f64> for Complex {
    fn add_assign(&mut self, rhs: f64) {
        let rhs_complex = Complex::from(rhs);
        self.real += rhs_complex.real;
        self.imag += rhs_complex.imag;
    }
}

impl SubAssign<Complex> for Complex {
    fn sub_assign(&mut self, rhs: Complex) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}

impl SubAssign<i32> for Complex {
    fn sub_assign(&mut self, rhs: i32) {
        let rhs_complex = Complex::from(rhs);
        self.real -= rhs_complex.real;
        self.imag -= rhs_complex.imag;
    }
}

impl SubAssign<f64> for Complex {
    fn sub_assign(&mut self, rhs: f64) {
        let rhs_complex = Complex::from(rhs);
        self.real -= rhs_complex.real;
        self.imag -= rhs_complex.imag;
    }
}

impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        let new_real = self.real * rhs.real - self.imag * rhs.imag;
        let new_imag = self.real * rhs.imag + self.imag * rhs.real;

        self.real = new_real;
        self.imag = new_imag;
    }
}

impl MulAssign<i32> for Complex {
    fn mul_assign(&mut self, rhs: i32) {
        let rhs_complex = Complex::from(rhs);

        let new_real = self.real * rhs_complex.real - self.imag * rhs_complex.imag;
        let new_imag = self.real * rhs_complex.imag + self.imag * rhs_complex.real;

        self.real = new_real;
        self.imag = new_imag;
    }
}

impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, rhs: f64) {
        let rhs_complex = Complex::from(rhs);

        let new_real = self.real * rhs_complex.real - self.imag * rhs_complex.imag;
        let new_imag = self.real * rhs_complex.imag + self.imag * rhs_complex.real;

        self.real = new_real;
        self.imag = new_imag;
    }
}

//----------------------------------------------------

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    //  testing bonus

    //  let mut k = Complex::new(1, 5);
    //  let l = Complex::new(1, 3);
    //  k += l + 2;
    
    //  assert_eq_rel!(k.real, 4);
    //  assert_eq_rel!(k.imag, 8);

    println!("ok!");
}
