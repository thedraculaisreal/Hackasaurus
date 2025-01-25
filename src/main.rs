use proc_mem::Process;
use std::thread;
use std::time::Duration;

mod offsets;
mod entities;

fn main() {
    let game = Process::with_name("cs2.exe").expect("Failed to find game");
    let client = game.module("client.dll").unwrap();
    println!("0x{:x}", client.base_address());
    let local_player_address: usize = game.read_mem::<usize>(client.base_address() + offsets::LOCAL_PLAYER)
	.expect("failed to read local_player address");
    loop {
	let health: i32 = game.read_mem::<i32>(local_player_address + 0x344)
	    .expect("failed to read health value");
	println!("{health}");
	thread::sleep(Duration::from_millis(1000));
    }
}
