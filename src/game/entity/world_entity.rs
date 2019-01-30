use game::coordinate::Coordinate;

#[allow(dead_code)]
pub struct WorldEntity {
    initialized     :bool,
    id              :u32,
    location        :Coordinate
}

impl WorldEntity {

    pub fn new(id: u32, x: u16, y: u16) -> WorldEntity{
        WorldEntity{initialized: true, id: id, location: Coordinate::new(x, y)}
    }

    pub fn initialized(&self) -> bool {
        return self.initialized
    }

    pub fn id(&self) -> u32 {
        return self.id
    }

    pub fn location(self) -> Coordinate {
        return self.location
    }
}
