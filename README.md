# anthropic-rust-sdk

一个由 AI 辅助的，将 [anthropic-sdk-typescript](https://github.com/anthropics/anthropic-sdk-typescript) 同步至 anthropic-rust-sdk 的项目。

An AI-assisted synchronization of the [anthropic-sdk-typescript](https://github.com/anthropics/anthropic-sdk-typescript) to the anthropic-rust-sdk.

## 概述

本项目以官方 TypeScript SDK 为上游参考实现，借助 AI 辅助将 API 定义、类型结构与行为逐步对齐并迁移为 Rust 实现，目标是提供与官方 SDK 能力对等的 Rust 客户端库。

## 上游依赖

本仓库通过 git submodule 引用官方 TypeScript SDK：

| 项目 | 路径 | 仓库 |
|------|------|------|
| anthropic-sdk-typescript | `anthropic-sdk-typescript/` | [anthropics/anthropic-sdk-typescript](https://github.com/anthropics/anthropic-sdk-typescript) |

### 克隆与初始化

```bash
# 克隆时一并拉取 submodule
git clone --recurse-submodules <repo-url>

# 或克隆后初始化
git submodule update --init --recursive
```

### 更新上游 SDK

```bash
cd anthropic-sdk-typescript
git fetch origin
git checkout <tag-or-commit>
cd ..
git add anthropic-sdk-typescript
git commit -m "更新 anthropic-sdk-typescript submodule"
```

## 许可证

本项目采用 [MIT License](LICENSE)。
