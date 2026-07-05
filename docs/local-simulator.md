# Local simulator validity

CiteStage is not a clone of Google AI Overviews, ChatGPT browsing, Perplexity, or any proprietary answer engine. It is a local, reproducible simulator for isolating stage-level failure modes.

## What it can tell you

- Whether a target document is present in a controlled corpus.
- Whether Markdown structure creates useful chunks.
- Whether lexical retrieval and simple reranking favor competitors.
- Whether an extractive generator would have enough source-specific text to cite the target.
- Which intervention improved a modeled pipeline under fixed conditions.

## What it cannot tell you

- Exact production ranking behavior for any external answer engine.
- Effects of web-scale link graphs, freshness, authority, personalization, or hidden policies.
- Whether a real engine will cite a project after a docs patch.

Use CiteStage for controlled before/after experiments and debugging hypotheses, not as a guarantee of real-world citation outcomes.
