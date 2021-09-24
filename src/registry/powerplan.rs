use std::process::Command;

pub fn set(
    scheme_guid: &String,
    sub_guid_path: &String,
    setting_guid_path: &String,
    setting_guid_data: &u32,
) {
    Command::new("powercfg")
        .args(&[
            "-SetAcValueIndex",
            &scheme_guid,
            &sub_guid_path,
            &setting_guid_path,
            &setting_guid_data.to_string(),
        ])
        .output()
        .expect("Failure setting Powerplan Setting");
}

pub fn default() {
    Command::new("powercfg")
        .args(&[ "-restoredefaultschemes" ])
        .output()
        .expect("Failure reset Powerplan Setting");
}
