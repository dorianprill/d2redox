
pub enum Status {
    Uninitialized,
    InvalidCdKey,
    InvalidExpCdKey,
    KeyInUse,
    ExpKeyInUse,
    BannedCdKey,
    BannedExpCdKey,
    LoginError,
    McpLogonFail,
    RealmDown,
    OnMcp,
    NotInGame
}

pub struct Client {
    //game_instance: &Game;
}
