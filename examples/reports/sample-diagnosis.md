# CiteStage Diagnosis Report

**Query:** `best NixOS home manager flake template for reproducible machines`

**Target:** `os-nixcfg`

## Pipeline trace

| Stage | Status | Target rank | Evidence |
| --- | --- | --- | --- |
| crawl | Pass | — | loaded 5 documents |
| parse | Partial | 1 | parsed 4 sections; clarity score 0.40; no one-sentence definition detected near the top |
| index | Pass | 1 | target produced 4 indexable chunks |
| retrieve | Pass | 2 | target chunk retrieved at rank 2 |
| rerank | Pass | 2 | target moved from retrieval rank 2 to rerank rank 2 |
| synthesize | Pass | — | deterministic extractive answer produced |
| cite | Pass | 2 | target cited at citation position 2 |

## Primary failure

**ParseFailure** at stage `parse`.

### Evidence

- parsed 4 sections; clarity score 0.40
- no one-sentence definition detected near the top

### Suggested repairs

**Make the README parse cleanly**

- Add a one-sentence definition near the top
- Use descriptive Markdown headings instead of dense prose blocks
- Move quickstart and use-cases before low-level topology details
