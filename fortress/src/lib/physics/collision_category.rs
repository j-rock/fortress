use liquidfun::box2d::common::settings::UInt16;

pub const COLLIDE_ALL: UInt16  = 1;
pub const BARRIER: UInt16 = 1 << 1;
pub const INTERACT: UInt16  = 1 << 2;
pub const PICKUP: UInt16  = 1 << 3;
pub const PLAYER_BODY: UInt16  = 1 << 4;
pub const PLAYER_WEAPON: UInt16  = 1 << 5;
pub const WRAITH: UInt16  = 1 << 6;
pub const CAT_8: UInt16  = 1 << 7;
pub const CAT_9: UInt16  = 1 << 8;
pub const CAT_10: UInt16 = 1 << 9;
pub const CAT_11: UInt16 = 1 << 10;
pub const CAT_12: UInt16 = 1 << 11;
pub const CAT_13: UInt16 = 1 << 12;
pub const CAT_14: UInt16 = 1 << 13;
pub const CAT_15: UInt16 = 1 << 14;
pub const CAT_16: UInt16 = 1 << 15;

pub const MASK_ALLOW_ALL: UInt16 = 0xFFFF;
pub const MASK_ALLOW_NONE: UInt16 = 0x0;
