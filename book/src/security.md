# Security

SQLRustGo takes security seriously.

## Tools

| Tool | Purpose | Status |
|------|---------|--------|
| `cargo-audit` | CVE vulnerability scanning | Enabled in CI |
| `cargo-deny` | License + dependency audit | Configured |
| Dependabot | Automatic dependency updates | Weekly checks |

## Reporting Vulnerabilities

Please report security issues via [GitHub Issues](https://github.com/tianhaikuo123/sqlrustgo/issues).

## Best Practices

- Keep dependencies updated (Dependabot automates this)
- Run `cargo audit` before each release
- Review `cargo deny check` for license compliance
- Never merge PRs with failing security checks
