# AI4OSE Lab1 总结报告



---

## 0. 基本信息

- 姓名/学号：朱煜坤
- 学校/院系：广东医科大学生物医学工程学院
- 仓库地址：https://github.com/jerryinsz/ai4ose-lab1-2026s
- 报告日期：2026-3-28
- 使用的 AI 工具：Trae配合GPT-5.3-Codex

---

## 1. 实验完成概览

### 1.1 五个基础实验完成状态

| 章节 | 目标一句话 | 完成状态 | 自测状态 | 备注 |
|---|---|---|---|---|
| ch3 | 实现系统调用统计与 sys_trace 追踪功能 | ✅ | ✅ | 采用扩大 TCB 内数组并拦截系统调用的方式实现 |
| ch4 | 实现 mmap/munmap 匿名内存映射，支持进程虚存管理 | ✅ | ✅ | 重写 sys_trace 支持地址空间权限检查 |
| ch5 | 实现 spawn 系统调用与 stride 调度算法 | ✅ | ✅ | 增加优先级和步长计算机制 |
| ch6 | 实现文件系统硬链接与获取文件状态(linkat/unlinkat/fstat) | ✅ | ✅ | 操作 inode 与文件系统解耦 |
| ch8 | 实现基于银行家算法的互斥锁/信号量死锁检测机制 | ✅ | ✅ | 利用资源矩阵计算安全序列 |

### 1.2 环境与复现入口

- Rust 版本：`1.85.0-nightly` (或以上，支持 edition 2024)
- QEMU 版本：`7.0.0` (或以上)
- 目标架构：`riscv64gc-unknown-none-elf`
- 一键复现步骤（总入口）：

```bash
# 例如复现 ch3 实验：
cd tg-ch3
cargo run --features exercise
# 或者执行测试脚本
./test.sh exercise
```

---

## 2. 证据整理（按章节）

> 要求：每章至少写清楚四件事：  
> 1）改动点；2）测试命令；3）输出结果；4）1个典型 bug + 修复过程

### 2.1 ch3

#### 改动点
- 扩大了内核栈大小 `(APP_CAPACITY * 4 + 2) * 8192`，以防止运行复杂程序时栈溢出。
- 在 `TaskControlBlock` 中新增 `syscall_cnt: [usize; 512]` 用于统计各个系统调用的调用次数。
- 在 `handle_syscall` 中拦截 `sys_trace (ID 410)`，并根据 `trace_request` 的不同（0:读内存, 1:写内存, 2:查调用次数）分别做处理。
- 在应用退出（finish）时打印其系统调用统计信息。

#### 测试命令

```bash
cd tg-ch3
cargo run --features exercise
# 运行后在 QEMU 终端中输入 `ch3_usertest` 或对应测例
```

#### 输出结果

```text
app0 syscall statistics:
  syscall 64 -> 2
  syscall 93 -> 1
  syscall 113 -> 5
  syscall 124 -> 1
  syscall 410 -> 7
app0 exit with code 0
```

#### 典型 bug + 修复过程
- 现象：TODO
- 定位：TODO
- 修复：TODO
- 验证：TODO

---

### 2.2 ch4

#### 改动点
- 实现 `mmap` (ID 222)：按页对齐虚拟地址，检查权限标志，确保目标区间无映射后，利用 `address_space.map` 分配匿名页面。
- 实现 `munmap` (ID 215)：检查目标虚存是否全部已被映射，随后使用 `address_space.unmap` 解除映射。
- 重写 `trace`：利用 `translate::<u8>` 将虚拟地址转换为物理地址，同时传入 `UR_V` 或 `UW_V` 检查用户态的读写权限。

#### 测试命令

```bash
cd tg-ch4
cargo run --features exercise
# 然后输入 ch4_usertest
```

#### 输出结果

```text
[kernel] mmap success: addr = 0x10000000, len = 4096
[kernel] munmap success: addr = 0x10000000, len = 4096
```

#### 典型 bug + 修复过程
- 现象：TODO
- 定位：TODO
- 修复：TODO
- 验证：TODO

---

### 2.3 ch5

#### 改动点
- 实现 `spawn` (ID 400)：通过传入路径读取 ELF 文件并直接创建新进程 `ProcStruct::from_elf`，避免了 fork+exec 的开销。
- 实现 `stride` 调度算法：在 `Process` 中新增 `stride` 和 `priority`（初始值为 16），每次在 `ProcManager::fetch` 中遍历就绪队列，选取 `stride` 最小的进程执行，并将其 `stride` 加上 `BIG_STRIDE / priority`。
- 实现 `set_priority` (ID 140) 供用户调整优先级。

#### 测试命令

```bash
cd tg-ch5
cargo run --features exercise
# 输入 ch5_usertest
```

#### 输出结果

```text
[kernel] spawn child process success, pid = 2
[kernel] schedule process pid = 2 (stride = 13421772)
```

#### 典型 bug + 修复过程
- 现象：TODO
- 定位：TODO
- 修复：TODO
- 验证：TODO

---

### 2.4 ch6

#### 改动点
- 实现 `linkat` (ID 37)：将虚拟地址翻译出新旧路径字符串，并调用 `FS.link` 建立硬链接（在 inode 层指向同一个文件块）。
- 实现 `unlinkat` (ID 35)：调用 `FS.unlink` 删除硬链接，并在链接数为 0 时回收资源。
- 实现 `fstat` (ID 80)：获取 `fd` 对应的 `inode`，提取其编号、类型和链接数，将其写入用户态的 `Stat` 结构体中。

#### 测试命令

```bash
cd tg-ch6
cargo run --features exercise
# 输入 ch6_usertest
```

#### 输出结果

```text
[kernel] linkat success: oldpath = file1, newpath = file2
[kernel] fstat: fd=3, ino=1, mode=FILE, nlink=2
```

#### 典型 bug + 修复过程
- 现象：TODO
- 定位：TODO
- 修复：TODO
- 验证：TODO

---

### 2.5 ch8

#### 改动点
- 在 `Process` 中新增资源追踪结构：`mutex_owners`、`sem_owners` (Allocation)，`sem_avail` (Available)，以及 `thread_waiting_for` (Need)。
- 实现 `enable_deadlock_detect` (ID 469) 开启死锁检测开关。
- 在 `mutex_lock` 和 `semaphore_down` 操作前调用 `check_deadlock`：基于银行家算法的思想，利用 Work 和 Finish 向量寻找安全序列。如果检测到死锁（即请求会导致所有线程无法运行完毕），则拒绝分配并返回 `-0xDEAD`。

#### 测试命令

```bash
cd tg-ch8
cargo run --features exercise
# 输入 ch8_usertest
```

#### 输出结果

```text
[kernel] deadlock detected for tid 2 requesting Mutex(1)
[kernel] deadlock detected for tid 3 requesting Semaphore(0)
[kernel] deadlock prevented, returning -0xDEAD
```

#### 典型 bug + 修复过程
- 现象：TODO
- 定位：TODO
- 修复：TODO
- 验证：TODO

---

## 3. 与 AI 协作的方法论（怎么问、怎么验、怎么纠错）

本部分可参考我在项目中建立的工作流，详细内容见 `jerrydoing.md`、`manage.md`、`update.md` 与 `reference/spec.md`：
- **共性+个性化工作流**：将通用标准写入 reference 当基建，将业务要求写进 spec 当指令。
- **全貌导向 Spec 书写**：不光提需求（What/Why），还要清晰界定 OS 概念和模块间拓扑联系，给 AI 建立上帝视角。
- **文档分离机制**：采用 `README.md`（展示）、`learn.md`（踩坑复盘）和 `q&a.md`（高频 FAQ）相分离的文档架构。
- **标准化版本发布**：采用“预配置 -> Git 推送与打 Tag -> 干跑测试 -> Crates 发布与自测”的四步法工作流，解决多环境与包名冲突。

### 3.1 需求拆解：课程内容 → 可实现接口

- 我的提问方式（示例）：
  - `TODO: 示例提示词1`
  - `TODO: 示例提示词2`
- AI 给出的关键建议：TODO
- 我实际采用/放弃的点：TODO（并说明理由）

### 3.2 代码审阅与潜在问题检查

- 我如何让 AI 扮演 reviewer：TODO
- 重点检查项：并发安全 / 边界条件 / 特权级切换 / 内存访问合法性 / 资源释放 等
- 我如何验证建议有效：TODO（对照实验、压力测试、回归测试）

### 3.3 调试流程（症状 → 假设 → 证据 → 修复）

1. 固定复现场景：TODO  
2. 收集证据（日志/断言/统计）：TODO  
3. 让 AI 提出候选原因：TODO（3~5条）  
4. 逐条排除：TODO  
5. 最终修复与回归：TODO

### 3.4 AI 参与测试生成与文档重构

- 测试脚本/用例方面：TODO
- 文档重构方面：引入了 `learn.md` 和 `q&a.md`，把 AI 试错成本转化为静态经验知识库。
- 个人经验总结（有效做法）：必须在生成前通过 `reference` 或 Spec 统一 AI 对格式的认知，避免反复微调浪费 token。

---

## 4. 学习效果评估（定量 + 定性）

### 4.1 定量指标（建议 6~10 个）

| 指标 | 定义 | 基线（原教程） | 我的结果 | 说明 |
|---|---|---:|---:|---|
| 实验通过率 | 通过用例/总用例 | TODO | 100% | ch3-ch8 基础测例全部通过 |
| 平均修复时长 | 每个关键 bug 从发现到修复时间 | TODO | TODO | TODO |
| 回归失败次数 | 引入新改动后导致旧功能失败次数 | TODO | TODO | TODO |
| 复现实验成功率 | 脚本重复运行成功率 | TODO | 100% | 按照 `manage.md` 四步法执行，助教 clone 后开箱即用成功率 100% |
| 文档覆盖率 | 关键模块是否有对应说明 | TODO | 100% | README 加上独立的 Q&A 和 Learn 文档体系全覆盖 |
| AI 建议采纳率 | 采纳建议数/建议总数 | TODO | TODO | TODO |

> 注：如果暂时没有性能型指标（P99/缺页率等），可先用工程型指标替代，但要说明原因。

### 4.2 定性评估（能力提升/代价）

#### 能力提升
- 系统理解：深刻理解了特权级切换、页表内存隔离、硬链接 inode 机制及银行家算法在死锁检测中的应用。
- 工程能力：掌握了基于 Cargo 和 Git 的多 Crate 协作与双端（GitHub + Crates.io）发布规范。
- 抽象设计能力：摸索出了面向 AI 的 Spec 规范驱动开发（SDD）工作流。

#### 代价与不足
- 时间投入增加：维护规范化文档和排查 AI 的“幻觉”代码占据了可观的时间。
- 文档维护成本：多个子文档的联动需要在版本迭代时小心对齐。

---

## 5. 与本校现有教学实验教程对比

### 5.1 对比维度

| 维度 | 现有教程 | 我的实践 | 结论 |
|---|---|---|---|
| 覆盖范围 | TODO | TODO | TODO |
| 教学结构（机制-现象-数据） | TODO | TODO | TODO |
| 测试体系（脚本化/压力测试） | TODO | TODO | TODO |
| 可扩展性（模块化/可插拔） | TODO | TODO | TODO |
| 反馈速度（定位与修复效率） | TODO | TODO | TODO |

### 5.2 总体评价

- 我认为最有价值的改进：TODO
- 仍需改进的短板：TODO

---

## 6. 结论与后续计划

### 6.1 本阶段结论

- 我已完成：TODO
- 我证明了：TODO
- 我还欠缺：TODO

### 6.2 下一阶段计划（可选）

- TODO（例如：扩展中断、调度对比、统一 trace 指标）

---

## 附录 A：AI 对话与证据索引

- 对话记录文件：`docs/ai-chat-log.md`（或你的实际路径）
- 截图目录：`docs/ai-chat-screenshots/`
- 关键测试日志：`TODO`
- 关键提交记录：`TODO`

## 附录 B：复现命令清单

```bash
TODO: 按章节列出 ch3/ch4/ch5/ch6/ch8 的复现命令
```

