use game::coordinate::Coordinate;

pub struct WorldEntity {
    initialized     :bool,
    id              :u32,
    location        :Coordinate
}

impl WorldEntity {

    pub fn new(&self, id: u32, x: u16, y: u16) {
        self.initialized  = true;
        self.id           = id;
        self.location     = Coordinate{x, y};
    }

    pub fn initialized(&self) -> bool {
        return self.initialized
    }

    pub fn id(&self) -> u32 {
        return self.id
    }

    pub fn location(&self) -> Coordinate {
        return self.location
    }
}
