# AI4OSE 实验项目版本与发布管理规范 (Release & Version Management)

在多人协作、多实验并行推进的 BigLabA 大实验中，将实验工程安全、规范地托管至 GitHub 并发布到 `crates.io` 是一项充满挑战的任务。

本指南旨在打破发布过程中的“循环依赖”困境，并建立一套从本地开发、Git 版本控制，到 Crate 发布的标准化工作流，确保：
1. **GitHub 仓库结构清晰，没有冗余的命名污染。**
2. **`crates.io` 上包名唯一，署名清晰，且版本号符合实验阶段的规范。**
3. **依赖关系正确，保证助教和老师可以通过 `cargo clone` 或 `git clone` 做到开箱即用。**

---

## 一、 命名哲学与版本号规范

### 1. 文件夹与仓库命名 (GitHub 端)
由于你在 GitHub 上拥有独立的个人仓库（如 `jerryinsz/ai4ose-lab1-2026s`），仓库本身已经具备了“命名空间（Namespace）”隔离的作用。
- **本地文件夹命名原则：** 保持简明扼要，**不要**在本地文件夹或 GitHub 路径上加上个人的后缀。
- **正例：** `tg-rcore-tutorial-ch1-uart`
- **反例：** `tg-rcore-tutorial-ch1-uart-jerryinsz`（冗余且破坏了原始的项目结构美感）。

### 2. Crate 包命名 (crates.io 端)
与 GitHub 不同，`crates.io` 是一个全局的扁平命名空间。十个同学都叫 `tg-rcore-tutorial-ch1-uart` 必然会导致冲突。
- **Crate 命名原则：** 在 `Cargo.toml` 的 `[package]` 中，`name` 字段**必须**加上你的个人标识后缀。
- **规范格式：** `[原实验名]-[个人标识]`
- **示例：** `name = "tg-rcore-tutorial-ch1-uart-jerryinsz"`

### 3. 版本号管理 (Version Control)
根据老师要求，由于目前发布的是测试/实验版本，必须采用带有预发布标识（Pre-release）的语义化版本控制。
- **规范格式：** `0.X.Y-preview.Z`
- **示例：** `version = "0.1.0-preview.1"`
- **更新逻辑：** 
  - 修复小 Bug 且不改变功能：升级 `Z`（如 `0.1.0-preview.2`）。
  - 增加新练习或功能：升级 `Y` 并重置 `Z`（如 `0.1.1-preview.1`）。

---

## 二、 解决“循环依赖”的标准发布工作流

很多同学在发布时会遇到“循环依赖”困境：`Cargo.toml` 中要求写 GitHub 仓库 URL，但代码还没推送到 GitHub，如果提前写死 URL，万一 GitHub 上传时改了仓库名或路径就会造成死链。

为了解决这个问题，请严格按照以下**四步法**执行：

### 第一步：本地收尾与预配置 (Local Prep)
在决定要发布一个实验（如 ch1-uart）时：
1. **清理代码与文档：** 确保 `README.md`、`learn.md`、`q&a.md` 等完备。
2. **配置 `Cargo.toml` (初版)：**
   ```toml
   [package]
   # 必须带上个人后缀防止 crates.io 冲突
   name = "tg-rcore-tutorial-ch1-uart-jerryinsz" 
   version = "0.1.0-preview.1"
   authors = ["Jerry <your-email@example.com>"]
   edition = "2024"
   
   # 预先规划好你在 GitHub 上的最终路径，此时先不要乱改动念，坚定执行
   repository = "https://github.com/jerryinsz/ai4ose-lab1-2026s/tree/main/tg-rcore-tutorial-ch1-uart"
   homepage = "https://github.com/jerryinsz/ai4ose-lab1-2026s"
   
   # 老师要求的 5 个关键字（强制要求）
   keywords = ["ai", "ai4ose", "kernel", "learning", "os"]
   license = "GPL-3.0"
   readme = "README.md"
   ```

### 第二步：推送至 GitHub 并打 Tag (Git Release)
先上 GitHub，再上 crates.io，这是打破循环的关键。
1. **提交并推送到 GitHub：**
   ```bash
   git add .
   git commit -m "release: prepare tg-rcore-tutorial-ch1-uart-jerryinsz v0.1.0-preview.1"
   git push origin main
   ```
2. **在 GitHub 上验证 URL：** 此时去浏览器点击你在 `Cargo.toml` 中写的 `repository` 链接，**确保链接是活的、正确的**。
3. **打上 Git Tag：** 这一步是为了方便老师通过 Git 复现。
   ```bash
   # Tag 命名规范：包名-版本号
   git tag tg-rcore-tutorial-ch1-uart-jerryinsz-v0.1.0-preview.1
   git push origin tg-rcore-tutorial-ch1-uart-jerryinsz-v0.1.0-preview.1
   ```

### 第三步：Crate 发布与验证 (Crates.io Publish)
当 GitHub 端已经稳定且 Tag 就绪后，再执行向 `crates.io` 的发布。
1. **干跑测试（非常重要）：**
   ```bash
   cargo publish --dry-run
   ```
   检查是否有未提交的文件、本地绝对路径依赖报错，或者大小超限等问题。
2. **正式发布：**
   ```bash
   cargo publish
   ```
3. **自测复现（扮演助教）：**
   发布成功后，换一个空目录，测试是否满足开箱即用：
   ```bash
   cd /tmp
   cargo clone tg-rcore-tutorial-ch1-uart-jerryinsz
   cd tg-rcore-tutorial-ch1-uart-jerryinsz
   cargo run
   ```

### 第四步：填写老师要求的审查表格
完成上述步骤后，你的 Crate 已经双端可用。按照老师要求，填写表格：
- **格式：** `完成时间：crate名字`
- **示例：** `2026-03-28：tg-rcore-tutorial-ch1-uart-jerryinsz`

---

## 三、 Crate 包内依赖管理避坑指南

如果在当前 Crate（如 `ch1-uart`）内部还依赖了自己写的其他子 Crate（如 `tg-rcore-tutorial-uart`）：
1. **本地开发时：** 可以使用路径依赖 `path = "tg-rcore-tutorial-uart"`。
2. **发布到 crates.io 时：** `crates.io` **不接受**纯本地 `path` 依赖的包上传。
   - **解决策略：** 你必须先将子 Crate (`tg-rcore-tutorial-uart`) 也加上个人后缀（如 `tg-rcore-tutorial-uart-jerryinsz`）并优先发布到 `crates.io`。
   - 然后在主 Crate 中修改依赖为版本依赖：
     ```toml
     [dependencies]
     tg-rcore-tutorial-uart-jerryinsz = "0.1.0-preview.1"
     ```
   - **例外情况：** 如果子 Crate 不需要单独复用，建议将其直接作为主项目 `src/` 下的一个 module（如 `src/uart.rs`），彻底消除发布时的多包依赖烦恼，符合“最简上传原则”。
