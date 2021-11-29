mod set;
mod powerplan;

use set::*;

use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_ALL_ACCESS, KEY_READ, REG_DWORD,
};
use winreg::transaction::Transaction;
use winreg::RegKey;
use winreg::RegValue;

// println!("\x1b[0;92m INFO \x1b[0m");
// println!("\x1b[0;93m WARN \x1b[0m");
// println!("\x1b[0;91m ERR \x1b[0m");

struct U32Element {
    key: String,
    value: u32,
    default: Option<u32>,
}

struct StringElement {
    key: String,
    value: String,
    default: Option<String>,
}

struct VecElement {
    key: String,
    value: Vec<u8>,
    default: Option<Vec<u8>>,
}

struct RegTweaks {
    path: String,
    data: Vec<Either>,
}

enum Either {
    #[allow(dead_code)]
    StringElement(StringElement),
    U32Element(U32Element),
    VecElement(VecElement),
}

pub struct Settings {
    local_machine: Vec<RegTweaks>,
    current_user: Vec<RegTweaks>,
    users: Vec<RegTweaks>,
}

struct SettingGuid {
    path: String,
    data: u32,

}
struct SubGuid {
    path: String,
    data: Vec<SettingGuid>,
}

pub struct PowerPlan {
    scheme: Option<String>,
    data: Vec<SubGuid>,
}

pub fn factory_settings(dpi: &u32) -> Settings {
    let local_machine = vec![
        RegTweaks {
            path: String::from(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR",
            ),
            data: vec![
                // Windows Game Recording and Broadcasting.
                // This setting enables or disables the Windows Game Recording and Broadcasting features. If you disable this setting, Windows Game Recording will not be allowed.
                // If the setting is enabled or not configured, then Recording and Broadcasting (streaming) will be allowed.
                // https://admx.help/?Category=Windows_10_2016&Policy=Microsoft.Policies.GameDVR::AllowGameDVR
                Either::U32Element(U32Element {
                    key: String::from("AllowGameDVR"),
                    value: 0u32,
                    default: None,
                }),
            ],
        },
        RegTweaks { // macht wohl nur mit QoS Sinn
            path: String::from(
                "SOFTWARE\\Policies\\Microsoft\\Windows\\Psched",
            ),
            data: vec![
                // https://www.overclock.net/threads/network-stack-packet-scheduler-timer-resolution.1744292/
                // http://systemmanager.ru/win2k_regestry.en/94171.htm
                Either::U32Element(U32Element {
                    key: String::from("TimerResolution"),
                    value: 1u32,
                    default: None, // TODO: 
                }),
                // https://thomasknoefel.de/tag/nonbesteffortlimit/
                Either::U32Element(U32Element {
                    key: String::from("NonBestEffortLimit"),
                    value: 0u32,
                    default: None, // TODO: 
                }),
            ],
        },
        RegTweaks {
            path: String::from(
                "SOFTWARE\\Microsoft\\MSMQ\\Parameters",
            ),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("TcpNoDelay"),
                    value: 1u32,
                    default: None,
                }),

            ],
        },
        RegTweaks {
            path: String::from(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile",
            ),
            data: vec![
                // https://github.com/djdallmann/GamingPCSetup/blob/master/CONTENT/DOCS/NETWORK/README.md#operating-system-specific-configuration
                // https://github.com/djdallmann/GamingPCSetup/blob/master/CONTENT/RESEARCH/NETWORK/README.md#networkthrottlingindex
                Either::U32Element(U32Element {
                    key: String::from("NetworkThrottlingIndex"),
                    // value: 0xffff_ffff_u32,
                    value: 20_u32,
                    default: Some(10u32)
                }),
                Either::U32Element(U32Element {
                    key: String::from("SystemResponsiveness"),
                    value: 0u32, // value of 0 is also treated as 10 // https://docs.microsoft.com/en-us/windows/win32/procthread/multimedia-class-scheduler-service
                    default: Some(20u32)
                }),
            ],
        },
        // https://docs.microsoft.com/en-us/windows/win32/procthread/multimedia-class-scheduler-service
        RegTweaks {
            path: String::from(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games",
            ),
            data: vec![
                // NoLazyMode https://github.com/djdallmann/GamingPCSetup/blob/master/CONTENT/RESEARCH/WINSERVICES/README.md#q-what-the-heck-is-nolazymode-is-it-real-what-does-it-do

                Either::U32Element(U32Element {
                    key: String::from("Priority"), // The task priority. The range of values is 1 (low) to 8 (high).For tasks with a Scheduling Category of High, this value is always treated as 2.
                    value: 8u32,
                    default: Some(2u32),
                }),
            ],
        },

        // https://www.overclock.net/threads/research-on-multimedia-class-scheduler-service-mmcss.1774590/
        // The most commonly requested task is Audio, this will occur naturally when Windows applications make requests to Microsofts High Level Apis for Audio playback.
        RegTweaks {
            path: String::from(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Pro Audio",
            ),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("Priority"),
                    value: 8u32,
                    default: Some(2u32),
                }),
                Either::StringElement(StringElement {
                    key: String::from("Scheduling Category"),
                    value: String::from("Medium"),
                    default: Some(String::from("High")),
                }),
            ],
        },
        RegTweaks {
            path: String::from(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Audio",
            ),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("Priority"),
                    value: 8u32,
                    default: Some(2u32),
                }),
            ],
        },

        // https://www.overclock.net/threads/gaming-and-mouse-response-bios-optimization-guide-for-modern-pc-hardware.1433882/page-213#post-28561474
        RegTweaks {
            path: String::from(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Windows",
            ),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("DwmInputUsesIoCompletionPort "),
                    value: 0_u32,
                    default: Some(1u32)
                }),
                Either::U32Element(U32Element {
                    key: String::from("EnableDwmInputProcessing"),
                    value: 0u32,
                    default: Some(7u32)
                }),
            ],
        },

        // Turn On Windows Hardware Accelerated GPU Scheduling
        RegTweaks {
            path: String::from(
                "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
            ),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("HwSchMode"),
                    value: 2u32,
                    default: None, // TODO:
                }),
            ],
        },

        RegTweaks {
            path: String::from(
                "SYSTEM\\CurrentControlSet\\Control\\Nsi\\{eb004a03-9b1a-11d4-9123-0050047759bc}\\0",
            ), 
            data: vec![
                Either::VecElement(VecElement {
                    key: String::from("0200"),
                    value: vec![
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                    ],
                    default: None, // TODO:
                }),
                Either::VecElement(VecElement {
                    key: String::from("1700"),
                    value: vec![
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
                    ],
                    default: None, // TODO:
                }),
            ],
        },

        // https://www.speedguide.net/articles/windows-10-manual-tcpip-registry-tweaks-7507
        // https://core.ac.uk/download/pdf/288500221.pdf
        // https://www.sciencedirect.com/topics/computer-science/congestion-indicator
        RegTweaks {
            path: String::from(
                "SYSTEM\\CurrentControlSet\\Control\\Nsi\\{eb004a03-9b1a-11d4-9123-0050047759bc}\\26",
            ),
            data: vec![
                // "00000000" - Internet (this template is default in later Windows 10 builds)
                // "04000000" - InternetCustom (this template was used in earlier versions of Windows 8/10, possibly with CTCP as the default CongestionProvider)
                Either::VecElement(VecElement {
                    key: String::from("00000000"),
                    // 00 - none
                    // 01 - NEWRENO
                    // 02 - CTCP
                    // 03 - DCTCP
                    // 04 - LEDBAT (where available)
                    // 05 - CUBIC
                    value: vec![
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00,
                    ],
                    default: None,
                }),
            ],
        },

        // https://docs.google.com/document/d/1c2-lUJq74wuYK1WrA_bIvgb89dUN0sj8-hO3vqmrau4/edit#
        // Open regedit and go to:
        // [HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control\PriorityControl]
        // Add together the decimal values you want and enter that as a decimal to the Win32PrioritySeparation key. Example: 32+4+2. (You cannot use the third column unless you use variable quantum. If you are using fixed quantum, ignore the third column.)
        // Decimal 40 theoretically would provide the most responsive input at the expense of smoothness and FPS (short, fixed, no boost). Decimal 22 should provide the smoothest gameplay. Dec 37 is a mix between 40 and 38. There is no set answer here, so feel free to try out lots of options. There is no restart required so you can leave regedit open and keep trying different values while having your game open.
        // Possible options: decimal 21, 22, 24, 37, 38, 40
        RegTweaks {
            path: String::from(
                "SYSTEM\\CurrentControlSet\\Control\\PriorityControl",
            ),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("Win32PrioritySeparation"),
                    value: 22u32, // 01 01 10 Longer intervals, Variable-length intervals, 3 : 1. The threads of foreground processes three times as much processor time than the threads of background.
                    default: Some(2u32),
                }),
            ],
        },

        // RegTweaks {
        //     path: String::from(
        //         "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        //     ),
        //     data: vec![
        //         Either::U32Element(U32Element {
        //             key: String::from("LargeSystemCache"),
        //             value: 0u32, // 0 is default
        //         }),
        //     ],
        // },

//         RegTweaks {
//             path: String::from(
//                 "SYSTEM\\CurrentControlSet\\Services\\AFD\\parameters", // uninstall delete keys
//             ),
//             data: vec![
//                 // https://answers.sap.com/questions/75047/do-you-configure-datagram-size-to-1500-according-t.html
//                 Either::U32Element(U32Element {
//                     key: String::from("FastSendDatagramThreshold"),
//                     // value: 1024u32,
//                     value: 16384u32,
//                     // value: *mtu,
//                     default: None,
//                 }),

//                 // When an application posts a receive with a buffer that is smaller than the current packet being buffered by Winsock, AFD can either make an additional copy of the packet and then copy data to the application buffers directly (two-stage copy because application buffers cannot be accessed directly under the lock), or it can lock and map application buffers and copy data once. This value represents a compromise between extra code execution for data copying, and extra code execution in the I/O subsystem and memory manager.
//                 Either::U32Element(U32Element {
//                     key: String::from("FastCopyReceiveThreshold"),
//                     // value: 1024u32,
//                     value: 16384u32,
//                     // value: *mtu,
//                     default: None,
//                 }),

// // https://github.com/danskee/AutoTweakingUtility/blob/b421397414d204a804e51a6d3d7b9a8413a56364/NetworkTweaksDialog.cs
// // https://github.com/sale1977/WindowsUnity/blob/main/SetupComplete.cmd
// // https://github.com/ArtanisInc/Post-Tweaks/blob/7bce01d1ebe9b5f61f7909139d067da03365dc25/PostTweaks.bat
// // https://github.com/SmurfsCC/FPS-Booster-/blob/8ce6c086407ffcd1aac26b17a560cdb9b6310618/11%20-%20Regedit%20FPS/11%20-%20DesktopWin10.reg
//                 // Either::U32Element(U32Element {
//                 //     key: String::from("DisableRawSecurity"),
//                 //     value: 1u32,
//                 //     default: None,
//                 // }),
//                 // Either::U32Element(U32Element {
//                 //     key: String::from("DynamicSendBufferDisable"),
//                 //     value: 0u32,
//                 //     default: None,
//                 // }),
//                 // Either::U32Element(U32Element {
//                 //     key: String::from("IrpStackSize"),
//                 //     value: 50u32,
//                 //     default: None,
//                 // }),
//                 // Either::U32Element(U32Element {
//                 //     key: String::from("PriorityBoost"),
//                 //     value: 0u32,
//                 //     default: None,
//                 // }),
//                 // Either::U32Element(U32Element {
//                 //     key: String::from("DoNotHoldNicBuffers"),
//                 //     value: 1u32,
//                 //     default: None,
//                 // }),
//             ],
//         },

        // The LanmanServer service allows your computer to share files and printers with other devices on your network.
        // RegTweaks {
        //     path: String::from(
        //         "SYSTEM\\CurrentControlSet\\Services\\LanmanServer\\parameters",
        //     ),
        //     data: vec![
        //         // https://docs.mellanox.com/display/winof2/Performance+Tuning
        //         // In order to improve live migration over SMB direct performance, please set the following registry key to 0 and reboot the machine:
        //         Either::U32Element(U32Element {
        //             key: String::from("RequireSecuritySignature"),
        //             value: 0u32,
        //         }),

        //         // Specifies the size of the request buffers that the Server service uses.
        //         // Small buffers use less memory, but large buffers can improve performance.
        //         // For computers running Windows Server 2003 and with 512 MB or more of physical memory, the default size of the request buffers is 16,644 bytes; for servers with less physical memory, the default size is 4,356 bytes. If this entry is present in the registry, its value overrides the default value.
        //         // https://docs.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2003/cc740106(v=ws.10)
        //         // Either::U32Element(U32Element {
        //         //     key: String::from("RequireSecuritySignature"),
        //         //     value: 16384u32,
        //         // }),
        //     ],
        // },

        // RegTweaks { // https://docs.microsoft.com/en-us/windows-server/administration/performance-tuning/role/file-server/#client-tuning-example
        //     path: String::from(
        //         "SYSTEM\\CurrentControlSet\\Services\\lanmanworkstation\\parameters",
        //     ),
        //     data: vec![
        //         Either::U32Element(U32Element {
        //             key: String::from("DisableBandwidthThrottling"),
        //             value: 1u32,
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("FileInfoCacheEntriesMax"),
        //             value: 32768u32,
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("DirectoryCacheEntriesMax"),
        //             value: 4096u32,
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("FileNotFoundCacheEntriesMax"),
        //             value: 32768u32,
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("MaxCmds"),
        //             value: 32768u32,
        //         }),
        //     ],
        // },

        RegTweaks {
            path: String::from(
                "SYSTEM\\CurrentControlSet\\Services\\Ndis\\Parameters",
            ),
            data: vec![
                // The RSS base CPU number is the CPU number of the first CPU that RSS can use. RSS cannot use the CPUs that are numbered below the base CPU number. For example, on a quad-core system with hyper-threading turned off, if base CPU number is set to 1, processors 1, 2, and 3 can be used for RSS.
                // https://docs.microsoft.com/de-de/windows-hardware/drivers/network/reserving-processors-for-applications
                Either::U32Element(U32Element {
                    key: String::from("RssBaseCpu"),
                    value: 2u32,
                    default: None,
                }),

                Either::U32Element(U32Element {
                    key: String::from("MaxNumRssCpus"),
                    value: 2u32,
                    default: None, // TODO: 
                }),
                // TODO: MaxNumRssCpus
                // https://docs.microsoft.com/en-us/windows-hardware/drivers/network/setting-the-number-of-rss-processors

            ],
        },

        // RegTweaks {
        //     path: String::from(
        //         "SYSTEM\\CurrentControlSet\\Services\\nvlddmkm",
        //     ),
        //     data: vec![
        //         // The RSS base CPU number is the CPU number of the first CPU that RSS can use. RSS cannot use the CPUs that are numbered below the base CPU number. For example, on a quad-core system with hyper-threading turned off, if base CPU number is set to 1, processors 1, 2, and 3 can be used for RSS.
        //         // https://forums.blurbusters.com/viewtopic.php?t=7323
        //         Either::U32Element(U32Element {
        //             key: String::from("DisableWriteCombining"),
        //             value: 1u32,
        //             default: None,
        //         }),

        //     ],
        // },

        RegTweaks { // https://www.speedguide.net/articles/host-resolution-priority-tweak-1130
            path: String::from(
                "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters",
            ),
            data: vec![
                // https://docs.microsoft.com/de-de/troubleshoot/windows-client/networking/tcpip-and-nbt-configuration-parameters+
                // Description: This parameter determines the number of times that TCP retransmits a connect request (SYN) before aborting the attempt. The retransmission timeout is doubled with each successive retransmission in a particular connect attempt. The initial timeout value is three seconds.
                Either::U32Element(U32Element {
                    key: String::from("TcpMaxConnectRetransmissions"),
                    value: 1u32,
                    default: Some(2u32),
                }),

                // MSS: (TcpMaxDataRetransmissions) How many times unacknowledged data is retransmitted (3 recommended, 5 is default)
                // https://admx.help/?Category=SecurityBaseline&Policy=Microsoft.Policies.MSS::Pol_MSS_TcpMaxDataRetransmissions
                // Description: This parameter determines the number of times that TCP retransmits a connect request (SYN) before aborting the attempt. The retransmission timeout is doubled with each successive retransmission in a particular connect attempt. The initial timeout value is three seconds.
                // https://docs.microsoft.com/de-de/troubleshoot/windows-client/networking/tcpip-and-nbt-configuration-parameters+
                Either::U32Element(U32Element {
                    key: String::from("TcpMaxDataRetransmissions"),
                    value: 1u32,
                    default: None,
                }),

                // https://docs.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2003/cc757802(v=ws.10)
                // https://social.technet.microsoft.com/Forums/en-US/2343ad57-062e-48ed-b1ce-4b17f138d3c9/tcpwindowsize-change?forum=winservergen

                // https://docs.microsoft.com/en-us/previous-versions/technet-magazine/cc162519(v=msdn.10)?redirectedfrom=MSDN
                Either::U32Element(U32Element {
                    key: String::from("TcpWindowSize"),
                    value: 65535u32,
                    default: Some(14674u32),
                }),

                // Disable TCP selective acks option for better CPU utilization:
                // https://docs.mellanox.com/display/winof2/Performance+Tuning
                Either::U32Element(U32Element {
                    key: String::from("SackOpts"),
                    value: 0u32,
                    default: Some(1u32),
                }),

                // Tcp1323Opts is a necessary setting in order to enable Large TCP Window support as described in RFC 1323. Without this parameter, the TCP Window is limited to 64K.
                // https://www.speedguide.net/articles/windows-2kxp-registry-tweaks-157
                // https://docs.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2003/cc757402(v=ws.10)
                Either::U32Element(U32Element {
                    key: String::from("Tcp1323Opts"), // Window scaling is enabled.
                    value: 1u32,
                    default: Some(1u32),
                }),

                // https://docs.microsoft.com/en-us/windows-hardware/drivers/network/using-registry-values-to-enable-and-disable-task-offloading
                // http://systemmanager.ru/win2k_regestry.en/94176.htm
                Either::U32Element(U32Element {
                    key: String::from("DisableTaskOffload"),
                    value: 0u32, // TODO: Default ist 0 also... könnte das auch weg
                    default: None,
                }),

                Either::U32Element(U32Element {
                    key: String::from("DefaultTTL"),
                    value: 64u32,
                    default: None,
                }),

                // https://docs.microsoft.com/en-us/biztalk/technical-guides/settings-that-can-be-modified-to-improve-network-performance
                Either::U32Element(U32Element {
                    key: String::from("TcpTimedWaitDelay"),
                    value: 30u32,
                    default: None,
                }),

                // Enable fast datagram sending for UDP traffic:
                // https://docs.mellanox.com/display/winof2/Performance+Tuning
                // Either::U32Element(U32Element {
                //     key: String::from("FastSendDatagramThreshold"),
                //     value: 64000u32,
                // }),

                // Set RSS parameters:
                // https://docs.mellanox.com/display/winof2/Performance+Tuning
                // Either::U32Element(U32Element {
                //     key: String::from("RssBaseCpu"),
                //     value: 1u32,
                // }),
            ],
        },

        // What we're aiming to do is increase the priority of the last 4 settings, while keeping their order.
        // The valid range is from -32768 to +32767 and lower numbers mean higher priority compared to other services. What we're aiming at is lower numbers without going to
        // RegTweaks { // https://www.speedguide.net/articles/host-resolution-priority-tweak-1130
        //     path: String::from(
        //         "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\ServiceProvider",
        //     ),
        //     data: vec![
        //         Either::U32Element(U32Element {
        //             key: String::from("LocalPriority"),
        //             value: 4u32,
        //             default: Some(499u32),
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("HostsPriority"),
        //             value: 5u32,
        //             default: Some(500u32),
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("DnsPriority"),
        //             value: 6u32,
        //             default: Some(2000u32),
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("NetbtPriority"),
        //             value: 7u32,
        //             default: Some(2001u32),
        //         }),
        //     ],
        // },

        // https://docs.microsoft.com/en-us/windows-hardware/drivers/display/changing-the-behavior-of-the-gpu-scheduler-for-debugging
        RegTweaks {
            path: String::from(
                "SYSTEM\\ControlSet001\\Control\\GraphicsDrivers\\Scheduler",
            ),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("EnablePreemption"),
                    value: 0u32,
                    default: None,
                }),
            ],
        },
    ];

    let current_user = vec![
        RegTweaks {
            path: String::from("Control Panel\\Mouse"),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("MouseSensitivity"), // @6-of-11
                    value: 10u32,
                    default: None,
                }),
                Either::VecElement(VecElement {
                    key: String::from("SmoothMouseXCurve"),
                    value: get_smooth_mouse_x_curve(*dpi),
                    default: Some(vec![
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x6e, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x29, 0xdc, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ]),
                }),
                Either::VecElement(VecElement {
                    key: String::from("SmoothMouseYCurve"),
                    value: get_smooth_mouse_y_curve(*dpi),
                    default: Some(vec![
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x11, 0x01, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0xfc, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xbb, 0x01,
                        0x00, 0x00, 0x00, 0x00,
                    ]),
                }),
            ],
        },
        RegTweaks {
            path: String::from("Software\\Microsoft\\GameBar"),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("AllowAutoGameMode"),
                    value: 0u32,
                    default: None,
                }),
                Either::U32Element(U32Element {
                    key: String::from("ShowStartupPanel"),
                    value: 0u32,
                    default: None,
                }),
                Either::U32Element(U32Element {
                    key: String::from("GamePanelStartupTipIndex"),
                    value: 3u32,
                    default: None,
                }),
                Either::U32Element(U32Element {
                    key: String::from("UseNexusForGameBarEnabled"),
                    value: 0u32,
                    default: None,
                }),
            ],
        },

        RegTweaks {
            path: String::from("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Serialize"),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("StartupDelayInMSec"),
                    value: 0u32,
                    default: None,
                }),
            ],
        },

        // RegTweaks {
        //     path: String::from(
        //         "Software\\Microsoft\\GameBar",
        //     ),
        //     data: vec![
        //         Either::U32Element(U32Element {
        //             key: String::from("ShowStartupPanel"),
        //             value: 0u32,
        //             default: None, // TODO: 
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("GamePanelStartupTipIndex"),
        //             value: 3u32,
        //             default: None, // TODO: 
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("AllowAutoGameMode"),
        //             value: 0u32,
        //             default: None, // TODO: 
        //         }),
        //         Either::U32Element(U32Element {
        //             key: String::from("UseNexusForGameBarEnabled"),
        //             value: 0u32,
        //             default: None, // TODO: 
        //         }),
        //     ],
        // },

        RegTweaks {
            path: String::from(
                "Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
            ),
            data: vec![
                Either::U32Element(U32Element {
                    key: String::from("AppCaptureEnabled"),
                    value: 0u32,
                    default: None, // TODO: 
                }),
            ],
        },

        // The Road to Fullscreen Optimizations
        // https://devblogs.microsoft.com/directx/demystifying-full-screen-optimizations/
        RegTweaks {
            path: String::from(
                "SYSTEM\\GameConfigStore",
            ),
            data: vec![
                // Disable Xbox Features
                Either::U32Element(U32Element {
                    key: String::from("GameDVR_Enabled"),
                    value: 0u32,
                    default: None, // TODO: 
                }),
                // Disable Fullscreen optimizations
                Either::U32Element(U32Element {
                    key: String::from("GameDVR_DSEBehavior"),
                    value: 2u32,
                    default: None, // TODO: 
                }),
                // Disable Fullscreen optimizations
                Either::U32Element(U32Element {
                    key: String::from("GameDVR_FSEBehaviorMode"),
                    value: 2u32,
                    default: None, // TODO: 
                }),
                // Disable Fullscreen optimizations
                Either::U32Element(U32Element {
                    key: String::from("GameDVR_FSEBehavior"),
                    value: 2u32,
                    default: None, // TODO: 
                }),
                // Disable Fullscreen optimizations
                Either::U32Element(U32Element {
                    key: String::from("GameDVR_HonorUserFSEBehaviorMode"),
                    value: 1u32,
                    default: None, // TODO: 
                }),
                // Disable Fullscreen optimizations
                Either::U32Element(U32Element {
                    key: String::from("GameDVR_DXGIHonorFSEWindowsCompatible"),
                    value: 1u32,
                    default: None, // TODO: 
                }),
                // Disable Fullscreen optimizations
                Either::U32Element(U32Element {
                    key: String::from("GameDVR_EFSEFeatureFlags"),
                    value: 0u32,
                    default: None, // TODO: 
                }),
            ],
        },
    ];
    // TODO let users https://github.com/spddl/apex-optimizer/blob/master/src/registry/mousefix.rs#L50
    // [HKEY_USERS\.DEFAULT\Control Panel\Mouse]

    let users = vec![RegTweaks {
        path: String::from(".DEFAULT\\Control Panel\\Mouse"),
        data: vec![
            Either::U32Element(U32Element {
                key: String::from("MouseSpeed"),
                value: 0u32,
                default: None,
            }),
            Either::U32Element(U32Element {
                key: String::from("MouseThreshold1"),
                value: 0u32,
                default: None,
            }),
            Either::U32Element(U32Element {
                key: String::from("MouseThreshold2"),
                value: 0u32,
                default: None,
            }),
        ],
    }];

    Settings {
        local_machine,
        current_user,
        users,
    }
}

#[allow(dead_code)]
pub fn factory_powerplan() -> PowerPlan {
// Computer\HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control\Power\User\PowerSchemes\2cd4d4f0-d578-48f4-be43-d145b9b71cbe\54533251-82be-4824-96c1-47b60b740d00\0cc5b647-c1df-4637-891a-dec35c318583
// ACSettingIndex 0x64 (100)

// Computer\HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control\Power\User\PowerSchemes\2cd4d4f0-d578-48f4-be43-d145b9b71cbe\54533251-82be-4824-96c1-47b60b740d00\45bcc044-d885-43e2-8605-ee0ec6e96b59
// ACSettingIndex 0x64 (100)

// Computer\HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control\Power\User\PowerSchemes\2cd4d4f0-d578-48f4-be43-d145b9b71cbe\54533251-82be-4824-96c1-47b60b740d00\893dee8e-2bef-41e0-89c6-b55d0929964c
// ACSettingIndex 0x64 (100)

// https://docs.microsoft.com/en-us/windows/win32/api/powrprof/nf-powrprof-powercreatesetting
    PowerPlan{ // https://bitsum.com/known-windows-power-guids/
        // scheme: None,
        scheme: Some(String::from("8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c")), // High performance
        data: vec![
            SubGuid{
                path: String::from("54533251-82be-4824-96c1-47b60b740d00"), // SUB_PROCESSOR
                data: vec![
                    // Makes maximum CPU speeds available, by default they're not
                    SettingGuid{
                        path: String::from("be337238-0d82-4146-a960-4f3749d470c7"), // PERFBOOSTMODE
                        data: 0u32,
                    },
                    SettingGuid{
                        path: String::from("06cadf0e-64ed-448a-8927-ce7bf90eb35d"), // PERFINCTHRESHOLD (Processor performance increase threshold)
                        data: 1u32,
                    },
                    SettingGuid{
                        path: String::from("06cadf0e-64ed-448a-8927-ce7bf90eb35e"), // PERFINCTHRESHOLD1 (Processor performance increase threshold for Processor Power Efficiency Class 1)
                        data: 1u32,
                    },
                    SettingGuid{
                        path: String::from("984cf492-3bed-4488-a8f9-4286c97bf5aa"), // PERFINCTIME (Processor performance increase time)
                        data: 1u32,
                    },
                    SettingGuid{
                        path: String::from("984cf492-3bed-4488-a8f9-4286c97bf5ab"), // PERFINCTIME1 (Processor performance increase time for Processor Power Efficiency Class 1)
                        data: 1u32,
                    },
                    SettingGuid{
                        path: String::from("12a0ab44-fe28-4fa9-b3bd-4b64f44960a6"), // PERFDECTHRESHOLD (Processor performance decrease threshold)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("12a0ab44-fe28-4fa9-b3bd-4b64f44960a7"), // PERFDECTHRESHOLD1 (Processor performance decrease threshold for Processor Power Efficiency Class 1)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("619b7505-003b-4e82-b7a6-4dd29c300971"), // LATENCYHINTPERF (Latency sensitivity hint processor performance)
                        data: 0u32,
                    },
                   SettingGuid{
                        path: String::from("619b7505-003b-4e82-b7a6-4dd29c300972"), // LATENCYHINTPERF1 (Latency sensitivity hint processor performance for Processor Power Efficiency Class 1)
                        data: 0u32,
                    },
                    SettingGuid{
                        path: String::from("8baa4a8a-14c6-4451-8e8b-14bdbd197537"), // PERFAUTONOMOUS (Processor performance autonomous mode)
                        data: 0u32,
                    },
                    SettingGuid{
                        path: String::from("4e4450b3-6179-4e91-b8f1-5bb9938f81a1"), // PERFDUTYCYCLING (Processor duty cycling)
                        data: 0u32,
                    },

                    // Sets overall throttles to maximum
                    SettingGuid{
                        path: String::from("bc5038f7-23e0-4960-96da-33abaf5935ec"), // PROCTHROTTLEMAX (Maximum processor state)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("bc5038f7-23e0-4960-96da-33abaf5935ed"), // PROCTHROTTLEMAX1 (Maximum processor state for Processor Power Efficiency Class 1)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("893dee8e-2bef-41e0-89c6-b55d0929964c"), // PROCTHROTTLEMIN (Minimum processor state)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("893dee8e-2bef-41e0-89c6-b55d0929964d"), // PROCTHROTTLEMIN1 (Minimum processor state for Processor Power Efficiency Class 1)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("1facfc65-a930-4bc5-9f38-504ec097bbc0"), // HETEROCLASS1INITIALPERF (Initial performance for Processor Power Efficiency Class 1 when unparked)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("fddc842b-8364-4edc-94cf-c17f60de1c80"), // HETEROCLASS0FLOORPERF (A floor performance for Processor Power Efficiency Class 0 when there are Processor Power Efficiency Class 1 processors unparked)
                        data: 100u32,
                    },

                    // Turns off CPU core controls, tells OS to just use them all.
                    SettingGuid{
                        path: String::from("ea062031-0e34-4ff1-9b6d-eb1059334028"), // CPMAXCORES (Processor performance core parking max cores)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("ea062031-0e34-4ff1-9b6d-eb1059334029"), // CPMAXCORES1 (Processor performance core parking max cores for Processor Power Efficiency Class 1)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("0cc5b647-c1df-4637-891a-dec35c318583"), // CPMINCORES (Processor performance core parking min cores)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("0cc5b647-c1df-4637-891a-dec35c318584"), // CPMINCORES1 (Processor performance core parking min cores for Processor Power Efficiency Class 1)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("e0007330-f589-42ed-a401-5ddb10e785d3"), // DISTRIBUTEUTIL (Processor performance core parking utility distribution)
                        data: 0u32,
                    },
                    SettingGuid{
                        path: String::from("4bdaf4e9-d103-46d7-a5f0-6280121616ef"), // CPDISTRIBUTION (Processor performance core parking distribution threshold)
                        data: 1u32,
                    },

                    // Minimizes CPU spinup time, and maximizes spindown time, just in case
                    SettingGuid{
                        path: String::from("2ddd5a84-5a71-437e-912a-db0b8c788732"), // CPINCREASETIME (Processor performance core parking increase time)
                        data: 1u32,
                    },
                    SettingGuid{
                        path: String::from("dfd10d17-d5eb-45dd-877a-9a34ddd15c82"), // CPDECREASETIME (Processor performance core parking decrease time)
                        data: 100u32,
                    },
                    SettingGuid{
                        path: String::from("f735a673-2066-4f80-a0c5-ddee0cf1bf5d"), // CPHEADROOM (Processor performance core parking concurrency headroom threshold)
                        data: 1u32,
                    },
                    SettingGuid{
                        path: String::from("2430ab6f-a520-44a2-9601-f7f23b5134b1"), // CPCONCURRENCY (Processor performance core parking concurrency threshold)
                        data: 1u32,
                    },
                    SettingGuid{
                        path: String::from("616cdaa5-695e-4545-97ad-97dc2d1bdd88"), // LATENCYHINTUNPARK (Latency sensitivity hint min unparked cores/packages)
                        data: 1u32,
                    },
                    SettingGuid{
                        path: String::from("616cdaa5-695e-4545-97ad-97dc2d1bdd89"), // LATENCYHINTUNPARK1 (Latency sensitivity hint min unparked cores/packages for Processor Power Efficiency Class 1)
                        data: 1u32,
                    },

                    // Sets energy savings preference to zero
                    SettingGuid{
                        path: String::from("36687f9e-e3a5-4dbf-b1dc-15eb381c6863"), // PERFEPP (Processor energy performance preference policy)
                        data: 0u32,
                    },

                    // SettingGuid{
                    //     path: String::from("45bcc044-d885-43e2-8605-ee0ec6e96b59"), // PERFBOOSTPOL (Processor performance boost policy)
                    //     data: 100u32,
                    // },
                    // SettingGuid{
                    //     path: String::from("893dee8e-2bef-41e0-89c6-b55d0929964c"), // PROCTHROTTLEMIN (Minimum processor state)
                    //     data: 100u32,
                    // },
                    // SettingGuid{
                    //     path: String::from("06cadf0e-64ed-448a-8927-ce7bf90eb35d"), // PERFINCTHRESHOLD (Processor performance increase threshold)
                    //     data: 1u32,
                    // },
                    // SettingGuid{
                    //     path: String::from("06cadf0e-64ed-448a-8927-ce7bf90eb35e"), // PERFINCTHRESHOLD1 (Processor performance increase threshold for Processor Power Efficiency Class 1)
                    //     data: 1u32,
                    // },
                    // SettingGuid{
                    //     path: String::from("2ddd5a84-5a71-437e-912a-db0b8c788732"), // CPINCREASETIME (Processor performance core parking increase time)
                    //     data: 100u32,
                    // },
                 ],
            },
        ],
    }
}

// pub fn default_powerplan() {
//     powerplan::default();
// }

#[allow(dead_code)]
pub fn check_powerplan(powerplan: &PowerPlan, write_settings: bool) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // Computer\HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Power\User\PowerSchemes
    // Computer\HKEY_LOCAL_MACHINE\SYSTEM\ControlSet001\Control\Power\User\PowerSchemes
    // ActivePowerScheme
    let power_scheme: RegKey= hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes").unwrap();
    let scheme_guid: String;


    match &powerplan.scheme {
        None => {
            scheme_guid = power_scheme.get_value("ActivePowerScheme").unwrap();
            // println!("active_power_scheme = {}", scheme_guid);

            let power_scheme_name = power_scheme.open_subkey(&scheme_guid).unwrap();
            let scheme_name: String = power_scheme_name.get_value("FriendlyName").unwrap();
            println!("Active Powerplan = {} {}", scheme_name, scheme_guid); // 	8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c
        },
        Some(scheme) => {
            scheme_guid = scheme.to_string();
            println!("Active Powerplan = {}", scheme_guid); // 	8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c
        },
    }
    // if powerplan.scheme.is_none() {
    //     scheme_guid = power_scheme.get_value("ActivePowerScheme").unwrap();
    //     // println!("active_power_scheme = {}", scheme_guid);

    //     let power_scheme_name = power_scheme.open_subkey(&scheme_guid).unwrap();
    //     let scheme_name: String = power_scheme_name.get_value("FriendlyName").unwrap();
    //     println!("Active Powerplan = {} {}", scheme_name, scheme_guid); // 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c
    // } else {
    //     scheme_guid = powerplan.scheme.unwrap();
    // }

    let scheme_guid_reg = power_scheme.open_subkey(&scheme_guid);
    if scheme_guid_reg.is_err() {
        println!("scheme_guidis_err");
    }
    let scheme_guid_reg = scheme_guid_reg.unwrap();

    //  scheme_GUID
    //   Specifies a power scheme GUID. A power scheme GUID is returned by running powercfg /list.

    //  sub_GUID
    //   Specifies a power setting subgroup GUID. Running powercfg /query returns a power setting subgroup GUID.

    //  setting_GUID
    //   Specifies a power setting GUID. A power setting GUID is returned by running powercfg /query.
    
    //  setting_index
    //   Specifies which possible value this setting is set to. A list of possible values is returned by running powercfg /query.

    // Computer\HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Power\User\PowerSchemes\2cd4d4f0-d578-48f4-be43-d145b9b71cbe\54533251-82be-4824-96c1-47b60b740d00\0cc5b647-c1df-4637-891a-dec35c318583
    // ACSettingIndex 0x64 (100)

    for sub_guid in powerplan.data.iter() {
        let sub_guid_path = scheme_guid_reg.open_subkey(sub_guid.path.clone());
        if sub_guid_path.is_err() {
            // println!("setting missing: \x1b[0;93m{}\x1b[0m", format!("HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes\\{}\\{}", scheme_guid, sub_guid.path));

            for setting_guid in sub_guid.data.iter() {
                println!("setting missing: \x1b[0;93m{}\x1b[0m", format!("HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes\\{}\\{}\\{}", scheme_guid, sub_guid.path, setting_guid.path));

                if write_settings {
                    powerplan::set(&scheme_guid, &sub_guid.path, &setting_guid.path, &setting_guid.data);
                }
            }
        } else {
            let sub_guid_path = sub_guid_path.unwrap();
            for setting_guid in sub_guid.data.iter() {
                let setting_guid_path = sub_guid_path.open_subkey(&setting_guid.path);
                if setting_guid_path.is_err() {
                    println!("setting missing: \x1b[0;93m{}\x1b[0m", format!("HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes\\{}\\{}\\{}", scheme_guid, sub_guid.path, setting_guid.path));
                    if write_settings {
                        powerplan::set(&scheme_guid, &sub_guid.path, &setting_guid.path, &setting_guid.data);
                    }
                    continue
                }

                match setting_guid_path.unwrap().get_value("ACSettingIndex") {
                    Err(e) => {
                        println!("setting missing (ACSettingIndex): \x1b[0;93m{}\x1b[0m",e);
                    }
                    Ok(reg_val) => {
                        let value: u32 = reg_val;
                        if value == setting_guid.data {
                            println!("correct setting: {}", format!("HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes\\{}\\{}\\{}\\ACSettingIndex = dword:{}", scheme_guid, sub_guid.path, setting_guid.path, setting_guid.data));
                        } else {
                            println!(
                                "wrong setting: \x1b[0;93m{} = dword:{}\x1b[0m (your value: {})",
                                format!("HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes\\{}\\{}\\{}\\ACSettingIndex", scheme_guid, sub_guid.path, setting_guid.path)
                                , setting_guid.data, value
                            );
                            if write_settings {
                                powerplan::set(&scheme_guid, &sub_guid.path, &setting_guid.path, &setting_guid.data);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_reg_tweaks(reg_settings: &Settings, write_settings: bool) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for section in reg_settings.local_machine.iter() {
        let reg_t = Transaction::new().unwrap();
        let (reg, _) =
            hklm.create_subkey_transacted_with_flags(&section.path, &reg_t, KEY_ALL_ACCESS).unwrap();
        let reg_path = format!("HKEY_LOCAL_MACHINE\\{}", section.path);

        for data in section.data.iter() {
            match data {
                Either::StringElement(ele) => {
                    set_str_reg(&reg, &ele.key, &ele.value, &reg_path, write_settings)
                }
                Either::U32Element(ele) => {
                    set_u32_reg(&reg, &ele.key, &ele.value, &reg_path, write_settings)
                }
                Either::VecElement(ele) => {
                    set_vac_reg(&reg, &ele.key, ele.value.clone(), &reg_path, write_settings)
                }
            }
        }
        if write_settings {
            reg_t.commit().unwrap();
        }
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for section in reg_settings.current_user.iter() {
        let reg_t = Transaction::new().unwrap();
        let (reg, _) =
            hkcu.create_subkey_transacted_with_flags(&section.path, &reg_t, KEY_ALL_ACCESS).unwrap();
        let reg_path = format!("HKEY_CURRENT_USER\\{}", section.path);

        for data in section.data.iter() {
            match data {
                Either::StringElement(ele) => {
                    set_str_reg(&reg, &ele.key, &ele.value, &reg_path, write_settings)
                }
                Either::U32Element(ele) => {
                    set_u32_reg(&reg, &ele.key, &ele.value, &reg_path, write_settings)
                }
                Either::VecElement(ele) => {
                    set_vac_reg(&reg, &ele.key, ele.value.clone(), &reg_path, write_settings)
                }
            }
        }

        if write_settings {
            reg_t.commit().unwrap();
        }
    }

    let hkus = RegKey::predef(HKEY_USERS);
    for section in reg_settings.users.iter() {
        let reg_t = Transaction::new().unwrap();
        let (reg, _) =
            hkus.create_subkey_transacted_with_flags(&section.path, &reg_t, KEY_ALL_ACCESS).unwrap();
        let reg_path = format!("HKEY_USERS\\{}", section.path);

        for data in section.data.iter() {
            match data {
                Either::StringElement(ele) => {
                    set_str_reg(&reg, &ele.key, &ele.value, &reg_path, write_settings)
                }
                Either::U32Element(ele) => {
                    set_u32_reg(&reg, &ele.key, &ele.value, &reg_path, write_settings)
                }
                Either::VecElement(ele) => {
                    set_vac_reg(&reg, &ele.key, ele.value.clone(), &reg_path, write_settings)
                }
            }
        }

        if write_settings {
            reg_t.commit().unwrap();
        }
    }
}

pub fn apply_tcp_tweaks(
    mtu: &u32,
    write_settings: bool,
    default_settings: bool,
) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let nics = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces",
        KEY_READ,
    ).unwrap();

    let addr_type_value = RegValue {
        bytes: vec![0u8, 0, 0, 0],
        vtype: REG_DWORD,
    };

    let mut nic_key = None;
    for maybe_nic in nics.enum_keys() {
        let nic_id = maybe_nic.unwrap();
        let nic = nics.open_subkey_with_flags(nic_id.clone(), KEY_READ).unwrap();
        if nic
            .enum_values()
            .filter(|r| match r {
                Ok((k, v)) => k == "AddressType" && *v == addr_type_value,
                _ => false,
            })
            .count()
            > 0
        {
            nic_key = Some(nic_id);
            break;
        }
    }

    if let Some(nic_id) = nic_key {
        let nic_t = Transaction::new().unwrap();
        let (nic, _) =
            nics.create_subkey_transacted_with_flags(nic_id.clone(), &nic_t, KEY_ALL_ACCESS).unwrap();
        let reg_path= format!("HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces\\{}", &nic_id);

        // Subkey: HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\<Interface GUID>
        // Entry: TcpAckFrequency
        // Value Type: REG_DWORD, number
        // Valid Range: 0-255
        // Default: 2
        // Description: Specifies the number of ACKs that will be outstanding before the delayed ACK timer is ignored. Microsoft does not recommend changing the default value without careful study of the environment.
        // If you set the value to 1, every packet is acknowledged immediately because there's only one outstanding TCP ACK as a segment is just received. The value of 0 (zero) isn't valid and is treated as the default, 2. The only time the ACK number is 0 when a segment isn't received and the host isn't going to acknowledge the data.
        // https://docs.microsoft.com/en-us/troubleshoot/windows-server/networking/registry-entry-control-tcp-acknowledgment-behavior
        if default_settings {
            del_key_reg(&nic, "TcpAckFrequency", &reg_path);
        } else {
            set_u32_reg(&nic, "TcpAckFrequency", &1u32, &reg_path, write_settings);
        }

        // https://support.microsoft.com/en-us/topic/fix-tcp-ip-nagle-algorithm-for-microsoft-message-queue-server-can-be-disabled-74ba2f6a-e558-d1df-1c60-57b0fab68ccc
        // set_u32_reg(&nic, "TCPNoDelay", &1u32, &nic_id, fixes, write_settings);

        // Key: Tcpip\Parameters\Interfaces\ID for Adapter
        // Value type: REG_DWORD Number
        // Valid range: 68 - the MTU of the underlying network
        // Default: 0xFFFFFFFF
        // Description: This parameter overrides the default Maximum Transmission Unit (MTU) for a network interface. The MTU is the maximum packet size in bytes that the transport transmits over the underlying network. The size includes the transport header. An IP datagram can span multiple packets. Values larger than the default value for the underlying network cause the transport to use the network default MTU. Values smaller than 68 cause the transport to use an MTU of 68.

        if default_settings {
            // let default_mtu = 1374u32;
            // set_u32_reg(&nic, "MTU", &default_mtu, &reg_path, write_settings);
            del_key_reg(&nic, "MTU", &reg_path);
        } else {
            set_u32_reg(&nic, "MTU", &mtu, &reg_path, write_settings);
        }

        // TCPNoDelay and set it also to 1 to disable “nagling”
        // TcpDelAckTicks and set it to 0

        if write_settings {
            nic_t.commit().unwrap();
        }
    } else {
        println!("\x1b[0;93mCould not find your current network interface!\x1b[0m");
    }
}

pub fn restore_default_reg(reg_settings: &Settings) -> std::io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for section in reg_settings.local_machine.iter() {
        let reg_t = Transaction::new()?;
        let (reg, _) =
            hklm.create_subkey_transacted_with_flags(&section.path, &reg_t, KEY_ALL_ACCESS)?;
        let reg_path = format!("HKEY_LOCAL_MACHINE\\{}", section.path);

        for data in section.data.iter() {
            match data {
                Either::StringElement(ele) => match &ele.default {
                    None => del_key_reg(&reg, &ele.key, &reg_path),
                    Some(val) => set_str_reg(&reg, &ele.key, &val, &reg_path, true),
                },
                Either::U32Element(ele) => match &ele.default {
                    None => del_key_reg(&reg, &ele.key, &reg_path),
                    Some(val) => set_u32_reg(&reg, &ele.key, &val, &reg_path, true),
                },
                Either::VecElement(ele) => match &ele.default {
                    None => del_key_reg(&reg, &ele.key, &reg_path),
                    Some(val) => set_vac_reg(&reg, &ele.key, val.clone(), &reg_path, true),
                },
            }
        }
        reg_t.commit()?;
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for section in reg_settings.current_user.iter() {
        let reg_t = Transaction::new()?;
        let (reg, _) =
            hkcu.create_subkey_transacted_with_flags(&section.path, &reg_t, KEY_ALL_ACCESS)?;
        let reg_path = format!("HKEY_CURRENT_USER\\{}", section.path);

        for data in section.data.iter() {
            match data {
                Either::StringElement(ele) => {
                    set_str_reg(&reg, &ele.key, &ele.value, &reg_path, true)
                }
                Either::U32Element(ele) => set_u32_reg(&reg, &ele.key, &ele.value, &reg_path, true),
                Either::VecElement(ele) => {
                    set_vac_reg(&reg, &ele.key, ele.value.clone(), &reg_path, true)
                }
            }
        }

        reg_t.commit()?;
    }

    let hkus = RegKey::predef(HKEY_USERS);
    for section in reg_settings.users.iter() {
        let reg_t = Transaction::new()?;
        let (reg, _) =
            hkus.create_subkey_transacted_with_flags(&section.path, &reg_t, KEY_ALL_ACCESS)?;
        let reg_path = format!("HKEY_USERS\\{}", section.path);

        for data in section.data.iter() {
            match data {
                Either::StringElement(ele) => {
                    set_str_reg(&reg, &ele.key, &ele.value, &reg_path, true)
                }
                Either::U32Element(ele) => set_u32_reg(&reg, &ele.key, &ele.value, &reg_path, true),
                Either::VecElement(ele) => {
                    set_vac_reg(&reg, &ele.key, ele.value.clone(), &reg_path, true)
                }
            }
        }

        reg_t.commit()?;
    }

    Ok(())
}

// TODO: control mmsys.cpl
// Computer\HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\MMDevices\Audio\Render\{d75eb062-247f-4dd3-b4d5-55fea9c4cee2}\Properties
// https://github.com/romanadamski/ChangeSoundDeviceTray/blob/268fc877622442d9f991ec670e785ebcabc51aeb/AudioSwitcher.AudioApi.CoreAudio/Internal/PropertyKeys.cs
// https://github.com/MSDN-WhiteKnight/answers/blob/7e87ccf0edd6978802964503aa4ff8efda5ece62/tools/data/ru.stackoverflow.com/posts/A743709.md
// https://stackoverflow.com/questions/52954849/enabling-recording-devices-programmatically
// https://www.pinvoke.net/default.aspx/Constants/PROPERTYKEY.html
// https://github.com/AutomatedLab/DeviceManagement/blob/master/DeviceManagementLib/Device.cs
// https://windowsreport.com/audio-device-disabled-windows-10/
// https://codemachine.com/articles/how_windows_sets_default_audio_device.html
// https://social.technet.microsoft.com/Forums/en-US/590fd01d-f27b-48db-bad4-9497474ff185/setting-playback-and-communication-device-via-registry?forum=win10itprosetup

#[allow(dead_code)]
pub fn apply_audio_settings(_write_settings: bool) -> std::io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let nics = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\MMDevices\\Audio\\Render",
        KEY_READ,
    )?;

    let addr_type_value = RegValue {
        bytes: vec![1u8, 0, 0, 0],
        vtype: REG_DWORD,
    };

    let mut soundcard_guid = Vec::new();

    for maybe_nic in nics.enum_keys() {
        let nic_id = maybe_nic?;
        let nic = nics.open_subkey_with_flags(nic_id.clone(), KEY_READ)?;

        for n in nic.enum_values() {
            match n {
                Err(e) => panic!("{:?}", e),
                Ok(ratio) => {
                    if ratio.0 == "DeviceState" && ratio.1 == addr_type_value {
                        soundcard_guid.push(nic_id.clone());
                    }
                }
            }
        }
    }

    for guid in soundcard_guid.iter() {
        println!("soundcard_guid HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\MMDevices\\Audio\\Render\\{}\\Properties", guid);

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm.open_subkey(format!(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\MMDevices\\Audio\\Render\\{}\\Properties",
            guid
        ))?;

        let bus: String = key.get_value("{b3f8fa53-0004-438e-9003-51a46e139bfc},2")?;
        let name: String = key.get_value("{b3f8fa53-0004-438e-9003-51a46e139bfc},6")?;
        // let deviceNameKey: String = key.get_value("{a45c254e-df1c-4efd-8020-67d146a850e0},2")?; // getOutputDevice(guid):

        println!("Name: {}, Bus: {}", name, bus)
    }

    Ok(())
}

pub fn apply_get_dpi() -> u32 {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey("Control Panel\\Desktop");

    if key.is_ok() {
        let regkey = key.unwrap();
        let win8_dpi_scaling: u32 = regkey.get_value("Win8DpiScaling").unwrap();
        if win8_dpi_scaling == 0u32 {
            96u32
        } else {
            // DPI	Scale factor // https://docs.microsoft.com/en-us/windows-hardware/manufacture/desktop/dpi-related-apis-and-registry-settings
            // 96	100
            // 120	125
            // 144	150
            // 192	200
            let log_pixels: u32 = regkey.get_value("LogPixels").unwrap();
            log_pixels
        }
    } else {
        96u32
    }
}

pub struct CpuPriority {
    pub process: String,
    pub cpu_priority_class: Option<u32>,
    pub io_priority: Option<u32>,
    pub page_priority: Option<u32>,
    pub working_set_limit_in_kb: Option<u32>,
}

pub fn set_cpu_priority(data: CpuPriority, write_settings: bool) {
    let ifeo = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options",
    ).unwrap();

    let (perf_options, _) = ifeo.create_subkey(format!("{}\\PerfOptions", data.process)).unwrap();
    if data.cpu_priority_class.is_some() {
        set_u32_reg(&perf_options, &"CpuPriorityClass", &data.cpu_priority_class.unwrap(), &format!("{}\\PerfOptions", data.process), write_settings);
    }
    if data.io_priority.is_some() {
        if write_settings {
            perf_options.set_value("IoPriority", &data.io_priority.unwrap()).unwrap();
        }
    }
    if data.page_priority.is_some() {
        if write_settings {
            perf_options.set_value("PagePriority", &data.page_priority.unwrap()).unwrap();
        }
    }
    if data.working_set_limit_in_kb.is_some() {
        if write_settings {
            perf_options.set_value("WorkingSetLimitInKB", &data.working_set_limit_in_kb.unwrap()).unwrap();
        }
    }

    // "CpuPriorityClass"=dword:00000001
    // 00000001 = Idle
    // 00000002 = Normal (def 2)
    // 00000003 = High
    // 00000004 = RealTime (n.a.)
    // 00000005 = Below Normal
    // 00000006 = Above Normal

    // "IoPriority"=dword:00000000
    // 00000000 = Very Low
    // 00000001 = Low
    // 00000002 = Normal (def 2)
    // 00000003 = High

    // "PagePriority"=dword:00000001
    // 00000000 = 0 Idle
    // 00000001 = 1 Very Low
    // 00000002 = 2 Low
    // 00000003 = 3 Background
    // 00000004 = 4 Background
    // 00000005 = 5 Normal (def 5)

    // "WorkingSetLimitInKB"
    // (def 1382)
}

fn get_smooth_mouse_x_curve(dpi: u32) -> Vec<u8> {
    return match dpi {
        // DPI 100%
        96 => vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xCC, 0x0C, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x80, 0x99, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x66, 0x26, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        // DPI 125%
        120 => vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        // DPI 150%
        144 => vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x33, 0x13, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x60, 0x66, 0x26, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x99, 0x39, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xC0, 0xCC, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        // DPI 200%
        192 => vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x99, 0x19, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x20, 0x33, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0xB0, 0xCC, 0x4C, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x40, 0x66, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        // DPI 100%
        _ => {
            println!(
                "SmoothMouse X Curve for this DPI ({}) Setting not found",
                dpi
            );
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xCC, 0x0C, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x80, 0x99, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x66, 0x26, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        }
    };
}

fn get_smooth_mouse_y_curve(dpi: u32) -> Vec<u8> {
    return match dpi {
        // DPI 100%
        96 => vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA8, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        // DPI 125%
        120 => vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA8, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        // DPI 150%
        144 => vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA8, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        // DPI 200%
        192 => vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA8, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        // DPI 100%
        _ => {
            println!(
                "SmoothMouse Y Curve for this DPI ({}) Setting not found",
                dpi
            );
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA8, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        }
    };
}
