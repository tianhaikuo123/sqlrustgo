# SQLRustGo Release Gate Checklist

> 版本发布前必须通过以下所有检查项。请在发布前逐项确认。

## 📋 代码质量门禁

- [ ] `cargo fmt --all -- --check` — 代码格式检查通过
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` — Clippy零警告
- [ ] `cargo build --all-features` — 编译成功

## 🧪 测试门禁

- [ ] `cargo test --all-features` — 所有测试通过
- [ ] 测试覆盖率 ≥ 80%（可选：`cargo llvm-cov --all-features`）

## 🔒 安全门禁

- [ ] `cargo audit` — 无已知安全漏洞
- [ ] `cargo deny check` — 许可证合规（可选）
- [ ] Dependabot 已启用且无高危预警

## 📦 版本门禁

- [ ] `Cargo.toml` 中版本号已更新
- [ ] `CHANGELOG.md` 已更新（如存在）
- [ ] Git 标签与 Cargo.toml 版本一致

## 📄 文档门禁

- [ ] `README.md` 内容与当前版本一致
- [ ] API 文档已生成（`cargo doc --no-deps`）
- [ ] Release Notes 已编写

## ✅ 最终确认

- [ ] 上述所有门禁通过
- [ ] 分支已合并到 main
- [ ] 准备创建 Git 标签和 GitHub Release

---

## 🚀 自动化检查

运行以下脚本可自动执行大部分门禁：

```bash
# Linux / macOS
bash scripts/pre-release.sh

# Windows (Git Bash)
bash scripts/pre-release.sh
```

---

*最后更新：2026-06-22*
