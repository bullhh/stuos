use core::panic::PanicInfo;

/// panic处理函数
/// 
/// 当内核发生严重错误时，该函数会被调用
/// 参数info包含了panic的相关信息
/// 函数不会返回
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // 在实际的操作系统中，这里应该记录错误信息并尝试安全关机
    loop {}
}