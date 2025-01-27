use crate::offsets;
use proc_mem::{Process, Module};
use std::thread;
use std::time::Duration;

pub fn entity_list_loop(client: proc_mem::Module, server: proc_mem::Module, game: proc_mem::Process) {
    loop {
	let entity_list = game.read_mem::<usize>(client.base_address() + offsets::ENTITY_LIST)
	    .expect("failed to read entity_list address");
	let player_count = game.read_mem::<i32>(server.base_address() + offsets::PLAYER_COUNT)
	    .expect("failed to read player_count");
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
	    let pawn_health = game.read_mem::<i32>(pawn_addr + offsets::PAWN_HEALTH)
		.expect("failed to read pawn_health");
	    println!("{}", pawn_health);
	    thread::sleep(Duration::from_millis(1000));
	    player_index += 1;
	}
    }
}

