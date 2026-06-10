# 路线图

状态标记：✅ 已完成 | ⚠️ 警告 | 💤 延期 | ❌ 未开始 | 🚫 放弃/不支持

## 范围

### 云厂商变体（不支持）

| 项目 | 状态 | 说明 |
|------|------|------|
| AWS Bedrock (`bedrock-sdk`) | 🚫 | `packages/` 不纳入 |
| Google Vertex AI (`vertex-sdk`) | 🚫 | `packages/` 不纳入 |
| AWS API Gateway (`aws-sdk`) | 🚫 | `packages/` 不纳入 |
| Azure Foundry (`foundry-sdk`) | 🚫 | `packages/` 不纳入 |

### 可暂缓

| 项目 | 状态 | 说明 |
|------|------|------|
| Legacy Completions API | 💤 | 阶段四低优先级 |
| Node Agent Toolset | 💤 | 无 CLI/agent 场景暂不 port |
| CLI 迁移工具 | 🚫 | 不在范围内 |

## 阶段零：工程脚手架

- ✅ Cargo 工程与模块骨架
- ✅ CI（fmt / clippy / test）
- ✅ `.gitignore` / `rust-toolchain.toml`
- ✅ `docs/ARCHITECTURE.md`
- ✅ `docs/ROADMAP.md`

## 阶段一：核心客户端与 HTTP 层

- ✅ `Anthropic` 客户端与 `ClientOptions`
- ✅ 环境变量 `ANTHROPIC_API_KEY` 读取
- ✅ 错误类型树
- ✅ 指数退避重试
- ✅ HTTP 请求基础设施

## 阶段二：Messages API

- ✅ Messages 核心类型
- ✅ `messages.create()`（非流式）
- ✅ `messages.count_tokens()`
- ✅ 示例 `examples/messages_create.rs`

## 阶段三：流式与 MessageStream

- ✅ SSE 流式解析
- ✅ `messages.create(stream: true)`
- ✅ `messages.stream()` / `MessageStream`
- ✅ 示例 `examples/streaming.rs`

## 阶段四：Models、Batches、Completions

- ✅ `models.list()` / `models.retrieve()`
- ✅ `messages.batches` CRUD
- ✅ `completions.create()`（legacy）

## 阶段五：Beta API 域

- ✅ `beta` 聚合入口与 `Anthropic-Beta` header
- ✅ `beta.messages` / `beta.models`
- ✅ `beta.files` / `beta.skills`
- ✅ `beta.agents` / `beta.environments` / `beta.sessions`
- ✅ `beta.deployments` / `beta.deployment_runs`
- ✅ `beta.vaults` / `beta.memory_stores`
- ✅ `beta.webhooks` / `beta.user_profiles`

## 阶段六：Helpers 与高级运行时

- ✅ `messages.parse()` 结构化输出
- ✅ `BetaToolRunner` 工具循环骨架
- ✅ Webhook `unwrap` 验签
- ✅ Middleware 钩子
