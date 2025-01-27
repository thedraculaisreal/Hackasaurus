pub static ENTITY_LIST: usize = 0x1A197E8; // client.dll: dwEntityList 
pub static LOCAL_PLAYER: usize = 0x186DE00; // client.dll: dwLocalPlayerPawn
pub static PLAYER_COUNT: usize = 0x13EFD84; // server.dll 
pub static PLAYER_PAWN: usize = 0x80C; // client.dll: m_hPlayerPawn

pub mod player_pawn {
    pub static HEALTH: usize = 0x344; // client.dll: m_iHealth
    pub static FEET_POS: usize = 0xDB8; // client.dll: m_vLastSlopeCheckPos
    pub static HEAD_POS: usize = 0x1384; // client.dll: m_vecLastClipCameraPos
    pub static NAME: usize = 0x660; // client.dll: entity_controller 
}

pub mod local_pawn {
    pub static VIEW_ANGLES: usize = 0x1A8E9A0; // client.dll: dwViewAngles
}
