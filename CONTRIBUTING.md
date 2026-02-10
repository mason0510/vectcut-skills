# 贡献指南

感谢您考虑为 vectcut-skills 做出贡献！

## 开发环境设置

1. **安装 Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Fork 并克隆仓库**
   ```bash
   git clone https://github.com/mason0510/vectcut-skills.git
   cd vectcut-skills
   ```

3. **运行测试**
   ```bash
   cargo test
   ```

## 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量

## 提交 Pull Request

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 报告 Bug

请使用 GitHub Issues 并包含：
- Rust 版本
- 操作系统
- 重现步骤
- 预期行为
- 实际行为
- 错误日志

## 行为准则

请尊重所有贡献者，保持友好和专业的交流。
