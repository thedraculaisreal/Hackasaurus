use proc_mem::Process;

fn main() {
    let game = Process::with_name("cs2.exe").expect("Failed to find game");
    let client = game.module("client.dll").unwrap();
    println!("0x{:x}", client.base_address());
}
