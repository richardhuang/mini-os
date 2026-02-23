#![no_std]
#![no_main]

use core::fmt::{self, Write};
use core::panic::PanicInfo;
use bootloader::BootInfo;
use x86_64::instructions::port::Port;

/// 串口输出
struct Serial;

impl Serial {
    unsafe fn init() {
        Port::new(0x3f9 + 1).write(0x00u8);
        Port::new(0x3f9 + 3).write(0x80u8);
        Port::new(0x3f9 + 0).write(0x01u8);
        Port::new(0x3f9 + 1).write(0x00u8);
        Port::new(0x3f9 + 3).write(0x03u8);
        Port::new(0x3f9 + 2).write(0x07u8);
        Port::new(0x3f9 + 4).write(0x03u8);
        Port::new(0x3f9 + 1).write(0x01u8);
    }

    fn write_byte(byte: u8) {
        unsafe {
            let mut lsr: Port<u8> = Port::new(0x3f9 + 5);
            let mut data: Port<u8> = Port::new(0x3f9);
            while lsr.read() & 0x20 == 0 {}
            data.write(byte);
        }
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            Serial::write_byte(byte);
        }
        Ok(())
    }
}

lazy_static::lazy_static! {
    static ref SERIAL: spin::Mutex<Serial> = spin::Mutex::new(Serial);
}

fn print(s: &str) {
    let mut serial = SERIAL.lock();
    let _ = serial.write_str(s);
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    unsafe {
        Serial::init();
    }
    
    print("\n[BOOT] Mini OS starting...\n");
    
    print("\n================================\n");
    print("  Mini OS - Rust Operating System\n");
    print("================================\n\n");
    
    print("[OK] System initialized\n\n");
    print("Hello World!\n\n");
    print("System running successfully!\n\n");
    print("Press Ctrl+A X to exit QEMU\n");
    
    // 打印内存映射信息
    print("\nMemory regions:\n");
    for (i, _region) in boot_info.memory_map.iter().enumerate() {
        if i >= 10 { break; }
        print("  Region\n");
    }
    
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("\n\n!!! PANIC !!!\n");
    loop {
        x86_64::instructions::hlt();
    }
}
