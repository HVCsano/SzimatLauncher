use std::{process::Command, thread, time::Duration};

use crate::{game::is_game_running, launcher::ensure_launcher};

mod game;
mod launcher;

fn main() {
    println!(
        "=== SZIMATLAUNCHER v{} by csano.hu",
        env!("CARGO_PKG_VERSION")
    );
    ensure_launcher();
    let first_check = is_game_running();
    if !first_check {
        println!("[LAUNCHER] Játék nem fut, Launcher indítása...");
        let szimat = Command::new("./SeeMTA_Launcher.exe").spawn();
        if szimat.is_err() {
            panic!("{}", szimat.unwrap_err());
        }
        loop {
            thread::sleep(Duration::from_secs(5));
            if is_game_running() {
                break;
            }
            println!("[LAUNCHER] Játék még mindig nem fut...")
        }
    }
    loop {
        if !is_game_running() {
            break;
        }
        thread::sleep(Duration::from_secs(20))
    }
    println!("[MAIN] Játék már nem fut, leállítás...")
}
