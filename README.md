# VectCut Skills - Rust CLI for VectCutAPI

> 用 Rust 封装的剪映自动化工具，通过 VectCutAPI 控制 CapCut/剪映草稿

## 📦 项目概述

这是一个基于 Rust 的命令行工具，用于调用 [VectCutAPI](https://github.com/sun-guannan/VectCutAPI) 实现剪映视频自动化编辑。

### 架构设计

```
┌─────────────────┐
│  vectcut CLI    │  ← 你的 Rust 工具（客户端）
├─────────────────┤
│  VectCutClient  │  ← HTTP API 封装
└────────┬────────┘
         │ HTTP Request
         ▼
┌─────────────────┐
│  VectCutAPI     │  ← Python 服务（独立运行）
│  :9001          │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  draft_content  │  ← 剪映草稿文件
│  .json          │
└─────────────────┘
```

## 🚀 快速开始

### 1. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 克隆并启动 VectCutAPI 服务

```bash
# 1. 克隆 VectCutAPI
git clone https://github.com/sun-guannan/VectCutAPI.git
cd VectCutAPI

# 2. 安装 Python 依赖
pip install -r requirements.txt

# 3. 启动 HTTP API 服务（默认端口 9001）
python capcut_server.py
```

### 3. 编译并运行 vectcut CLI

```bash
# 回到本项目目录
cd vedio-VectCutAPI-skills

# 编译（使用外置 target 目录）
cargo build --release --target-dir /Volumes/otherdata/mac/rust-target-dir/vectcut-skills

# 运行测试
./target/release/vectcut test

# 创建视频草稿
./target/release/vectcut create --video /path/to/video.mp4 --title "我的标题"
```

## 📖 使用示例

### 测试 API 连接

```bash
vectcut test
vectcut test --server http://192.168.1.100:9001
```

### 创建简单视频草稿

```bash
# 基础用法
vectcut create --video ./assets/videos/demo.mp4

# 带标题
vectcut create --video ./assets/videos/demo.mp4 --title "欢迎关注"

# 指定服务器地址
vectcut create --video ./assets/videos/demo.mp4 --server http://localhost:9001
```

## 🛠️ 开发

### 项目结构

```
.
├── Cargo.toml           # Rust 项目配置
├── README.md            # 本文档
├── src/
│   ├── main.rs          # CLI 入口
│   └── client.rs        # VectCut API 客户端
├── assets/              # 素材文件
│   ├── videos/
│   ├── audios/
│   └── images/
└── output/              # 生成的草稿文件
```

### 添加新功能

编辑 `src/client.rs` 添加新的 API 方法：

```rust
impl VectCutClient {
    pub async fn add_audio(&self, audio_url: &str, start: f64) -> Result<()> {
        // TODO: 实现
        Ok(())
    }
}
```

然后在 `src/main.rs` 中添加新的 CLI 命令。

## 🔧 技术栈

- **Rust** - 系统编程语言，高性能和内存安全
- **reqwest** - HTTP 客户端
- **tokio** - 异步运行时
- **clap** - CLI 参数解析
- **serde** - 序列化/反序列化
- **anyhow** - 错误处理

## 📝 许可证

本项目采用 Apache License 2.0 许可证。

VectCutAPI 也是 Apache 2.0 许可证，可免费用于商业用途。

## 🔗 相关资源

- [VectCutAPI GitHub](https://github.com/sun-guannan/VectCutAPI)
- [剪映草稿格式分析](https://github.com/sun-guannan/VectCutAPI/blob/main/docs/draft_format.md)

## ⚠️ 注意事项

1. **VectCutAPI 服务必须先启动** - CLI 只是客户端，需要后端服务运行
2. **草稿文件兼容性** - 剪映更新可能导致草稿格式变化
3. **本地素材路径** - 视频文件路径必须是 VectCutAPI 服务可访问的路径

## 🎯 下一步计划

- [ ] 添加音频处理功能
- [ ] 支持关键帧动画
- [ ] 批量处理视频
- [ ] 导出模板配置文件

---

**Made with ❤️ and Rust**
