pub fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut sum = 0.0;
    for &coord in vector {
        sum += coord * coord;
    }
    sum.sqrt()
}

pub fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for item in vector {
        *item /= mag;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry() {
        let mut v = [0.0, 3.0, 4.0];
        assert_eq!(magnitude(&v), 5.0);
        normalize(&mut v);
        assert!((magnitude(&v) - 1.0).abs() < 1e-9);
    }
}
