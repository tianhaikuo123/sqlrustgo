# SQLRustGo Release Checklist

> 本清单用于发布前的质量门禁检查，确保每个Release都满足最低质量标准。

## 📋 Release 信息

| 项目 | 内容 |
|------|------|
| **版本号** | （填写，例如 v1.0.0） |
| **发布日期** | （填写） |
| **发布负责人** | 赵天海 |
| **上一版本** | （填写） |

---

## ✅ 代码质量门禁

| # | 检查项 | 命令 | 状态 |
|---|--------|------|------|
| G1 | 代码格式化 | `cargo fmt -- --check` | ⬜ |
| G2 | Clippy Lint零警告 | `cargo clippy --all-targets -- -D warnings` | ⬜ |
| G3 | 项目编译成功 | `cargo build --all` | ⬜ |
| G4 | 所有测试通过 | `cargo test --all` | ⬜ |

## 🔒 安全门禁

| # | 检查项 | 命令 | 状态 |
|---|--------|------|------|
| S1 | 依赖安全扫描 | `cargo audit` | ⬜ |
| S2 | 许可证合规 | `cargo deny check` | ⬜ |
| S3 | Dependabot已配置 | 检查 `.github/dependabot.yml` | ⬜ |

## 📦 版本门禁

| # | 检查项 | 说明 | 状态 |
|---|--------|------|------|
| V1 | 版本号已更新 | `Cargo.toml` 中 version 字段 | ⬜ |
| V2 | CHANGELOG已更新 | 记录本版本的所有变更 | ⬜ |
| V3 | Git标签已创建 | `git tag -a vX.Y.Z` | ⬜ |

## 📄 文档门禁

| # | 检查项 | 说明 | 状态 |
|---|--------|------|------|
| D1 | README已更新 | 版本号和状态徽章 | ⬜ |
| D2 | API文档已生成 | `cargo doc --no-deps` | ⬜ |
| D3 | Release Notes已编写 | GitHub Release 描述 | ⬜ |

---

## 🚀 一键执行

运行自动化门禁脚本：

```bash
bash scripts/pre-release.sh
```

如果全部通过，输出：**ALL CHECKS PASSED — Ready for release!**

---

## 📝 发布后检查

| # | 检查项 | 状态 |
|---|--------|------|
| 1 | CI/CD 流水线通过 | ⬜ |
| 2 | GitHub Release 已创建 | ⬜ |
| 3 | Release Notes 已发布 | ⬜ |

---

*清单版本: v1.0*
*最后更新: 2026-06-23*
