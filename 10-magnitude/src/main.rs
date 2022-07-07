use magnitude::{Point2D, Point3D, Distance};

struct Vector<T, U, V> {
    i: T,
    j: U,
    k: V,
}

impl Vector <f64, f64, f64> {
    fn magnitude(&self) -> f64 {
        (self.i.powf(2.0) + self.j.powf(2.0) + self.k.powf(2.0)).sqrt()
    }
}
 
fn main() {
    let v1 = Vector { 
        i: 1.0,
        j: 2.0,
        k: 3.0, 
    };
    let _p1 = Point2D {
        x: 1.0,
        y: 2.0,
    };
    let _p2 = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    println!("Vector: {:.2}", v1.magnitude());
    println!("P1: {:.2}", _p1.distance());
    println!("P2: {:.2}", _p2.distance());
}
