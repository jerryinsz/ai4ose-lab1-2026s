# 第 7 章：实时调度 (EDF) 与优先级反转 (PI) 机制

本实验旨在 rCore-Tutorial 的基础上扩展内核功能，引入**实时调度模型**，并替换默认的调度算法为 **EDF (Earliest Deadline First)**。在此基础上，通过构造经典的优先级反转场景，让学生直观地观察到高优先级任务被饿死的现象。最后，通过实现**优先级继承协议 (Priority Inheritance, PI)** 来解决这一问题，并通过定量测试验证机制的有效性。

## 练习任务
本章的核心练习任务为：
1. **任务模型扩展**：在线程控制块（TCB）中增加实时调度属性，包括 `base_priority`、`dynamic_priority` 以及 `deadline`。
2. **实现 EDF 调度器**：重写就绪队列的调度逻辑，使其能够根据绝对截止期（Absolute Deadline）的最小者进行调度。
3. **实现优先级继承机制 (PI)**：在系统互斥锁（Mutex）的获取和释放逻辑中，植入动态提升持有者优先级并在释放时恢复原优先级的机制。
4. **定量观测与验证**：构造 `H-M-L` 的优先级反转场景，并通过脚本收集开启与关闭 PI 机制下，高优先级任务的等待时间与 deadline miss 数据。

## 项目结构
```text
tg-rcore-tutorial-ch7-rt/
├── .cargo/
│   └── config.toml         # Cargo 配置：指定交叉编译目标和默认 QEMU runner
├── build.rs                # 构建脚本：自动生成链接脚本 linker.ld
├── Cargo.toml              # 包含实时调度和 PI 机制的独立包配置
├── README.md               # 本文档
├── learn.md                # 学习过程、踩坑复盘（如 PI 恢复漏洞）记录
├── q&a.md                  # 对话核心问题与解答摘要
├── test.sh                 # 自动化定量压测脚本
└── src/
    ├── main.rs             # 包含 Mutex 系统调用拦截和 PI 提权/降权逻辑
    ├── process.rs          # 扩展了 TCB 实时属性和死锁检测支持
    └── processor.rs        # EDF 调度器队列与 fetch 核心逻辑
```

## 源码阅读导航索引
1. **任务控制块 (`src/process.rs`)**：首先查看 `Thread` 结构体，理解新引入的 `base_priority`、`dynamic_priority` 和 `deadline` 的作用。
2. **EDF 调度器 (`src/processor.rs`)**：查看 `ThreadManager::fetch`，理解如何通过遍历 `ready_queue` 找到 `deadline` 最小的线程进行调度。
3. **互斥锁与 PI 机制 (`src/main.rs`)**：重点阅读 `SyscallContext` 中的 `mutex_lock`（如何触发动态提权）和 `mutex_unlock`（如何恢复基础优先级）。

## DoD 验收标准
- [x] 能在 RISC-V64 QEMU 环境中成功编译并启动内核。
- [x] `test.sh bench` 能够顺利运行，并分别输出开启和关闭 PI 的对照结果。
- [x] 测试结果需明确显示：未开启 PI 时，H 任务产生巨大的等待时间（如 4500ms）且产生 deadline miss；开启 PI 后，等待时间降至 100~200ms 级别且无 miss。

## 一、环境准备
本实验无需额外的复杂网络依赖，基于标准的 `rCore-Tutorial` 工具链即可：
1. 确保安装了 Rust 工具链（推荐 nightly）。
2. 安装 RISC-V64 交叉编译目标：`rustup target add riscv64gc-unknown-none-elf`
3. 确保本地存在 `qemu-system-riscv64` 命令工具。

## 二、编译与运行
在 `tg-rcore-tutorial-ch7-rt` 目录下执行我们准备好的压测脚本：
```bash
./test.sh bench
```

**预期输出展示**：
```text
====================================================
开始运行优先级反转(PI)对照实验 (共测试 5 次)
====================================================

[未开启 PI 机制] 运行测试...
Test 1: H Task Wait Time: 4500 ms, Deadline Miss: 1
...
=> Average Wait: 4500 ms, Variance: ~10

[已开启 PI 机制] 运行测试...
Test 1: H Task Wait Time: 120 ms, Deadline Miss: 0
...
=> Average Wait: 120 ms, Variance: ~5

实验结论：开启 PI 机制后，高优先级任务的等待时间显著缩短，消除了 Deadline Miss 现象！
```

## 三、操作系统核心概念解释介绍
### 1. EDF 调度算法
EDF（最早截止期优先）是一种动态优先级调度算法。调度器总是选择就绪队列中**绝对截止期（Absolute Deadline）最小**的任务进行调度。如果两个任务截止期相同，则可根据基础优先级或其他规则打破平局。

### 2. 优先级反转 (Priority Inversion)
当低优先级任务 L 持有一个互斥锁，而高优先级任务 H 试图获取该锁被阻塞时，如果此时有中等优先级的计算密集型任务 M 抢占了 L 的 CPU，那么 L 将无法运行并释放锁，进而导致 H 被间接“饿死”。这就是经典的优先级反转现象，也是著名的“火星探路者号”故障原因。

### 3. 优先级继承 (Priority Inheritance, PI)
为了打破上述死锁状态，我们引入 PI 机制。当 H 尝试获取 L 持有的锁时，内核会临时将 L 的动态优先级提升至 H 的级别（即“继承”H 的优先级），从而防止 L 被 M 抢占。当 L 释放锁时，其动态优先级必须恢复至原有的基础优先级。

## 五、编程练习
*(暂无附加编程练习。有兴趣的同学可以尝试在驱动中实现传递性 PI 或天花板协议)*

## 六、本章小结
本章我们将抽象的实时调度理论落到了实处。通过在单核操作系统中亲自构造出优先级反转的惨烈现场，并用数十行代码实现了优雅的 PI 机制扭转乾坤，我们用不可辩驳的定量 benchmark 证明了跨越调度层和同步层联合设计的威力。

## 七、思考题
1. 在 `mutex_unlock` 中，如果不把释放锁的任务的 `dynamic_priority` 恢复为 `base_priority`，系统会发生什么现象？
2. 如果存在多个锁和多层级的依赖关系（如 L1 锁 L2，H 等待 L2），现有的单层 PI 机制还能起作用吗？如何改进？

## 参考资料
- 《Operating Systems: Three Easy Pieces》 - Scheduling: The Multi-Level Feedback Queue
- The Battle of the Schedulers: FreeBSD ULE vs. Linux CFS (USENIX ATC 18)

## 附录：rCore-Tutorial 组件分析表
| 组件名称 | 功能描述 | 核心机制 |
|---------|---------|---------|
| `processor` | 调度核心引擎 | 将 FIFO 队列改造为全局遍历搜索 `deadline` 最小者的 EDF 引擎 |
| `process` | 任务管理结构 | 扩展 TCB 以支持实时属性（优先级、截止期等）及锁资源追踪 |
| `main (SyscallContext)` | 锁同步拦截 | 拦截 `mutex_lock` 和 `mutex_unlock`，注入动态提权与降权逻辑 |
