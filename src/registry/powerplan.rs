use std::process::Command;

// https://github.com/nsmryan/luster/blob/master/src/main.rs
// fn find_guid(search_string: &str, query: &str, skip: usize) -> String {
//     let mut guid = None;
//     for line in search_string.lines() {
//         if line.contains(query) {
//             guid = line.trim().split(" ").skip(skip).next();
//             break;
//         }
//     }
//     let guid = guid.expect("Could not find GUID");

//     return guid.to_string();
// }

// pub fn check() {
//     // powercfg /SETACVALUEINDEX fb5220ff-7e1a-47aa-9a42-50ffbf45c673 7516b95f-f776-4464-8c53-06167f40cc99 3c0bc021-c8a8-4e07-a973-6b14cbcb2b7e 600
//     let power_query_output = Command::new("powercfg")
//         .args(&["/q"])
//         .output()
//         .expect("failed to run powercfg query");

//     // println!("power_query_output: {}", &power_query_output.stdout);
//     let power_query =
//         std::str::from_utf8(&power_query_output.stdout).expect("Invalid output from powercfg");

//     let scheme_guid = find_guid(power_query, "Power Scheme GUID", 3);
//     // let subgroup_guid = find_guid(power_query, "(Display)", 2);
//     // let setting_guid = find_guid(power_query, "(Display brightness)", 3);
//     println!("scheme_guid: {}", scheme_guid);
//     println!("power_query: {}", power_query);
//     // Command::new("powercfg")
//     //     .args(&[
//     //         "-SetDcValueIndex",
//     //         &scheme_guid,
//     //         &subgroup_guid,
//     //         &setting_guid,
//     //         &value,
//     //     ])
//     //     .output()
//     //     .expect("Failure setting AC brigtness value");
// }

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

#[allow(dead_code)]
pub fn default() {
    Command::new("powercfg")
        .args(&[ "-restoredefaultschemes" ])
        .output()
        .expect("Failure reset Powerplan Setting");
}
