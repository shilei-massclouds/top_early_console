#![no_std]

pub fn init(_cpu_id: usize, _dtb_pa: usize) {
    let msg = "\n[early_console]: Hello, ArceOS!\n";
    early_console::write_bytes(msg.as_bytes());
}
