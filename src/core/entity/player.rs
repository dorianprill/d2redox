// Player struct
// composes with a WorldEntity struct

use core::entity::Entity;


#[allow(dead_code)]
pub struct Player {
        name:               String,
        has_mercenary:      bool,
        directory_known:    bool,
        mercenary_id:       u32,
        level:              u32,
        portal_id:          u32,
        pub entity:         Entity
        //protected Globals.CharacterClassType m_class;
}

// TODO translate form c#
//public Globals.CharacterClassType Class { get { return m_class; } set { m_class = value; } }

impl Player {
    pub fn name(self) -> String {
        return self.name
    }

    pub fn has_mercenary(&self) -> bool {
        return self.has_mercenary
    }

    pub fn mercenary_id(&self) -> u32 {
        return self.mercenary_id
    }

    pub fn mercenary_id_set(&mut self, merc_id: u32) {
        self.has_mercenary  = true;
        self.mercenary_id   = merc_id;
    }

    pub fn directory_known(&self) -> bool {
        return self.directory_known
    }

    pub fn level(&self) -> u32 {
        return self.level
    }

    pub fn set_level(&mut self, lvl: u32) -> u32 {
        self.level = lvl;
        return self.level
    }

    pub fn portal_id(&self) -> u32 {
        return self.portal_id
    }

    pub fn set_portal_id(&mut self, portal_id: u32) -> u32 {
        self.portal_id = portal_id;
        return portal_id
    }

}


pub enum PlayerItemSlot {
    Helm            = 0x01,
    Amulet          = 0x02,
    Armor           = 0x03,
    LeftWeapon      = 0x04,
    RightWeapon     = 0x05,
    LeftRing        = 0x06,
    RightRing       = 0x07,
    Belt            = 0x08,
    Boots           = 0x09,
    Gloves          = 0x0A
}

pub enum PhraseId {
    Help    = 0x19,
    Follow  = 0x1A,
    Gift    = 0x1B,
    Thanks  = 0x1C,
    Sorry   = 0x1D,
    Bye     = 0x1E,
    Die     = 0x1F,
    Flee    = 0x20
}

// taken from https://bnetdocs.org/packet/98/d2gs-trade
//Press Accept button (unaccept) should be sent when placing items in the trade window as well
pub enum TradeActionId {
    CancelTradeRequest  = 0x02,
    AcceptTradeRequest  = 0x03,
    PressAcceptButton   = 0x04,
    UnpressAcceptButton = 0x07,
    RefreshWindow       = 0x08,
    CloseStash          = 0x12,
    WithdrawGold        = 0x13,
    DepositGold         = 0x14,
    CloseHoradricCube   = 0x17
}
