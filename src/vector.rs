use crate::tuple::Tuple;

pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
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
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vector() {
        let vec = Vector::new(4.3, -4.2, 3.1);

        assert_eq!(vec.x(), 4.3);
        assert_eq!(vec.y(), -4.2);
        assert_eq!(vec.z(), 3.1);
        assert_eq!(vec.w(), 1.0)
    }
}
