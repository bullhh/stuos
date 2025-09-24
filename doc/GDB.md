# 设置架构
(gdb) set architecture aarch64

# 连接到QEMU
(gdb) target remote localhost:1234

# 设置断点在主函数入口
(gdb) break kernel_main

# 继续执行到断点
(gdb) continue

# 查看当前代码位置
(gdb) list

# 查看寄存器状态
(gdb) info registers

# 单步执行
(gdb) step
# 或者
(gdb) next

# 查看调用栈
(gdb) backtrace

# 查看变量值
(gdb) print variable_name