---
name: update-version
description: 更新 anthropic-rust-sdk 的 crate 版本号并与上游同步，确保全部位置一致、避免遗漏。覆盖上游 tag 拉取、Cargo.toml、src/internal/mod.rs 的 SDK_VERSION、README.md 依赖示例、docs/ARCHITECTURE.md 版本说明、Cargo.lock 与 Git 标签。当用户要求更新版本号、bump version、发版，或将 crate 版本与上游 anthropic-sdk-typescript 对齐时使用。
---

# 更新版本号

本项目的 crate 版本号与上游 `anthropic-sdk-typescript` 保持一致。更新版本时必须同步下列全部位置，缺一不可，否则会出现版本号不一致。

> 重要教训：即便只是 patch 级 bump（例如 `0.106.0` 到 `0.106.1`），也必须同步 `SDK_VERSION` 与 `docs/ARCHITECTURE.md`，不能只改 `Cargo.toml`。历史上正是漏了这两处才导致版本号不一致。

## 版本号位置清单

| 位置 | 内容 | 操作方式 |
|------|------|----------|
| `Cargo.toml` -> `[package].version` | crate 版本 | 手动修改 |
| `src/internal/mod.rs` -> `SDK_VERSION` | 用于 `x-stainless-package-version` 请求头 | 手动修改 |
| `README.md` -> 依赖示例 `anthropic-rust-sdk = "X.Y"` | 安装文档 | 手动修改（仅写主次号，省略 patch） |
| `docs/ARCHITECTURE.md` -> 「版本」小节 | 架构文档（参考版本、crate 版本共两处） | 手动修改 |
| `Cargo.lock` -> 本 crate 条目 | 锁定版本 | 不手改，运行 `cargo build` 自动更新 |
| `anthropic-sdk-typescript/src/version.ts` -> `VERSION` | 上游版本来源 | 只读，用于确定目标版本 |
| Git 标签 `sdk-v<version>` | 同步点标签 | `git tag -f sdk-v<version>` |

## 更新步骤

复制以下清单并逐项勾选：

```
- [ ] 0. 同步上游、确定目标版本
- [ ] 1. 修改全部「手动修改」位置
- [ ] 2. cargo build 更新 Cargo.lock
- [ ] 3. 一致性校验（三处核心版本号 + 全局残留）
- [ ] 4. cargo fmt/clippy/test 验证
- [ ] 5. 提交（chore(version) 中英双语）
- [ ] 6. 创建或移动 Git 标签
```

**步骤 0：同步上游、确定目标版本**

若为「同步上游」场景，先在子模块内拉取最新标签并切换（子模块本地可能未 fetch 到最新版本）：

```bash
git -C anthropic-sdk-typescript fetch --tags origin
git -C anthropic-sdk-typescript checkout sdk-v<version>
```

确定「最新版本号」的只读方式（无需修改本地即可查询）：

```bash
git -C anthropic-sdk-typescript ls-remote --tags origin "refs/tags/sdk-v*"
```

也可查询 npm 最新 release：`https://registry.npmjs.org/@anthropic-ai/sdk/latest`。

切换后读取 `anthropic-sdk-typescript/src/version.ts` 的 `VERSION` 确认目标版本；crate 版本与之一致。子模块指针改动单独提交（`build(submodule): 同步子模块至 sdk-v<version>`）。

**步骤 1：修改手动位置**

按上表逐一修改。`README.md` 依赖示例只写主次号（如 `0.110`），其余位置写完整三段号（如 `0.110.0`）。`docs/ARCHITECTURE.md` 有两处需改（参考版本、crate 版本）。

**步骤 2：构建更新锁文件**

```bash
cargo build
```

`cargo build` 会把 `Cargo.lock` 中本 crate 的版本同步为新值，无需手动编辑该文件。

**步骤 3：一致性校验**

先确认三处最易遗漏的核心版本号是否一致（限定文件，避免全仓库扫描）：

```bash
rg -n "SDK_VERSION|^version" Cargo.toml src/internal/mod.rs docs/ARCHITECTURE.md
```

再检查旧版本号残留。优先用 `git grep`（只搜父仓库跟踪文件，天然排除子模块与 `target/`，比全仓库 `rg` 更快更稳，避免扫描子模块导致卡顿）：

```bash
git grep -nE "<旧版本号>"
```

排除干扰项：`Cargo.toml` 的依赖版本（如 `async-trait = "0.1"`）、`Cargo.lock` 中其他依赖版本（如 `1.0.106`）、`.plan/` 历史计划文件，均非本 crate 版本，勿改。

**步骤 4：质量验证**

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
```

**步骤 5：提交**

遵循 [Conventional Commits 中英双语规范](../../rules/commit-convention.mdc)，标题格式如下：

```
chore(version): 对齐 crate 版本至上游 <version> / Align crate version to upstream <version>
```

只提交版本相关文件，不混入无关改动。

**步骤 6：Git 标签**

为同步点打标签，命名与上游一致：

```bash
git tag -f sdk-v<version>
```

若该标签此前已推送到远端，移动后需 `git push -f` 才能更新远端标签；推送属于破坏性操作，执行前须先征得用户确认。

## 发布到 crates.io（可选）

发布不可逆（版本只能 yank，无法删除或覆盖），执行前须征得用户确认。

本机将 `crates-io` 源替换为镜像（见 `~/.cargo/config.toml` 的 `replace-with`），因此发布必须显式指定官方源，否则会失败：

```bash
cargo publish --dry-run --registry crates-io
cargo publish --registry crates-io
```

## 验证清单

提交前逐项确认：

- [ ] 三处核心版本号（`Cargo.toml` / `SDK_VERSION` / `ARCHITECTURE`）一致。
- [ ] `git grep -nE "<旧版本号>"` 除干扰项外无残留。
- [ ] `cargo build`、`cargo test --all` 通过。
- [ ] `Cargo.lock` 中本 crate 版本已更新为新值。
