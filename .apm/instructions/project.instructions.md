# CiteStage project instructions

CiteStage is a Rust workspace for deterministic, stage-level diagnosis of citation failures in generative answer-engine pipelines.

## Working rules

- Preserve deterministic corpus, trace, diagnosis, and repair-plan behavior for fixture-based evaluation.
- Keep the stage taxonomy aligned with the documented crawl → parse → index → retrieve → rerank → synthesize → cite pipeline.
- Diagnosis output must be evidence-backed: cite stage records, trace fields, or corpus snippets rather than unsupported guesses.
- Keep CLI behavior, docs, examples, schemas, and reports aligned when trace or report formats change.
- Prefer small, focused Rust functions with explicit error handling and helpful diagnostics.
- Never log secrets, private corpus contents beyond intended report excerpts, or raw environment variables.

## Validation

Run these before submitting code changes:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
nix flake check
```

If Nix is unavailable, still run the Cargo checks and note that `nix flake check` was skipped.
