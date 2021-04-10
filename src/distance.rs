use crate::domain::Coords;

pub fn distance(a: &Coords, b: &Coords) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {

    use crate::distance::distance;
    use crate::domain::Coords;

    #[test]
    fn coordinate_distance() {
        assert_eq!(
            true,
            (distance(
                &Coords {
                    x: -11.46875,
                    y: 39.78125,
                    z: 22.78125
                },
                &Coords {
                    x: 73.875,
                    y: -3.5625,
                    z: -52.625
                },
            ) - 121.853_760_168_439_2_f64)
                .abs()
                < f64::EPSILON
        );
    }
}
