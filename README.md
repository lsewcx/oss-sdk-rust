# oss-sdk-rust

阿里云对象存储（OSS）的 Rust SDK：**不依赖官方 SDK**，直接基于 OSS REST API 实现，异步优先、类型安全。

> [!IMPORTANT]
> **非官方项目。** 本项目由社区独立开发，与阿里云（Alibaba Cloud）及其关联公司无任何关系，不代表官方立场，也未获官方背书。issue / PR 请提到本仓库，不要去官方仓库。
>
> **状态：早期开发中。** 当前仓库只有 crate 骨架（`src/lib.rs` + 空的 `src/oss.rs`），**尚未实现任何 OSS API**。下文的「功能范围」是路线图而非现状；示例代码是目标 API，现在编译不过。

---

## 背景：为什么要自己写一个

阿里云官方 SDK 覆盖了 Java / Go / Python / PHP / C# / Node.js / C++ / C / Ruby / Swift / Android / iOS 等语言，**Rust 长期缺席**：

- 官方直到 **2026-05-08** 才在 crates.io 发布 `alibabacloud-oss-sdk-rust-v2`，版本号 `0.1.0-delta.1` —— 未 GA 的预览版；对应 GitHub 仓库 `aliyun/alibabacloud-oss-rust-sdk-v2` 目前只有 `LICENSE`，没有源码、文档与示例。
- 社区实现各有取舍：接口风格、依赖选择与文档完善度差异较大，维护活跃度也参差（见下方「同类项目」）。

所以本项目从零实现一套 Rust SDK：接口形态对齐官方 SDK 的语义（Bucket / Object / 分片 / 预签名），实现上直接对接 REST API，不套壳任何现有库。

## 设计原则

- **零官方 SDK 依赖**：只依赖通用 HTTP / 序列化 / 加密 crates，产物轻、可审计。
- **异步优先**：默认基于 `tokio`；同步接口后续以 feature 提供，不污染主路径。
- **类型安全**：端点、参数、返回结构以强类型表达，减少拼接字符串式的调用。
- **错误可诊断**：把 OSS 的 `ErrorCode` / `RequestId` / HTTP 状态还原成结构化错误，便于排障与重试判定。
- **默认安全**：签名、时钟偏移、重试与超时行为显式可见，不藏在黑盒里。

## 功能范围（路线图）

- [ ] **客户端与配置**：AccessKey / STS 临时凭证、region + endpoint、自定义域名（CNAME）、代理、超时与重试
- [ ] **签名**：V1、V4（含 URL 预签名）
- [ ] **Bucket**：创建 / 列举 / 删除 / 获取信息（含 region、存储类型、ACL）
- [ ] **Object 基础**：`put` / `get` / `head` / `delete` / `copy` / `list`（含分页与前缀过滤）
- [ ] **高级上传**：分片上传（multipart）、断点续传、并发分片
- [ ] **元数据与访问控制**：自定义元数据、Object ACL、标签、存储类型转换
- [ ] **预签名 URL**：上传 / 下载 / 带回调的预签名
- [ ] **流式读写**：`AsyncRead` / `AsyncWrite` 与 bytes 缓冲双路径，避免整对象入内存
- [ ] **可选特性**：`blocking`（同步 API）、`serde`（结构体序列化）、按需子模块裁剪

## 目标 API（尚未实现，仅示意形态）

```rust
use oss_sdk_rust::{Client, Config};

#[tokio::main]
async fn main() -> Result<(), oss_sdk_rust::Error> {
    let client = Client::new(Config {
        access_key_id: std::env::var("OSS_ACCESS_KEY_ID").unwrap(),
        access_key_secret: std::env::var("OSS_ACCESS_KEY_SECRET").unwrap(),
        endpoint: "oss-cn-hangzhou.aliyuncs.com".into(),
        ..Default::default()
    });

    client.put_object("my-bucket", "hello.txt", b"hello oss".to_vec()).await?;

    let bytes = client.get_object("my-bucket", "hello.txt").await?;

    let url = client.presign_get("my-bucket", "hello.txt", std::time::Duration::from_secs(600))?;

    let _ = (bytes, url);
    Ok(())
}
```

## 构建与测试

需要 Rust **1.85+**（edition 2024）。

```bash
cargo build
cargo test
```

## 认证与配置

凭证建议通过环境变量注入，不要写进代码或提交进仓库：

| 变量 | 说明 |
| --- | --- |
| `OSS_ACCESS_KEY_ID` | AccessKey ID |
| `OSS_ACCESS_KEY_SECRET` | AccessKey Secret |
| `OSS_SESSION_TOKEN` | STS 临时凭证的 SecurityToken（仅临时授权时需要） |
| `OSS_ENDPOINT` | 例如 `oss-cn-hangzhou.aliyuncs.com` |

AccessKey 请在阿里云控制台用 RAM 子账号创建，并最小化授权。主账号 AccessKey 权限过大，不建议使用。

## 同类项目

截至 2026-09-23，crates.io 上与 OSS 相关的主要选择：

| crate | 版本 | 最近更新 | 性质 |
| --- | --- | --- | --- |
| [`alibabacloud-oss-sdk-rust-v2`](https://crates.io/crates/alibabacloud-oss-sdk-rust-v2) | 0.1.0-delta.1 | 2026-05-08 | 官方，预览版未 GA |
| [`aliyun-oss-client`](https://crates.io/crates/aliyun-oss-client) | 0.13.3 | 2026-06-01 | 社区，使用较广 |
| [`aliyun-oss-rust-sdk`](https://crates.io/crates/aliyun-oss-rust-sdk) | 0.2.2 | 2025-08-27 | 社区 |
| [`aliyun-oss-rs`](https://crates.io/crates/aliyun-oss-rs) | 0.3.0 | 2026-01-21 | 社区 |
| [`opendal`](https://crates.io/crates/opendal)（`services-oss`） | 0.59.3 | 2026-09-22 | 通用对象存储抽象层，非 OSS 专用 |
| [`reqsign-aliyun-oss`](https://crates.io/crates/reqsign-aliyun-oss) | 3.1.5 | 2026-08-24 | 只提供签名，需自行组请求 |

需要成熟方案或统一抽象时，优先考虑官方预览版与 OpenDAL；本项目面向希望直接掌控 OSS 语义与依赖面的场景。

## 贡献

早期阶段接口还会大改，欢迎提 issue 讨论设计；提交代码前请先开 issue 对齐方向，避免白做。

## 许可证

[Apache-2.0](LICENSE)。

「阿里云」「Alibaba Cloud」「OSS」是阿里云及其关联公司的商标。本项目仅为对 OSS 公开 API 的独立实现，对上述名称的使用仅用于说明兼容对象。
