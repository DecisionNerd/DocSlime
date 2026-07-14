# Observability

DocSlime needs evidence at two levels: whether released artifacts work and whether the documentation lifecycle improves the user and agent experience. Workflow success, a deployed site, or an available formula is configured state; verification requires observing the artifact through the path users take.

## Observable outcomes

| Outcome / requirement | Signal | Current source | Interpretation |
| --- | --- | --- | --- |
| Safe scaffolding / FR-1, FR-2 | End-to-end CLI tests and throwaway-repo release smoke | `tests/cli.rs`, release verification | The CLI emits the expected tree without overwriting work. |
| Installability / NFR-3 | Release assets, Homebrew formula, installed version smoke | GitHub Releases, Homebrew | Users can obtain the current artifact through supported channels. |
| Skill usability / FR-9, FR-16 | Structural validation and fresh-install inspection | CI, `npx skills add` smoke | Agents receive the intended lifecycle guidance. |
| Design-context discovery / FR-11 | `impeccable` context resolution | Local context check | Canonical product/design context is discoverable without bridges. |
| Publication / FR-15 | Site build plus deployed route smoke | `docmd`, configured host | Generated docs are available and navigable. |
| Product learning / FR-18 | Issues, user feedback, adoption signals, and failed journeys | Repository and distribution channels | Evidence feeds new discovery artifacts or requirement changes. |

## Service and release health

DocSlime has no persistent application service today. Its operational health is therefore artifact-oriented:

- CI is green for the exact source revision being promoted.
- Release artifacts exist for every supported target and match the tagged version.
- Homebrew installs or upgrades to the expected version.
- `docslime init`, `list`, `add`, and `add adr` succeed in a throwaway repository.
- Published documentation routes resolve and navigation contains the current lifecycle docs.

## Telemetry and privacy

The CLI does not collect product telemetry. Do not add tracking that sends repository paths, document contents, command arguments, or other local project data without an explicit product decision, privacy review, and user consent. Prefer aggregate public distribution signals and direct feedback when evaluating adoption.

## Alerts and response

| Signal | Trigger | Response |
| --- | --- | --- |
| CI | Required job fails | Block promotion and fix the owning surface. |
| Release | Expected artifact or formula missing | Stop verification; inspect the release workflow and credentials. |
| Install smoke | Installed version or scaffold differs | Treat the release as unverified and repair distribution before announcing it. |
| Site | Build or important route fails | Restore the last known-good output or source revision and rebuild. |

## Production learning loop

Maintainer and user feedback, issues, adoption signals, release failures, and observed agent confusion should be reviewed as discovery evidence. Material findings update [`../experience/`](../experience/), then flow through requirements, architecture, tests, publishing, and a new production check. Dashboards are not a substitute for this review.
