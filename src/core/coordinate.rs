// (X,Y)-coordinates on the map to localize objects and entities
#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    pub fn new(x: u16, y: u16) -> Coordinate {
        Coordinate { x, y }
    }

    /// DEPRECATED (derived for struct)
    // pub fn equals(&self, other: Coordinate) -> bool {
    //     if self.x == other.x && self.y == other.y {
    //         return true
    //     } else {
    //         return false
    //     }
    // }

    pub fn hash(&self) -> i32 {
        // bitwise XOR
        (self.x ^ self.y) as i32
    }

    pub fn distance(&self, other: Coordinate) -> f32 {
        ((self.x - other.x).pow(2) as f32 + (self.y - other.y).pow(2) as f32).sqrt()
    }
}
