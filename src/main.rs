#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;

mod vga;
mod gdt;
mod interrupts;
mod memory;
mod fs;

/// 内核入口点
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 初始化 VGA 输出
    vga::init();
    
    // 打印欢迎信息
    println!("\n================================");
    println!("  Mini OS - Rust Operating System");
    println!("================================\n");
    
    // 初始化 GDT
    gdt::init();
    
    // 初始化中断
    interrupts::init();
    
    // 初始化内存管理
    memory::init();
    
    // 初始化简单文件系统
    fs::init();
    
    // 显示系统信息
    println!("[OK] System initialization complete!\n");
    
    // 运行 Hello World 程序
    println!("Running Hello World program...\n");
    fs::run_program("hello");
    
    println!("\nSystem halted. Press Ctrl+A X to exit QEMU.");
    
    // 进入无限循环
    loop {}
}

/// Panic 处理函数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! PANIC !!!\n{}\n", info);
    loop {}
}
