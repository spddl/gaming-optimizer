use std::net::IpAddr;
use winping::{Buffer, Pinger};

pub fn ping() -> u32 {
    let dst = std::env::args()
        .nth(1)
        .unwrap_or(String::from("1.1.1.1"))
        .parse::<IpAddr>()
        .expect("Could not parse IP Address");

    let mut pinger = Pinger::new().unwrap();
    pinger.set_ttl(64);
    pinger.set_df(true);

    let mut done = true;
    let mut max_mtu: u32 = 1500;
    while done {
        if max_mtu < 1 {
            max_mtu = 1472; // + header = 1500
            break;
        }

        let mut buf = Buffer::new();
        buf.request_data
            .resize_with(max_mtu as usize, Default::default);

        match pinger.send(dst, &mut buf) {
            Ok(_) => done = false,
            Err(_) => max_mtu -= 1,
        }
    }
    max_mtu + 28 // Header 28 bit
}
