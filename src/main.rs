use proc_mem::Process;
use std::thread;
use std::time::Duration;

mod offsets;
mod entities;

fn main() {
    thread::spawn(move || {
	let game = Process::with_name("cs2.exe").expect("Failed to find game");
	let client = game.module("client.dll").unwrap();
	let server = game.module("server.dll").unwrap();
	entities::entity_list_loop(client, server, game);
    });
    thread::sleep(Duration::from_millis(1000));
    unsafe {
	loop {
	    for player in entities::PLAYER_LIST.clone() {
		player.print_values();
		thread::sleep(Duration::from_millis(1000));
	    }
	}
    }
}
