#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = *include_bytes!(concat!(env!("OUT_DIR"), "/boot2.bin"));
