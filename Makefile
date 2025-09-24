# StuOS操作系统的Makefile
# 用于构建和运行aarch64架构的操作系统内核

# 构建配置
# TARGET: 指定编译目标平台为aarch64裸机环境
TARGET := aarch64-unknown-none-softfloat

# MODE: 指定构建模式，可以是debug或release
MODE := debug

# KERNEL_ELF: 指定生成的内核ELF文件路径
KERNEL_ELF := target/$(TARGET)/$(MODE)/stuos

# KERNEL_BIN: 指定生成的内核二进制文件路径
KERNEL_BIN := $(KERNEL_ELF).bin

# DISASM_TMP: 反汇编输出的临时文件路径
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

# 根据构建模式设置Cargo构建参数
# 如果是release模式，则添加--release参数
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# QEMU配置
# QEMU模拟器参数配置
QEMU_ARGS := -M virt \
			 -cpu cortex-a53 \
			 -nographic \
			 -kernel $(KERNEL_ELF)

# 默认目标，构建内核
.PHONY: all
all: build

# 环境检查目标
# 确保构建环境已安装必要的组件
env:
	@echo "检查并安装必要的构建组件..."
	(rustup target list | grep "$(TARGET) (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

# 构建目标
# 依赖env目标确保环境已准备就绪
build: env $(KERNEL_BIN)

# 生成内核二进制文件
# 使用rust-objcopy将ELF格式转换为二进制格式
$(KERNEL_BIN): kernel
	@echo "正在生成内核二进制文件..."
	@rust-objcopy --binary-architecture=aarch64 $(KERNEL_ELF) --strip-all -O binary $@
	@echo "内核二进制文件生成完成: $(KERNEL_BIN)"

# 内核编译目标
kernel:
	@echo "正在编译StuOS内核..."
	@cargo build $(MODE_ARG)
	@echo "内核编译完成: $(KERNEL_ELF)"

# 清理目标
# 清理所有构建产物
.PHONY: clean
clean:
	@echo "正在清理构建产物..."
	@cargo clean
	@echo "清理完成"

# 在QEMU中运行内核
.PHONY: run
run: build
	@echo "正在QEMU中运行StuOS内核..."
	@qemu-system-aarch64 $(QEMU_ARGS)

# 在QEMU中运行内核GDB调试
.PHONY: run-gdb
run-gdb: build
	@echo "正在QEMU中运行StuOS内核并打开GDB监听端口..."
	@echo "监听端口: 1234 (GDB), 5555 (QEMU monitor)"
	@qemu-system-aarch64 $(QEMU_ARGS) -s -S -monitor telnet:localhost:5555,server,nowait

# GDB调试客户端连接
.PHONY: gdb-client
gdb-client:
	@echo "正在启动GDB客户端连接到QEMU..."
	@echo "在GDB中执行以下命令:"
	@echo "  1. (gdb) set architecture aarch64"
	@echo "  2. (gdb) target remote localhost:1234"
	@echo "  3. (gdb) break kernel_main"
	@echo "  4. (gdb) continue"
	@gdb-multiarch -ex 'file $(KERNEL_ELF)'

# 安装aarch64 GDB调试器（如果尚未安装）
.PHONY: install-gdb
install-gdb:
	@echo "正在安装aarch64 GDB调试器..."
	@sudo apt update
	@sudo apt install gdb-multiarch
	@echo "安装完成。您可以使用 'gdb-multiarch' 进行aarch64调试"

# 发布版本构建目标
.PHONY: release
release:
	@echo "正在构建发布版本StuOS内核..."
	@$(MAKE) MODE=release build

# 反汇编目标
# 生成内核的反汇编代码并使用less查看
.PHONY: disasm
disasm: kernel
	@echo "正在生成内核反汇编代码..."
	@rust-objdump --arch-name=aarch64 -x $(KERNEL_ELF) | less

# 反汇编并在VIM中查看
.PHONY: disasm-vim
disasm-vim: kernel
	@echo "正在生成内核反汇编代码并在VIM中打开..."
	@rust-objdump --arch-name=aarch64 -x $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

# 查看内核信息
.PHONY: info
info: kernel
	@echo "StuOS内核文件信息:"
	file $(KERNEL_ELF)
	@echo ""
	@echo "内核大小:"
	ls -lh $(KERNEL_ELF)

# 查看链接脚本
.PHONY: linker-info
linker-info:
	@echo "链接脚本内容:"
	@echo "================"
	@cat linker.ld
	@echo "================"

# 检查内核内存布局
.PHONY: layout
layout: kernel
	@echo "内核内存布局信息:"
	aarch64-linux-gnu-objdump -h $(KERNEL_ELF) 2>/dev/null || echo "请安装aarch64-linux-gnu-objdump工具"