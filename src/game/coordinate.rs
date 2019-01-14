// (X,Y)-coordinates on the map to localize objects and entities
pub struct Coordinate {
    x: u16,
    y: u16
}

impl Coordinate {

    pub fn new(x: u16, y: u16) -> Coordinate { // we create a method to instantiate `Foo`
        Coordinate { x: x, y: y }
    }

    pub fn equals(&self, other: Coordinate) -> bool {
        if self.x == other.x && self.y == other.y {
            return true
        } else {
            return false
        }
    }

    pub fn get_hash(&self) -> i32 {
        // bitwise XOR
        (self.x ^ self.y) as i32
    }

    pub fn distance(&self, other: Coordinate) -> f32 {
        ( (self.x - other.x).pow(2) as f32 + (self.y - other.y).pow(2) as f32).sqrt()
    }
}
