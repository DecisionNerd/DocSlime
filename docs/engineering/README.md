# Engineering

Engineering carries DocSlime's [`requirements contract`](../REQUIREMENTS.md) through system
design, pre-release evidence, artifact delivery, and production learning.

## Lifecycle

| Document | Responsibility |
|---|---|
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | Rust CLI boundaries, embedded templates, skills, and site architecture. |
| [`TESTING.md`](TESTING.md) | Unit, integration, skill, and site checks plus CI gates. |
| [`PUBLISHING.md`](PUBLISHING.md) | CLI release, Homebrew publication, skill distribution, site build, deployment verification, and rollback. |
| [`OBSERVABILITY.md`](OBSERVABILITY.md) | Release verification, adoption signals, failures, and product-learning feedback. |
| [`adrs/`](adrs/) | Durable architectural decisions and their consequences. |

## Supporting references

- `Cargo.toml` and `Cargo.lock` define the Rust package and release version.
- `.github/workflows/ci.yml` owns continuous-integration gates and branch policy.
- `.github/workflows/release.yml` is the tag-driven `cargo-dist` release path.
- `docmd.config.json` defines the documentation-site build and navigation.
- `.agents/skills/` contains the bundled DocSlime lifecycle skills.

Create the next decision record with:

```sh
docslime add adr <short-slug>
```
