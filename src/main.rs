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
    unsafe fn switch_to_high_stack();
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
        
    println!("Initializing MMU...");
    
    // 首先初始化页表
    unsafe { mm::boot_pt::init_boot_page_table() };
    // 然后初始化MMU，传入页表的物理地址
    unsafe { mm::memory::init_mmu((&raw const mm::boot_pt::BOOT_PT_L0) as usize) };
    
    // 切换到高地址栈
    println!("Switching to high address stack...");
    unsafe { switch_to_high_stack() };
    println!("Stack switched to high address");

    test_mmu();

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

pub fn test_mmu() {

    let phys_addr = 0x40001000;  // 选择一个安全的物理地址
    let test_value = 0x12345678u32;
    
    // 直接写入物理地址
    unsafe {
        *(phys_addr as *mut u32) = test_value;
    }
    
    // 验证写入
    let read_value = unsafe { *(phys_addr as *const u32) };
    println!("Read from physical address 0x{:x}: 0x{:x}", phys_addr, read_value);

    
    let virt_addr = phys_addr + 0xFFFF_0000_0000_0000 as usize;
    let read_value = unsafe { *(virt_addr as *const u32) };
    println!("Read from virtual address 0x{:x}: 0x{:x}", virt_addr, read_value);


    let mmu_test_value = 0x87654321u32;
    println!("ref: {:#x}", &mmu_test_value);
    println!("ref: {:p}", &mmu_test_value);
    println!("mmu_test_value address: 0x{:x}", &mmu_test_value as *const _ as usize);
}
