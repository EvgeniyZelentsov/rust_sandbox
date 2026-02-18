use std::fmt;

pub struct StarSystem{
    pub coordinate: Coordinate
}



pub struct Coordinate{
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x:{}, y:{}, z:{})", self.x, self.y, self.z)
    }
}


