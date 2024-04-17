# Rufl贡献指南

Hi! 首先感谢你使用Rufl。

Rufl是一个功能全面、高效、可复用的Rust语言工具函数库。它消除了处理文件、数学、集合、字符串等的麻烦，使Rust开发变得更容易。

Rufl的成长离不开大家的支持，如果你愿意为rufl贡献代码或提供建议，请阅读以下内容。

## Issue 规范

- Issue仅用于提交Bug或Feature以及设计相关的内容，其它内容可能会被直接关闭。

- 在提交issue之前，请搜索相关内容是否已被提出。

- 请说明rulf和Rust的版本号，并提供操作系统信息。

## Pull Request 规范

- 请先 fork 一份到自己的项目下，不要直接在仓库下建分支。

- commit 信息要以 `type(scope): 描述信息` 的形式填写，例如 `fix(mod): [scrollbar] fix xxx bug`。

  1. type: 必须是 chore, docs, feat, fix, refactor, release, test 其中的一个。

  2. scope: 必须是 mod, file, internal 其中的一个。

  3. header: 描述信息不要超过 72 个字符。

- 提交 PR 前请 rebase，确保 commit 记录的整洁。

- 提交 PR 前请执行单元测试命令：cargo test --verbose，确保所有单元测试任务通过。

- 确保 PR 是提交到 `rc` 分支，而不是其他分支。

- 如果是修复 bug，请在 PR 中给出描述信息。

- 如果PR是新功能，确保完成相关单元测试和文档测试。