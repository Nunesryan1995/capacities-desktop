//! Windows registry read helpers for Capacities Desktop.

pub const APP_KEY: &str = "Software\\Capacities\\Capacities Desktop";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Capacities Desktop";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Capacities Desktop") },
        RegValue { name: "Version",     value: String::from("v1.2.14") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Capacities Desktop") },
        RegValue { name: "Publisher",       value: String::from("Capacities") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Capacities Desktop\\uninstall.exe\"") },
    ]
}
