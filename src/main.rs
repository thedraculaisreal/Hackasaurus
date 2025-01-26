use proc_mem::Process;
use std::thread;
use std::time::Duration;

mod offsets;
mod entities;

fn main() {
    let game = Process::with_name("cs2.exe").expect("Failed to find game");
    let client = game.module("client.dll").unwrap();
    let server = game.module("server.dll").unwrap();
    println!("0x{:x}", client.base_address());
    entities::entity_list_loop(client, server, game);
}
