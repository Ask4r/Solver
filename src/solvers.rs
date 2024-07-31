const ROOT_MAX_ITER: usize = 100_000;
const INT_MAX_ITER: usize = 100_000;

pub fn root(f: impl Fn(f64) -> f64, mut x1: f64, mut x2: f64, eps: f64) -> Option<f64> {
    let mut tmp;
    let mut x3;
    for _ in 0..ROOT_MAX_ITER {
        if x1 > x2 {
            tmp = x1;
            x1 = x2;
            x2 = tmp;
        }
        if f(x1) == 0.0 {
            return Some(x1);
        }
        if f(x2) == 0.0 {
            return Some(x2);
        }
        if (x2 - x1 < eps) && (f(x1) * f(x2) < 0.0) {
            return Some(x1);
        }
        if f(x2) == f(x1) {
            break;
        }
        x3 = (x1 * f(x2) - x2 * f(x1)) / (f(x2) - f(x1));
        if f(x3) == 0.0 || f(x1) == f(x3) {
            return Some(x3);
        } else if f(x1) * f(x3) < 0.0 {
            x2 = x3;
        } else if f(x2) * f(x3) < 0.0 {
            x1 = x3;
        } else {
            break;
        }
    }
    return None;
}

pub fn integral(f: impl Fn(f64) -> f64, mut x1: f64, mut x2: f64, eps: f64) -> f64 {
    if x1 > x2 {
        let tmp = x1;
        x1 = x2;
        x2 = tmp;
    }

    let mut step = x2 - x1;
    let mut sum = 0.5 * (f(x1) + f(x2));
    let mut inc = 0.0;
    let mut xi;

    let mut n = 1_usize;
    while n < INT_MAX_ITER {
        xi = x1 + step * 0.5;
        inc = f(xi);
        for _ in 1..n {
            xi += step;
            inc += f(xi);
        }
        if step * f64::abs(sum - inc) < 6.0 * eps {
            break;
        }
        sum += inc;
        step *= 0.5;
        n <<= 1;
    }
    return 0.5 * step * (sum + inc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let f = |x| f64::sin(x) - x.powi(3) + 3.0;

        let eps = 0.000_000_1;

        let r = root(f, 1.0, 2.0, eps).unwrap();
        let i = integral(f, -2.0, 2.0, eps);
        let expected_r = 1.587_382_86;
        let expected_i = 12.0;

        assert!(f64::abs(r - expected_r) < eps);
        assert!(f64::abs(i - expected_i) < eps);
    }
}
