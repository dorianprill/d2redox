pub mod coordinate;

struct Coordinate {
    x: u16;
    y: u16;
}

impl Coordinate {
    pub fn equals(self&, other: Coordinate) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        } else {
            false
        }
    }

    pub fn get_hash(self&) -> i32 {
        // bitwise XOR
        self.x ^ self.y;
    }

    pub fn distance(self&, other: Coordinate) -> f32 {
        ( (self.x - other.x).pow(2) + (self.y - other.y).pow(2) ).sqrt()
    }
}
