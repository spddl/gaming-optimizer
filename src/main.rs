use dialoguer::{theme::ColorfulTheme, Select};

mod ping;
mod powershell;
mod registry;

// TODO! Performance Tuning Network Adapters https://docs.microsoft.com/en-us/windows-server/networking/technologies/network-subsystem/net-sub-performance-tuning-nics
#[cfg(windows)]
fn main() {
    let game_launcher_bloat = vec![
        String::from("EpicWebHelper.exe"),
        String::from("GameOverlayUI.exe"),
        String::from("OriginWebHelperService.exe"),
        String::from("RiotClientCrashHandler.exe"),
        String::from("RiotClientUx.exe"),
        String::from("RiotClientUxRender.exe"),
        String::from("vgtray.exe"),
        String::from("SocialClubHelper.exe"),
        String::from("SteamService.exe"),
        String::from("steamwebhelper.exe"),
        String::from("UplayWebCore.exe"),
    ];

    let games = vec![String::from("r5apex.exe"), String::from("csgo.exe")];
    let system_process_high =
        vec![String::from("csrss.exe"), String::from("ntoskrnl.exe")];

    let system_process_below_normal = vec![
        String::from("SearchIndexer.exe"), // Microsoft Windows Search Indexer
        String::from("svchost.exe"),
    ];
    println!("\n# Check Registry");
    let mtu = ping::ping();
    let dpi = registry::apply_get_dpi();
    let reg_settings = registry::factory_settings(&dpi);
    registry::apply_reg_tweaks(&reg_settings, false);
    registry::apply_tcp_tweaks(&mtu, false, false);

    for process in game_launcher_bloat.iter() {
        registry::set_cpu_priority(registry::CpuPriority {
            process: process.to_string(),
            cpu_priority_class: Some(5u32), // 5 = Below Normal
            io_priority: Some(1u32),        // 1 = Low
            page_priority: None,
            working_set_limit_in_kb: None,
        }, false);
    }

    for process in games.iter() {
        registry::set_cpu_priority(registry::CpuPriority {
            process: process.to_string(),
            cpu_priority_class: Some(3u32), // 3 = High
            io_priority: Some(3u32),        // 3 = High
            page_priority: None,
            working_set_limit_in_kb: None,
        }, false);
    }

    for process in system_process_high.iter() {
        registry::set_cpu_priority(registry::CpuPriority {
            process: process.to_string(),
            cpu_priority_class: Some(3u32), // 3 = High
            io_priority: Some(3u32),        // 3 = High
            page_priority: None,
            working_set_limit_in_kb: None,
        }, false);
    }

    for process in system_process_below_normal.iter() {
        registry::set_cpu_priority(registry::CpuPriority {
            process: process.to_string(),
            cpu_priority_class: Some(5u32), // 5 = Below Normal
            io_priority: Some(1u32),        // 1 = Low
            page_priority: None,
            working_set_limit_in_kb: None,
        }, false);
    }

    println!("\n# Check PowerPlan");
    let powerplan = registry::factory_powerplan();
    registry::check_powerplan(&powerplan, false);

    println!("\n# Check BcdStore");
    powershell::check_bcd_store();

    if let Ok(select) = Select::with_theme(&ColorfulTheme::default())
        .items(&vec![
            "Apply fixes",
            "Restore Windows Default Settings",
            "Exit",
        ])
        .default(0)
        .interact()
    {
        match select {
            0 => {
                registry::check_powerplan(&powerplan, true);
                registry::apply_reg_tweaks(&reg_settings, true);
                registry::apply_tcp_tweaks(&mtu, true, false);

                powershell::set_bcd_store(false);

                for process in game_launcher_bloat.iter() {
                    registry::set_cpu_priority(registry::CpuPriority {
                        process: process.to_string(),
                        cpu_priority_class: Some(5u32), // 5 = Below Normal
                        io_priority: Some(1u32),        // 1 = Low
                        page_priority: None,
                        working_set_limit_in_kb: None,
                    }, true);
                }

                for process in games.iter() {
                    registry::set_cpu_priority(registry::CpuPriority {
                        process: process.to_string(),
                        cpu_priority_class: Some(3u32), // 3 = High
                        io_priority: Some(3u32),        // 3 = High
                        page_priority: None,
                        working_set_limit_in_kb: None,
                    }, true);
                }

                for process in system_process_high.iter() {
                    registry::set_cpu_priority(registry::CpuPriority {
                        process: process.to_string(),
                        cpu_priority_class: Some(3u32), // 3 = High
                        io_priority: Some(3u32),        // 3 = High
                        page_priority: None,
                        working_set_limit_in_kb: None,
                    }, true);
                }

                for process in system_process_below_normal.iter() {
                    registry::set_cpu_priority(registry::CpuPriority {
                        process: process.to_string(),
                        cpu_priority_class: Some(5u32), // 5 = Below Normal
                        io_priority: Some(1u32),        // 1 = Low
                        page_priority: None,
                        working_set_limit_in_kb: None,
                    }, true);
                }
            }
            1 => {
                registry::default_powerplan();
                registry::restore_default_reg(&reg_settings).unwrap();
                registry::apply_tcp_tweaks(&mtu, true, true);

                powershell::set_bcd_store(true);
            }
            _ => std::process::exit(0),
        }
    };

    dialoguer::Input::<String>::new().interact_text().unwrap();
}
