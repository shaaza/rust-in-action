#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Q7(i8);

impl From<f64> for Q7 {
    fn from(n: f64) -> Self {
        // assert!(n >= -1.0);
        // assert!(n <= 1.0);
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<f32> for Q7 {
    fn from(n: f32) -> Self {
        Q7::from(n as f64)
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> f32 {
        f64::from(n) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_to_q7() {
        assert_eq!(Q7::from(0.7_f32), Q7(89));
    }

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.0), Q7(127));
        assert_eq!(Q7::from(-10.0), Q7(-128));
    }

    #[test]
    fn q7_to_f32() {
        let n: f32 = Q7(89).into();

        assert_eq!(n, 0.6953125);
    }
}
