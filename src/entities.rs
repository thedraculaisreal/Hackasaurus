use crate::offsets;
use rustbot::Vec3;
use std::thread;
use std::time::Duration;

// global variables for accessing from other threads
pub static mut PLAYER_LIST: Vec<Player> = Vec::new();
pub static mut LOCAL_PLAYER: LocalPlayer = LocalPlayer {
    health: 0,
    head_pos: Vec3::new_const(0.0,0.0,0.0),
    view_angles: Vec3::new_const(0.0,0.0,0.0),
};

#[derive(Default , Clone)]
pub struct LocalPlayer {
    pub health: u8,
    pub head_pos: Vec3,
    pub view_angles: Vec3,
}

impl LocalPlayer {
    pub fn new(address: usize, game: &proc_mem::Process, client: &proc_mem::Module) -> Self {
	let health: u8 = game.read_mem::<u8>(address + offsets::player_pawn::HEALTH)
	    .expect("failed to read health");
	let head_pos: Vec3 = game.read_mem::<Vec3>(address + offsets::player_pawn::HEAD_POS)
	    .expect("Failed to read health_pos");
	let view_angles: Vec3 = game.read_mem::<Vec3>(client.base_address() + offsets::local_pawn::VIEW_ANGLES)
	    .expect("failed to read view_angles");
	Self {
	    health,
	    head_pos,
	    view_angles,
	}
    }
}

#[derive(Default , Clone)]
pub struct Player {
    pub health: u8,
    pub feet_pos: Vec3,
    pub head_pos: Vec3,
}

impl Player {
    pub fn new(address: usize, game: &proc_mem::Process) -> Self {
	let health: u8 = game.read_mem::<u8>(address + offsets::player_pawn::HEALTH)
	    .expect("failed to read health");
	let feet_pos: Vec3 = game.read_mem::<Vec3>(address + offsets::player_pawn::FEET_POS)
	    .expect("Faled to read feet_pos");
	let head_pos: Vec3 = game.read_mem::<Vec3>(address + offsets::player_pawn::HEAD_POS)
	    .expect("Failed to read health_pos");
	Self {
	    health,
	    feet_pos,
	    head_pos,
	}
    }
    pub fn print_values(&self) {
        println!("{}", self.health);
        println!("{}, {}, {} ", self.feet_pos.x, self.feet_pos.y, self.feet_pos.z);
    }
}

pub fn entity_list_loop(client: proc_mem::Module, server: proc_mem::Module, game: proc_mem::Process) {
    unsafe {
	loop {
	    PLAYER_LIST = Vec::new();
	    let local_player_addr = game.read_mem::<usize>(client.base_address() + offsets::LOCAL_PLAYER)
		.expect("failed to read local_player_addr");
	    LOCAL_PLAYER = LocalPlayer::new(local_player_addr, &game, &client);
	    let entity_list = game.read_mem::<usize>(client.base_address() + offsets::ENTITY_LIST)
		.expect("failed to read entity_list address");
	    let player_count = game.read_mem::<i32>(server.base_address() + offsets::PLAYER_COUNT)
		.expect("failed to read player_count");
	    // start at 1 so we dont read local_player values;
	    let mut player_index: i32 = 1;
	    while player_index <= player_count {
		let list_entry = game.read_mem::<usize>(entity_list + (((8 * (player_index & 0x7FFF) >> 9) + 16) as usize))
		    .expect("failed to read list_entry");
		let entity_controller = game.read_mem::<usize>(list_entry + ((120 * (player_index & 0x1FF)) as usize))
		    .expect("failed to read entity_controller");
		let player_pawn = game.read_mem::<u32>(entity_controller + offsets::PLAYER_PAWN)
		    .expect("failed to read player_pawn");
		let list_entity = game.read_mem::<usize>(entity_list + ((8 * ((player_pawn & 0x7FFF) >> 9) + 16) as usize))
		    .expect("failed to read list entity");
		let pawn_addr = game.read_mem::<usize>(list_entity + ((120 * (player_pawn & 0x1FF)) as usize))
		    .expect("failed to read pawn_addr");
		let player = Player::new(pawn_addr, &game);
		PLAYER_LIST.push(player);
		thread::sleep(Duration::from_millis(1));
		player_index += 1;
	    }
	}
    }
}

