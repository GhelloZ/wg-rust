use tun::{Configuration, Device};
use std::io::Read;

fn main() -> std::io::Result<()> {
    // TUN configuration
    let mut config = Configuration::default();
    config.address((10,0,0,1))
        .netmask((255,255,255,0))
        .mtu(1500)
        .up();

    // Create TUN device
    let mut dev = tun::create(&config).unwrap();
    println!("TUN device created: {:?}", dev.name());

    // Buffer for reading packets
    let mut buf = [0u8; 1504];

    loop {
        let n = dev.read(&mut buf)?;
        println!("Read {} bytes: {:02x?}", n, &buf[..n]);
    }
}
