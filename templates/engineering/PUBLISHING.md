<!-- LLM: This document defines continuous delivery: how a change that passes TESTING.md
becomes a versioned artifact and safely reaches users. Cover software, packages, APIs,
infrastructure, and documentation that this project actually publishes. Link to official
platform documentation rather than copying volatile provider instructions. Interview the
user about the real release path, promotion gates, rollback, and ownership. Remove LLM
comments as you complete each section. -->

# Publishing

_How does a verified change become an artifact and reach users safely?_

## Artifacts and destinations

<!-- LLM: List what the project publishes and where it goes: application, service,
container, package, binary, schema, infrastructure, docs, or another artifact. Remove rows
that do not apply. -->

| Artifact | Destination | Versioned by | Owner |
|---|---|---|---|
| _Artifact_ | _Registry, environment, store, or host_ | _Tag, digest, release, or commit_ | _Role/team_ |

## Build and continuous delivery

<!-- LLM: Add the exact commands and automation that build, sign, package, and publish.
Explain which TESTING.md gates must pass before an artifact can move forward. -->

```sh
_build / publish command_
```

## Environments and promotion

<!-- LLM: Describe the actual path to users (for example preview -> staging -> production),
who or what approves each transition, and whether releases are gradual. Do not invent an
environment that does not exist. -->

| From | To | Required evidence / approval |
|---|---|---|
| _Environment_ | _Environment_ | _CI gate, human approval, change window, or policy_ |

## Deployment verification

<!-- LLM: Name the smoke tests, health checks, and OBSERVABILITY.md signals that confirm a
release is healthy. Separate "deployed" from "verified." -->

- _Verification and expected result._

## Rollback and recovery

<!-- LLM: State the exact rollback trigger, authority, mechanism, and data-migration caveats.
Link to a runbook when recovery is more involved than one command. -->

_How is a harmful release stopped or reversed safely?_

## Official references

- _CI/CD, registry, hosting, package, or documentation publishing reference._
