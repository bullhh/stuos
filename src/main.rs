#![no_std]
#![no_main]
// 这是一个为aarch64-unknown-none-softfloat环境设计的简单操作系统内核
// 该内核不依赖标准库，在裸机环境下运行
mod lang_items;

// 声明外部汇编入口点
unsafe extern "C" {
    unsafe fn _start_asm();
}

/// 一个简单的输出函数，用于在QEMU中显示信息
/// 这里我们使用一个简化的实现来输出字符
unsafe fn print_str(_s: &str) {
    // 在实际的操作系统中，这里会是更复杂的串口输出实现
    // 目前我们只是简单地进入循环，但在真实环境中会输出字符串
}

/// Rust主入口点
/// 
/// 由汇编代码调用，作为操作系统的主要入口
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    // 操作系统初始化和主循环
    // 在这里可以添加硬件初始化、内存管理等核心功能
    
    // 输出启动信息
    unsafe {
        print_str("StuOS kernel started successfully!\n");
        print_str("Entering main loop...\n");
    }
    
    // 为了演示，我们简单地进入一个无限循环
    // 在实际的操作系统中，这里会是更复杂的逻辑
    let mut counter = 0;
    loop {
        // 每次循环增加计数器
        counter += 1;
        
        // 每1000000次循环输出一次信息（模拟简单的调度）
        if counter % 1000000 == 0 {
            // 在实际实现中，这里会输出计数信息
            // 但由于我们没有实现完整的串口输出，暂时只循环
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