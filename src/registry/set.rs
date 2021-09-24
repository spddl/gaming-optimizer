use winreg::enums::RegType::REG_BINARY;
use winreg::RegKey;
use winreg::RegValue;

pub fn set_u32_reg(reg: &RegKey, key: &str, val: &u32, reg_path: &str, write_settings: bool) {
    match reg.get_value(key) {
        Err(_) => {
            if write_settings {
                match reg.set_value(key, val) {
                    Err(e) => println!("\x1b[0;91m{:?}\x1b[0m", e),
                    Ok(()) => println!(
                        "write reg key: \x1b[0;92m{}\\{} = dword:{}\x1b[0m",
                        reg_path, key, val
                    ),
                }
            } else {
                println!(
                    "setting missing: \x1b[0;93m{}\\{} = dword:{}\x1b[0m",
                    reg_path, key, val
                );
            }
        }
        Ok(reg_val) => {
            let value: u32 = reg_val;
            if value != *val {
                if write_settings {
                    match reg.set_value(key, val) {
                        Err(e) => println!("\x1b[0;91m{:?}\x1b[0m", e),
                        Ok(()) => println!(
                            "write reg key: \x1b[0;92m{}\\{} = dword:{}\x1b[0m",
                            reg_path, key, val
                        ),
                    }
                } else {
                    println!(
                        "wrong setting: \x1b[0;93m{}\\{} = dword:{}\x1b[0m (your value: {})",
                        reg_path, key, val, value
                    );
                }
            } else {
                println!("correct setting: {}\\{} = dword:{}", reg_path, key, val);
            }
        }
    }
}

pub fn set_str_reg(reg: &RegKey, key: &str, val: &str, reg_path: &str, write_settings: bool) {
    match reg.get_value(key) {
        Err(_) => {
            if write_settings {
                match reg.set_value(key, &val) {
                    Err(e) => println!("\x1b[0;91m{:?}\x1b[0m", e),
                    Ok(()) => println!(
                        "write reg key: \x1b[0;92m{}\\{} = sz:{}\x1b[0m",
                        reg_path, key, val
                    ),
                }
            } else {
                println!(
                    "setting missing: \x1b[0;93m{}\\{} = sz:{}\x1b[0m",
                    reg_path, key, val
                );
            }
        }
        Ok(reg_val) => {
            let value: String = reg_val;
            if value != val {
                if write_settings {
                    match reg.set_value(key, &val) {
                        Err(e) => println!("\x1b[0;91m{:?}\x1b[0m", e),
                        Ok(()) => println!(
                            "write reg key: \x1b[0;92m{}\\{} = sz:{}\x1b[0m",
                            reg_path, key, val
                        ),
                    }
                } else {
                    println!(
                        "wrong setting: \x1b[0;93m{}\\{} = sz:{}\x1b[0m (your value: {})",
                        reg_path, key, val, value
                    );
                }
            } else {
                println!("correct setting: {}\\{} = sz:{}", reg_path, key, val);
            }
        }
    }
}

pub fn set_vac_reg(reg: &RegKey, key: &str, val: Vec<u8>, reg_path: &str, write_settings: bool) {
    match reg.get_raw_value(key) {
        Err(_) => {
            if write_settings {
                match reg.set_raw_value(
                    key,
                    &RegValue {
                        vtype: REG_BINARY,
                        bytes: val,
                    },
                ) {
                    Err(e) => println!("\x1b[0;91m{:?}\x1b[0m", e),
                    Ok(()) => println!("write reg key: \x1b[0;92m{}\\{}\x1b[0m", reg_path, key),
                }
            } else {
                println!("setting missing: \x1b[0;93m{}\\{}\x1b[0m", reg_path, key);
            }
        }
        Ok(reg_val) => {
            let value: Vec<u8> = reg_val.bytes;
            if value != val {
                // *fixes += 1;
                if write_settings {
                    match reg.set_raw_value(
                        key,
                        &RegValue {
                            vtype: REG_BINARY,
                            bytes: val,
                        },
                    ) {
                        Err(e) => println!("\x1b[0;91m{:?}\x1b[0m", e),
                        Ok(()) => println!("write reg key: \x1b[0;92m{}\\{}\x1b[0m", reg_path, key),
                    }
                } else {
                    println!("wrong setting: \x1b[0;93m{}\\{}\x1b[0m", reg_path, key);
                }
            } else {
                println!("correct setting: {}\\{} = binary:{:?}", reg_path, key, val);
            }
        }
    }
}

pub fn del_key_reg(reg: &RegKey, key: &str, reg_path: &str) {
    match reg.delete_value(key) {
        Err(e) => println!("\x1b[0;91m{:?}\x1b[0m", e),
        Ok(()) => println!("deleted key: \x1b[0;93m{}\\{}\x1b[0m", reg_path, key),
    }
}
