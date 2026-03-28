# 第 1 章：裸机环境与S-Mode串口驱动实验

本章旨在实现一个最简单的 RISC-V S-Mode 裸机程序，并脱离底层固件的限制，直接实现 16550a 兼容的串口设备驱动以进行控制台输出。

## 练习任务
本章的核心练习任务为：
1. **移除对 RustSBI 字符输出的依赖**：不再使用 `sbi_console_putchar`，以掌握操作系统对底层硬件的直接控制权。
2. **实现 S-Mode 串口驱动**：开发一个名为 `tg-rcore-tutorial-uart` 的功能组件，通过读写 MMIO 寄存器来初始化 UART 并输出字符。
3. **集成与格式化输出**：在内核中实现 `console.rs` 模块，将 Rust 的 `core::fmt::Write` 特性与新的串口驱动相连，恢复 `print!` 与 `println!` 宏。

## 项目结构
```text
tg-rcore-tutorial-ch1-uart/
├── .cargo/
│   └── config.toml         # Cargo 配置：指定交叉编译目标和默认 QEMU runner
├── build.rs                # 构建脚本：自动生成 S-mode 链接脚本 linker.ld
├── Cargo.toml              # 极简内核配置与依赖（包含自研串口驱动）
├── README.md               # 本文档
├── learn.md                # 学习过程、AI 交互及复盘总结
├── q&a.md                  # 对话核心问题与解答摘要
├── src/
│   ├── console.rs          # 格式化输出模块：基于 UART 实现 print! 和 println! 宏
│   ├── main.rs             # 极简内核源码：程序入口、UART 初始化、panic 处理
│   └── sbi.rs              # SBI 关机接口：通过内联汇编触发系统重置
└── tg-rcore-tutorial-uart/ # 独立的 UART 功能组件
    ├── Cargo.toml
    └── src/
        └── lib.rs          # 16550a 串口驱动实现，提供基于 MMIO 的初始化与输出接口
```

## 源码阅读导航索引
1. **启动与入口 (`src/main.rs`)**：首先阅读 `_start` 裸函数，了解 RISC-V 的栈指针初始化与 S 态主函数跳转。
2. **硬件驱动 (`tg-rcore-tutorial-uart/src/lib.rs`)**：查看 16550a 串口寄存器的定义、基地址 `0x10000000` 以及 `init` 和 `put_char` 函数的具体 MMIO 操作。
3. **格式化输出 (`src/console.rs`)**：了解如何通过实现 `core::fmt::Write` trait，将底层的 `put_char` 包装成标准的高级输出宏。
4. **关机与异常 (`src/sbi.rs` & `panic_handler`)**：阅读如何通过内联汇编调用 SBI 的 `SYSTEM_RESET` 扩展以优雅退出 QEMU。

## DoD 验收标准
- [x] 能在 RISC-V64 QEMU 环境中成功启动内核，并不再依赖 `tg-sbi` 等外部本地包。
- [x] 通过内核直接操作 UART 的 `THR` 寄存器打印出 `"Hello, world!"`。
- [x] 系统能够正常触发 SBI Shutdown 进行退出，退出码为 0。

## 一、环境准备
本实验无需额外的复杂网络依赖，是一个自完备的裸机程序：
1. 确保安装了 Rust 工具链（推荐 nightly）。
2. 安装 RISC-V64 交叉编译目标：`rustup target add riscv64gc-unknown-none-elf`
3. 确保本地存在 `qemu-system-riscv64` 命令工具。

## 二、编译与运行
由于项目已在 `.cargo/config.toml` 中配置好了 target 和 runner，只需要在 `tg-rcore-tutorial-ch1-uart` 目录下执行：
```bash
cargo run
```
**预期输出展示**：
```text
OpenSBI v1.3
...
Boot HART MEDELEG         : 0x0000000000f0b509
Hello, world!
UART driver initialized successfully in S-Mode!
```
---

## 三、操作系统核心概念解释介绍
1. **S-Mode (Supervisor Mode)**：操作系统的核心运行态，享有对物理内存和中断的部分控制权，但需要借助 M-Mode（Machine Mode）固件（如 RustSBI/OpenSBI）进行系统级重置。
2. **MMIO (Memory-Mapped I/O)**：一种硬件交互方式，将外部设备寄存器映射到物理内存地址空间。通过对内存特定地址（如本实验中的 `0x10000000`）进行读写，即可操控串口的收发。
3. **16550a UART**：经典的串行通信控制器标准，支持发送和接收缓冲 FIFO、中断和波特率控制。本实验中，我们将禁用中断，通过轮询（Spin Loop）的方式向 `THR` 寄存器写入数据。

## 五、编程练习
*(暂无附加编程练习。有兴趣的同学可以尝试在驱动中实现 `get_char` 功能)*

## 六、本章小结
本章我们搭建了一个纯净的 RISC-V 裸机实验环境，脱离了原版实验对 RustSBI 字符输出的依赖。通过引入自定义的 16550a 串口驱动模块，我们不仅学习了如何在 no_std 下构建功能组件，也深刻理解了操作系统是如何直接与硬件进行 MMIO 通信的。

## 七、思考题
1. 在串口驱动的 `put_char` 函数中，为什么写入 `THR` 寄存器之前，需要先通过一个 `while` 循环不断读取 `LSR` 寄存器的状态？如果不进行这一步会发生什么？
2. `tg-rcore-tutorial-uart` 中使用了 `spin::Mutex` 来包装 `Uart` 结构体，这在裸机环境中有什么必要性？

## 参考资料
- [16550a UART 硬件规范](http://byterunner.com/16550.html)
- [rCore-Tutorial-Book-v3 第九章：设备驱动](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter9/2device-driver-1.html#)

## 附录：rCore-Tutorial 组件分析表
| 组件名称 | 功能描述 | 核心机制 |
|---------|---------|---------|
| `console` | 格式化输出引擎 | 借助 `fmt::Write` trait，封装底层驱动提供 `print!` 宏 |
| `sbi` | S-Mode 环境调用 | 封装 `ecall` 指令，负责与 M-Mode 固件通信以进行关机 |
| `uart` | 硬件驱动 | 操作基地址为 `0x10000000` 的 MMIO 寄存器，实现硬件级别的字符收发 |
