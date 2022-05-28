use crate::Float;

/// Computes the roots of the equation a t^2 + b t + c = 0.
/// The result will be of the form (t0, t1) with t0 <= t1 if the roots
/// exist and None otherwise.
pub fn quadratic(a: Float, b: Float, c: Float) -> Option<[Float; 2]> {
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        return None;
    }

    let root_discriminant = discriminant.sqrt();
    let q = if b < 0. {
        -0.5 * (b - root_discriminant)
    } else {
        -0.5 * (b + root_discriminant)
    };

    let t0 = q / a;
    let t1 = c / q;
    if t0 > t1 {
        Some([t1, t0])
    } else {
        Some([t0, t1])
    }
}

#[cfg(test)]
mod test {
    use super::quadratic;
    use crate::Float;

    #[test]
    fn test_quadratic() {
        struct Test {
            input: (Float, Float, Float),
            expected: Option<[Float; 2]>,
        }

        let tests = [
            Test {
                input: (1., 5., 6.),
                expected: Some([-3., -2.]),
            },
            Test {
                input: (5., 0., 9.),
                expected: None,
            },
        ];

        for test in tests {
            let (a, b, c) = test.input;
            let actual = quadratic(a, b, c);
            assert_eq!(test.expected, actual)
        }
    }
}
