# Evaluation

The MVP evaluation loop is:

1. Build a fixture corpus containing a target, competitors, and distractors.
2. Run `citestage run --query ...` to produce a JSON trace.
3. Apply a documentation repair.
4. Run the same query again.
5. Compare target ranks, citation presence, and primary failure class.

Future evaluation work should add benchmark corpora, human-labeled expected citations, hybrid lexical/vector retrieval, and controlled AnswerCI experiments.
