use num_complex::Complex;

/// Compute roots of a quadratic equation a*x^2 + b*x + c = 0.
///
/// Returns `None` when the equation is degenerate (a == 0 and b == 0).
/// Otherwise returns `Some((root1, root2))` where both roots are `Complex<f64>`.
pub fn quadratic_roots(a: f64, b: f64, c: f64) -> Option<(Complex<f64>, Complex<f64>)> {
    const EPS: f64 = 1e-12;

    if a.abs() <= EPS {
        // Linear or degenerate: bx + c = 0
        if b.abs() <= EPS {
            // No equation or infinite solutions; return None for degenerate
            return None;
        }
        let root = -c / b;
        let r = Complex::new(root, 0.0);
        return Some((r, r));
    }

    // Use complex arithmetic for discriminant to automatically handle negative values
    let two_a = 2.0 * a;
    let disc = b * b - 4.0 * a * c;
    let sqrt_disc = Complex::new(disc, 0.0).sqrt();

    let minus_b = Complex::new(-b, 0.0);
    let root1 = (minus_b + sqrt_disc) / Complex::new(two_a, 0.0);
    let root2 = (minus_b - sqrt_disc) / Complex::new(two_a, 0.0);
    Some((root1, root2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    fn approx_eq(a: Complex<f64>, b: Complex<f64>) -> bool {
        let eps = 1e-9;
        (a.re - b.re).abs() < eps && (a.im - b.im).abs() < eps
    }

    #[test]
    fn real_distinct_roots() {
        // x^2 - 3x + 2 = 0 -> roots 1 and 2
        let (r1, r2) = quadratic_roots(1.0, -3.0, 2.0).unwrap();
        assert!(approx_eq(r1, Complex::new(2.0, 0.0)) || approx_eq(r1, Complex::new(1.0, 0.0)));
        assert!(approx_eq(r2, Complex::new(2.0, 0.0)) || approx_eq(r2, Complex::new(1.0, 0.0)));
    }

    #[test]
    fn complex_conjugate_roots() {
        // x^2 + 1 = 0 -> roots i and -i
        let (r1, r2) = quadratic_roots(1.0, 0.0, 1.0).unwrap();
        assert!(approx_eq(r1, Complex::new(0.0, 1.0)) || approx_eq(r1, Complex::new(0.0, -1.0)));
        assert!(approx_eq(r2, Complex::new(0.0, 1.0)) || approx_eq(r2, Complex::new(0.0, -1.0)));
    }

    #[test]
    fn linear_equation() {
        let (r1, r2) = quadratic_roots(0.0, 2.0, -4.0).unwrap();
        assert!(approx_eq(r1, Complex::new(2.0, 0.0)));
        assert!(approx_eq(r2, Complex::new(2.0, 0.0)));
    }

    #[test]
    fn degenerate() {
        assert!(quadratic_roots(0.0, 0.0, 0.0).is_none());
    }
}
