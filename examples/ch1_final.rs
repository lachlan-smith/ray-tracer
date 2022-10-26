use ray_tracer::{point::Point, tuple::Tuple, vector::Vector};

fn main() {
    let mut pos = Point::new(0.0, 1.0, 0.0);
    let mut vel = Vector::new(2.0, 2.0, 0.0);
    let mut grav = Vector::new(0.0, -0.1, 0.0);
    let mut wind = Vector::new(-0.01, 0.0, 0.0);
    let mut count = 0;

    while pos.y() > 0.0 {
        count = count + 1;
        (pos, vel, grav, wind) = tick(pos, vel, grav, wind);

        println!("x:{} y:{}", pos.x(), pos.y());
    }

    println!("Took {} iterations", count)
}

fn tick(pos: Point, vel: Vector, grav: Vector, wind: Vector) -> (Point, Vector, Vector, Vector) {
    return (pos + vel, vel + grav + wind, grav, wind);
}
