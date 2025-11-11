use sysinfo::System;

const GAME_EXECUTABLE: &str = "gta_sa.exe";

/// Checks if the specified game process is currently running.
pub fn is_game_running() -> bool {
    // 1. Initialize the System struct. This will fetch all current process information.
    let mut sys = System::new_all();

    // We need to refresh the process list to ensure we have the latest data
    sys.refresh_all();

    // 2. Iterate through all running processes.
    // processes() returns a reference to a HashMap of processes.
    for (_pid, process) in sys.processes() {
        // 3. Compare the process name (case-insensitive search is safer).
        if process.name().eq_ignore_ascii_case(GAME_EXECUTABLE) {
            println!(
                "[GAME] ✅ Játék '{}' megtaláltva (PID: {}).",
                GAME_EXECUTABLE,
                process.pid()
            );
            return true;
        }
    }

    println!("[GAME] ❌ Játék '{}' nem fut.", GAME_EXECUTABLE);
    false
}
