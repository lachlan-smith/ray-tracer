#[derive(Clone, Copy, Debug)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Colour { r, g, b }
    }
}

impl PartialEq for Colour {
    fn eq(&self, other: &Self) -> bool {
        relative_eq!(self.r, other.r)
            && relative_eq!(self.g, other.g)
            && relative_eq!(self.b, other.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_colour() {
        let colour = Colour::new(-0.5, 0.4, 1.7);

        assert_relative_eq!(colour.r, -0.5);
        assert_relative_eq!(colour.g, 0.4);
        assert_relative_eq!(colour.b, 1.7);
    }

    #[test]
    fn colours_equality() {
        let colour1 = Colour::new(-0.5, 0.4, 1.7);
        let colour2 = Colour::new(-0.5, 0.4, 1.7);

        assert_eq!(colour1, colour2)
    }
}
