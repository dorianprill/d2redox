use coordinate::*;

pub mod world_entity;

struct WorldEntity {
    initialized     :bool;
    id              :u32;
    location        :Coordinate;



        public Entity()
        {
            m_initialized = false;
            m_location = new Coordinate(0, 0);
        }

        public Entity(UInt32 id, UInt16 x, UInt16 y)
        {
            m_initialized = true;
            m_id = id;
            m_location = new Coordinate(x, y);
        }

    }

    impl WorldEntity {
        WorldEntity(&self, id: u32, x: u16, y: u16) {
            self.initalized  = true;
            self.id          = id;
            self.location         = Coordinate(x, y);
        }

        pub fn initialized(&self) -> bool {
            return initialized
        }

        pub fn id(&self) -> u32 {
            return self.id
        }

        pub fn location(&self) -> Coordinate {
            return self.location
    }
