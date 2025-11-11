use std::{fs::File, io, path::Path};

pub fn ensure_launcher() {
    let launcher_path = Path::new("SeeMTA_Launcher.exe");
    if !launcher_path.exists() {
        let mut client_dl =
            reqwest::blocking::get("https://client.seega.me/dl/SeeMTA%20Launcher%20v1.0.70.exe")
                .expect("[LAUNCHERPREP/ERROR] Launcher letöltése sikertelen");
        let mut client_out = File::create("SeeMTA_Launcher.exe")
            .expect("[LAUNCHERPREP/ERROR] Fájl létrehozása sikertelen");
        io::copy(&mut client_dl, &mut client_out)
            .expect("[LAUNCHERPREP/ERROR] Launcher mentése sikertelen");
        println!("[LAUNCHERPREP] SeeMTA Launcher letöltve")
    }
    println!("[LAUNCHERPREP] SeeMTA Launcher használatra kész")
}
