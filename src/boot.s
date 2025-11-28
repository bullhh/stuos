// StuOS 汇编入口点
// 该文件定义了内核的汇编级入口点，负责初始设置

// 将代码放在.text.entry段中
.section .text.entry

.global _start_asm
.type _start_asm, @function

_start_asm:
    // 禁用中断
    msr daifset, #0xf

    // 设置栈指针
    adr x0, stack_top
    mov sp, x0

    // 调用Rust主函数
    bl kernel_main

    // 如果kernel_main返回（不应该发生），则进入无限循环
    b .

.size _start_asm, . - _start_asm

// 确保符号被正确导出给链接器
.global stack_top

// 切换到高地址栈的函数
.global switch_to_high_stack
.type switch_to_high_stack, @function

switch_to_high_stack:
    // 设置物理地址到虚拟地址的偏移量 (0xFFFF_0000_0000_0000)
    mov x8, #0xFFFF000000000000
    // 将当前栈指针加上偏移量，切换到高地址
    add sp, sp, x8
    ret

.size switch_to_high_stack, . - switch_to_high_stack