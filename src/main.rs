#![no_std]
#![no_main]
// 这是一个为aarch64-unknown-none-softfloat环境设计的简单操作系统内核
// 该内核不依赖标准库，在裸机环境下运行
mod lang_items;
#[macro_use]
mod print;
mod mm;


// 声明外部汇编入口点
unsafe extern "C" {
    unsafe fn _start_asm();
}

/// Rust主入口点
/// 
/// 由汇编代码调用，作为操作系统的主要入口
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    // 操作系统初始化和主循环
    // 在这里可以添加硬件初始化、内存管理等核心功能
    
    // 使用新的打印宏输出启动信息
    println!("StuOS kernel started successfully!");
    println!("Entering main loop...");
    
        
    println!("Initializing MMU...");
    
    // 首先初始化页表
    unsafe { mm::boot_pt::init_boot_page_table() };
    // 然后初始化MMU，传入页表的物理地址
    unsafe { mm::memory::init_mmu((&raw const mm::boot_pt::BOOT_PT_L0) as usize) };
    println!("MMU initialized");

    // 为了演示，我们简单地进入一个无限循环
    // 在实际的操作系统中，这里会是更复杂的逻辑
    let mut counter = 0;
    loop {
        // 每次循环增加计数器
        counter += 1;
        
        // 每1000000次循环输出一次信息（模拟简单的调度）
        if counter % 1000000 == 0 && counter < 10000000 {
            // 使用格式化打印输出计数信息
            println!("Kernel running, counter: {}", counter);
        }
        
        // 防止编译器优化掉这个循环
        core::hint::spin_loop();
    }


}

/// 获取栈顶指针
/// 
/// 用于初始化内核栈
#[unsafe(no_mangle)]
pub extern "C" fn get_stack_top() -> usize {
    unsafe extern "C" {
        unsafe static stack_top: usize;
    }
    unsafe { stack_top }
}