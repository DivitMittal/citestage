# Contributing

Contributions are welcome — bug reports, fixes, docs improvements, and reproducible citation-failure fixtures for CiteStage.

## Setup

```sh
nix develop   # preferred: enter the pinned development shell
```

If Nix is unavailable, install the stable Rust toolchain and run the Cargo commands directly.

## Guidelines

- Format Rust code with `cargo fmt --check`.
- Run `cargo clippy --all-targets -- -D warnings` before submitting.
- Run `cargo test`; tests and examples should be deterministic on the same corpus and query.
- Run `nix flake check` when Nix is available — CI runs the same flake check.
- Keep the stage taxonomy aligned with the documented crawl → parse → index → retrieve → rerank → synthesize → cite pipeline.
- Diagnosis and repair-plan output should be evidence-backed by stage records, trace fields, or corpus excerpts.
- Update docs, schemas, examples, and reports when changing trace or report formats.
- Keep changes focused; avoid mixing retrieval/reranking behavior, corpus fixtures, and documentation rewrites in one PR.

## Submitting Changes

1. Fork the repo and create a branch: `feat/description` or `fix/description`.
2. Keep commits atomic; use [Conventional Commits](https://www.conventionalcommits.org/) format.
3. Open a PR against `main` with a clear description of what changed and why.
4. Include the query, corpus manifest, StageTrace, or before/after diagnosis when the change affects pipeline behavior.

## Reporting Issues

Open a GitHub issue with:

- Your OS, Rust version, and whether you used Nix.
- The command you ran and the full error output.
- Minimal corpus, query, and trace data needed to reproduce.
- Expected vs actual behavior.
