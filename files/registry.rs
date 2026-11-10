//! Windows registry read helpers for Esprit Cam.

pub const APP_KEY: &str = "Software\\Esprit\\Esprit Cam";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Esprit Cam";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Esprit Cam") },
        RegValue { name: "Version",     value: String::from("v4.0.24") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Esprit Cam") },
        RegValue { name: "Publisher",       value: String::from("Esprit") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Esprit Cam\\uninstall.exe\"") },
    ]
}
