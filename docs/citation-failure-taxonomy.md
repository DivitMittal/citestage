# Citation failure taxonomy

| Stage | Failure examples | Detection signal | Typical repair |
| --- | --- | --- | --- |
| Crawl | README missing, docs not included, target URL absent | Target document absent from corpus | Add root README, link docs, add llms.txt |
| Parse | Dense prose, missing definition, confusing headings | Low structural clarity or no top definition | Add one-sentence definition and clear headings |
| Index | Target text creates no useful chunks | No target chunks or chunks lack query terms | Split sections and repeat key terms naturally |
| Retrieve | Competitors outrank target for the query | Target absent from retrieval top-k | Add use-case language matching likely questions |
| Rerank | Target retrieved but demoted by clarity signals | Target rank worsens after rerank | Move quickstart/use cases up; improve structure |
| Synthesis | Evidence retrieved but not answer-shaped | Generator produces no extractive answer | Add concise answer paragraphs |
| Cite | Answer cites competitors, not target | Target absent from citation list | Make source-specific claims and canonical examples |
| Factuality | Citation exists but claim is unsupported | Human or claim-level check fails | Remove unsupported claims or add source evidence |
