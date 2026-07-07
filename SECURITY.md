# Security Policy

Thank you for helping keep CiteStage and its users safe.

## Supported Versions

CiteStage is currently a research/MVP artifact. Security fixes are accepted for the `main` branch and the latest tagged release, if one exists.

## Reporting a Vulnerability

Please do not report security vulnerabilities through public GitHub issues.

To report a vulnerability, contact the maintainer directly by email or open a private GitHub security advisory if the repository has advisories enabled. Include:

- A concise description of the issue and its impact.
- Steps to reproduce, proof-of-concept corpus/query data, or affected files.
- Whether the issue can expose private corpus material, traces, generated reports, local files, or CI credentials.
- Any suggested mitigation or patch, if available.

The maintainer will acknowledge reports as soon as possible and coordinate a fix or disclosure timeline based on severity.

## Security Expectations

- Never commit secrets, private corpora, private traces, generated reports with sensitive excerpts, or `.env` files.
- Keep fixture-based evaluation deterministic and safe to run locally and in CI.
- Redact sensitive corpus excerpts from bug reports, screenshots, traces, and diagnosis output.
- Treat StageTrace JSON, repair plans, and report artifacts as potentially sensitive when they come from private documentation.
- Prefer least-privilege GitHub tokens and CI permissions when adding workflows or integrations.

## Scope

In scope:

- Secret exposure in examples, corpora, traces, reports, logs, or CI output.
- Unsafe handling of private corpus content or generated diagnosis artifacts.
- Vulnerabilities in CLI behavior that can overwrite unexpected files, read secrets, or mis-handle untrusted corpus input.
- Supply-chain concerns in build, Nix, Cargo, or APM metadata.

Out of scope:

- Publicly known dependency advisories without a CiteStage-specific exploit path.
- Findings that require malicious local shell access beyond the repository's documented command surface.
- Availability-only reports against public services not controlled by this project.
