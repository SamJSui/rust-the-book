pub trait Distance {
    fn distance(&self) -> f64;
}

pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Distance for Point2D {
    fn distance(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

impl Distance for Point3D {
    fn distance(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}