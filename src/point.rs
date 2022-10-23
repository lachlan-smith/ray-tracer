use crate::tuple::Tuple;

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_point() {
        let vec = Point::new(4.3, -4.2, 3.1);

        assert_eq!(vec.x(), 4.3);
        assert_eq!(vec.y(), -4.2);
        assert_eq!(vec.z(), 3.1);
        assert_eq!(vec.w(), 0.0)
    }
}
