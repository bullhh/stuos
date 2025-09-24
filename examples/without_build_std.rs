// 示例：不使用build-std配置的项目
// 需要手动处理标准库依赖

// .cargo/config.toml内容：
// [build]
// target = "aarch64-unknown-none-softfloat"
// 
// （没有[unstable]部分）

#![no_std]
#![no_main]
// 启用rustc_private特性以使用编译器私有库
#![feature(rustc_private)]

// 需要显式声明使用的标准库组件
extern crate core;
// 注意：compiler_builtins在没有build-std的情况下难以手动引入

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}