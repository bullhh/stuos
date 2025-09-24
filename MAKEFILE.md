# StuOS Makefile 使用说明

## 概述

本项目包含一个Makefile，用于简化StuOS操作系统的构建、运行和管理流程。Makefile为aarch64架构的裸机环境提供了完整的工具链支持。

## 使用方法

### 1. 构建内核

#### 开发版本构建
```bash
make build
```
- 编译StuOS内核的开发版本
- 生成未优化的可执行文件，包含调试信息
- 文件位置：target/aarch64-unknown-none-softfloat/debug/stuos

#### 发布版本构建
```bash
make release
```
- 编译StuOS内核的发布版本
- 启用编译器优化，生成更小、更快的可执行文件
- 文件位置：target/aarch64-unknown-none-softfloat/release/stuos

### 2. 运行内核

#### 开发版本运行
```bash
make run
```
- 自动构建开发版本内核（如果需要）
- 在QEMU模拟器中运行StuOS内核
- 使用虚拟硬件平台和Cortex-A53处理器模拟

#### 发布版本运行
```bash
make run-release
```
- 自动构建发布版本内核（如果需要）
- 在QEMU模拟器中运行优化后的StuOS内核

### 3. 管理命令

#### 清理构建产物
```bash
make clean
```
- 删除所有构建生成的文件
- 清理target目录中的内容

#### 查看内核信息
```bash
make info
```
- 显示内核文件的详细信息
- 展示文件类型和大小

#### 查看链接脚本
```bash
make linker-info
```
- 显示链接脚本的内容
- 用于检查内存布局定义

#### 检查内核内存布局
```bash
make layout
```
- 显示内核的内存段落信息
- 需要objdump工具支持

#### 安装QEMU
```bash
make install-qemu
```
- 安装QEMU模拟器（如果尚未安装）
- 需要sudo权限

### 4. 其他目标

#### 默认构建
```bash
make
```
- 等同于`make build`

#### 全部构建
```bash
make all
```
- 等同于`make build`

## QEMU运行说明

当使用`make run`或`make run-release`命令时，系统会在QEMU中启动StuOS内核：

1. 使用的QEMU参数：
   - `-M virt`：使用虚拟硬件平台
   - `-cpu cortex-a53`：模拟Cortex-A53处理器
   - `-nographic`：无图形界面模式
   - `-kernel`：指定要运行的内核文件

2. 退出QEMU：
   - 按下`Ctrl+A`然后按`X`退出QEMU

## 目录结构

- 源代码：src/main.rs
- 汇编入口点：src/boot.s
- 链接脚本：linker.ld
- 配置文件：Cargo.toml, .cargo/config.toml
- 构建产物：target/aarch64-unknown-none-softfloat/
- Makefile：项目根目录下的Makefile

## 环境要求

- Rust工具链
- Cargo构建工具
- QEMU模拟器（aarch64版本）
- aarch64-linux-gnu-objdump（用于内存布局检查，可选）