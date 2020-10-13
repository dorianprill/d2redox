pub mod map;

pub(crate) enum CollisionFlag {
    None                = 0x0000,
    BlockWalk           = 0x0001,
    BlockLineOfSight    = 0x0002,
    Wall                = 0x0004,
    BlockPlayer         = 0x0008,
    AlternateTile       = 0x0010,
    Blank               = 0x0020,
    Missile             = 0x0040,
    Player              = 0x0080,
    NPCLocation         = 0x0100,
    Item                = 0x0200,
    Object              = 0x0400,
    ClosedDoor          = 0x0800,
    NPCCollision        = 0x1000,
    FriendlyNPC         = 0x2000,
    Unknown             = 0x4000,
    DeadBody            = 0x8000, // also portal
    // ThickenedWall		= 0xfefe,
    Avoid = 0xffff,
    Special = 0xf000
}


pub struct Exit {
    target:     u32,
    position:   Point,
    exit_type:  ExitType,
    tile_id:    u32,
}


pub trait Map {

    fn width() -> u32;
    fn height() -> u32;

    fn get_map_data(point: &Coordinate, abs: bool) -> u32;
    fn is_valid_point(point: &Coordinate, abs: bool) -> bool;

    fn point_has_flag(flag: u32, point: &Coordinate, abs: bool) -> bool;
    fn path_has_flag(flag: u32, point: &Coordinate, abs: bool) -> bool;
    fn allow_crit_space();
}
