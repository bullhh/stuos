//! 操作系统打印功能模块
//! 提供内核级别的打印输出功能

use core::fmt::{self, Write};

/// QEMU virt平台的UART基地址
const UART_BASE: usize = 0x9000000;
// const UART_BASE: usize = 0xFFFF0000_9000000;

/// UART数据寄存器偏移
const UARTDR: usize = 0x00;

/// UART标志寄存器偏移
const UARTFR: usize = 0x18;

/// UART标志寄存器位定义
const UARTFR_TXFF: u32 = 1 << 5; // 发送FIFO满

/// 向UART发送一个字符
unsafe fn uart_putc(c: u8) {
    let uart_base = UART_BASE as *mut u32;
    let uartdr = unsafe { uart_base.add(UARTDR / 4) };
    let uartfr = unsafe { uart_base.add(UARTFR / 4) };
    
    // 等待发送FIFO有空闲空间
    while unsafe { uartfr.read_volatile() } & UARTFR_TXFF != 0 {
        // 空循环等待
    }
    
    // 发送字符
    unsafe { uartdr.write_volatile(c as u32) };
}

/// 控制台输出结构体
struct Stdout;

/// 向控制台输出单个字符
/// 
/// # 参数
/// * `c` - 要输出的字符（作为usize类型）
unsafe fn console_putchar(c: usize) {
    // 在QEMU virt平台中，通过PL011 UART控制器输出字符
    unsafe {
        uart_putc(c as u8);
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                console_putchar(c as usize);
            }
        }
        Ok(())
    }
}

/// 格式化打印函数
/// 
/// 类似于标准库中的println!宏，但适用于内核环境
/// 
/// # 参数
/// * `args` - 格式化参数
pub fn stdout_write_fmt(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 打印函数
/// 
/// # 参数
/// * `s` - 要打印的字符串
pub fn stdout_write_str(s: &str) {
    Stdout.write_str(s).unwrap();
}

/// 类似于标准库的print!宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::print::stdout_write_fmt(format_args!($($arg)*));
    });
}

/// 类似于标准库的println!宏
#[macro_export]
macro_rules! println {
    () => ({
        $crate::print::stdout_write_str("\n");
    });
    ($fmt:expr) => ({
        $crate::print::stdout_write_str($fmt);
        $crate::print::stdout_write_str("\n");
    });
    ($fmt:expr, $($arg:tt)*) => ({
        $crate::print::stdout_write_fmt(format_args!($fmt, $($arg)*));
        $crate::print::stdout_write_str("\n");
    });
}