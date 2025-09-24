// 示例：使用build-std配置的项目
// 这是当前项目的配置方式

// .cargo/config.toml内容：
// [build]
// target = "aarch64-unknown-none-softfloat"
// 
// [unstable]
// build-std = ["core", "compiler_builtins"]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}