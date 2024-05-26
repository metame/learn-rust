#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        if n > 1. {
            Q7(i8::MAX)
        } else if n < -1. {
            Q7(i8::MIN)
        } else {
            Q7((n * 128.) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> Self {
        (n.0 as f64) * 2_f64.powf(-7.)
    }
}

impl From<f32> for Q7 {
    fn from (n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> Self {
        f64::from(n) as f32
    }
}

fn main() {
    let f: f64 = 0.89765;
    let q: Q7 = f.into();
    println!("{f} * 2^-7 = {}", q.0);
    println!("2^-7 = {}", 2_f64.powf(-7.));
    println!("f64 {f}, q7 {q:?}, f32 {}", f32::from(q));
}
