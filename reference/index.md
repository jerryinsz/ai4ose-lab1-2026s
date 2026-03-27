# 实验项目索引与AI智能体工作指南 (Index & AI Guidelines)

## 零、 核心目标仓库 (GitHub Repository)
为了保证实验成果能够成功上传并减少后续的修改，所有实验都将上传至以下 GitHub 仓库：
👉 **[https://github.com/jerryinsz/ai4ose-lab1-2026s](https://github.com/jerryinsz/ai4ose-lab1-2026s)**

每个实验模块（例如实验1）都应作为该目录下的一个子目录存在，例如 `tg-rcore-tutorial-ch1-uart`。在生成配置（如 `Cargo.toml`）和组织项目依赖时，必须基于此仓库结构的相对路径，确保 Clone 下来后能正常运行。

## 一、 当前阶段 (Current Stage)
当前所处阶段：**BigLabA**
我们已经完成了基础实验 `tg-rcore-tutorial-ch{3/4/5/6/8}` 下所列的5个基础实验练习，完成的实验代码和文档存储在 `tg-ch3` 到 `tg-ch8` 文件夹中（由学生和AI共同完成）。

## 二、 必读文档指南 (Must-read Documents)
AI 智能体或后续开发者在接手工作前，**必须阅读**以下核心参考文档：

1. **项目规范与模范基准**
   - [`scripts/crates.txt`](../scripts/crates.txt)：未被修改过的模范文件列表，可作为参考基准。
   - [`src/lib.rs`](../src/lib.rs)：展示了项目的元数据、工作区捆绑和宏观架构。
   - [`reference/spec.md`](./spec.md)：操作系统实验模块标准化生成规范（规定了目录结构、依赖要求和 README 模板）。
   - [`reference/index.md`](./index.md)：本文档本身。
2. **指导书范本（重点参考）**
   - [`tg-ch3/README.md`](../tg-ch3/README.md)：标准化实验指导书的高质量范本。
   - [`tg-rcore-tutorial-ch1/README.md`](../tg-rcore-tutorial-ch1/README.md)：早期实验指导书的基础范本。
3. **实验任务与考核标准**
   - [`docs/实验2.md`](../docs/实验2.md)：个性化操作系统实验设计的潜在任务指南。
   - [`docs/实验3.md`](../docs/实验3.md)：用户态图形贪吃蛇游戏的扩展设计指南。
   - [`content.md`](../content.md)：非常重要的指导性文件，包含了 AI4OSE 实验一的目标、要求和考核标准。
   - [`report.md`](../report.md)：完成 BigLabA 后写实验总结的标准化模板（老师规定的格式）。
4. **飞书核心指导文档**
   - [校外学生：AI4OSE实验一 - 飞书云文档](https://ycn7wewvu6vl.feishu.cn/wiki/HqgYwqMHui4PSLkBWD3c8mq3n4c)
   - [校内学生：BigLabA - 飞书云文档](https://ycn7wewvu6vl.feishu.cn/wiki/BQ2ywaNSfi8po8kMNHec3Uc0nqd)
5. **项目架构与依赖**
   - [`docs/deps-ascii.md`](../docs/deps-ascii.md)：内核关系依赖图解。
   - [`README.md`](../README.md)：整个文件夹的实验介绍与宏观导航。

## 三、 根目录项目架构与导航索引 (Project Structure & Index)
以下是本工作区核心目录结构的索引说明，帮助快速定位文件功能：

```text
tg-rcore-tutorial-test/
├── docs/                 # 文档目录：存放实验任务指南与图解
│   ├── 实验2.md          # 个性化操作系统实验设计的任务指南与提示
│   ├── 实验3.md          # 贪吃蛇游戏等扩展设计的任务指南与提示
│   └── deps-ascii.md     # 内核关系依赖图解，展示各模块依赖链
├── reference/            # 标准化规范目录（AI智能体工作标准）
│   ├── index.md          # 本文档：实验项目索引与AI智能体工作指南
│   └── spec.md           # 操作系统实验模块标准化生成规范
├── scripts/              # 脚本目录
│   └── crates.txt        # 未被修改过的模范文件列表（模块清单）
├── src/                  # 源代码目录
│   └── lib.rs            # 项目元数据和子模块列表声明
├── tg-ch3/               # 已完成的基础实验模块示例（ch3）
│   ├── README.md         # 模范实验指导书
│   └── ...               # 其他源码与配置
├── tg-rcore-tutorial-ch1/ # 第一章基础实验模块
│   ├── README.md         # 模范实验指导书
│   └── ...               # 其他源码与配置
├── content.md            # AI4OSE实验一总体指导文件（含考核要求与目标）
├── README.md             # 工作区实验介绍与总体导航
└── report.md             # BigLabA 阶段的实验总结标准化模板
```

## 四、 AI 智能体工作流标准 (AI Agent Workflow Standards)

后续 AI 智能体在处理实验生成、修改或指导书撰写时，**必须**遵循以下规范：

1. **先查阅指南**：在开始新任务前，先阅读《必读文档指南》中的参考文件，确保上下文对其一致。
2. **遵守实验生成规范**：必须严格遵循 `reference/spec.md` 中的实验模块结构、依赖管理和文档排版要求。
3. **输出标准化文档**：
   - 实验主文档必须命名为 `README.md`，遵循严格的章节结构。所有文档须使用 Markdown 格式编写，图表应采用纯文本方式绘制（推荐使用 Mermaid 语法）。
   - 所有的对话建议、AI 对话过程，**以及在编程和调试的过程中遇到的问题、报错与复盘总结**，**绝对不能**放在 `README.md` 中，必须新建 `learn.md` 并存入其中。
   - **新增要求：**AI 必须将每次交互过程中的对话内容进行总结摘要，并保存到各实验目录下的 `q&a.md` 中。
   - 交互练习任务必须单独生成 `exercise.md`（参考 `tg-ch6/exercise.md`），作为配套的文字类习题及解答。
   - 须包含完整的实验代码（基于 Rust + RISC-V 64 架构），以及对应的测试用例或参考答案。
4. **依赖管理要求**：牢记最终的上传目标为 GitHub 仓库 `https://github.com/jerryinsz/ai4ose-lab1-2026s`，所有 `Cargo.toml` 依赖必须符合开源分发与 `crates.io` 标准，**禁止**乱用绝对路径，以免 Clone 失败。
5. **代码与输出规范**：代码必须带合理的中文注释，输出日志与表格符合排版要求并带中文说明，不明之处用 `TODO` 占位并提醒用户。
