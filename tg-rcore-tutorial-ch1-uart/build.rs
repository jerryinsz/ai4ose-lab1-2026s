//! 构建脚本：为 RISC-V64 目标自动生成链接脚本。
//!
//! 链接脚本控制程序各段在内存中的布局，确保：
//! - S-mode 代码（_start 入口）从 0x80200000 开始

fn main() {
    use std::{env, fs, path::PathBuf};

    // 仅在交叉编译到 RISC-V64 时生成链接脚本
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "riscv64" {
        let ld = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("linker.ld");
        fs::write(&ld, LINKER_SCRIPT).unwrap();
        // 告诉 rustc 使用此链接脚本
        println!("cargo:rustc-link-arg=-T{}", ld.display());
    }
}

/// 链接脚本内容。
///
/// 内存布局：
///
/// ```text
/// 0x80200000  S-mode 区域（本程序）
///   .text           代码段（含 .text.entry 入口）
///   .rodata         只读数据段
///   .data           可读写数据段
///   .bss            未初始化数据段（含栈空间）
/// ```
const LINKER_SCRIPT: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)

/* S-mode code base address: QEMU loads kernel here when using -bios default */
BASE_ADDRESS = 0x80200000;

SECTIONS {
    . = BASE_ADDRESS;
    .text   : {
        *(.text.entry)          /* _start entry, must come first */
        *(.text .text.*)        /* other code */
    }
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    .data   : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    .bss    : {
        *(.bss.uninit)          /* stack space */
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
}";
