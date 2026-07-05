# Architecture

CiteStage models a generative answer engine as a deterministic local pipeline. Each stage emits a `StageResult` with pass, partial, or fail status and evidence.

```mermaid
flowchart LR
  A[Corpus build] --> B[Crawl]
  B --> C[Parse Markdown]
  C --> D[Chunk and index]
  D --> E[Retrieve top-k]
  E --> F[Rerank]
  F --> G[Synthesize answer]
  G --> H[Assign citations]
  H --> I[Diagnose first failing stage]
  I --> J[Markdown report]
```

The current MVP uses a hand-rolled BM25 implementation rather than Tantivy. This keeps tests fast, avoids native dependency variance on macOS aarch64, and is sufficient for deterministic stage debugging.
