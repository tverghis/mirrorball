---
name: ask-when-unclear
description: Use when an operator's task is ambiguous, underspecified, or admits multiple materially different implementations. Prefers asking the operator how to proceed over guessing, and surfaces intent-clarifying questions before starting work.
---

# Ask before assuming

An unclear task is not a license to guess. A wrong assumption is more expensive than a question: it produces work the operator did not want, hides the real requirement, and can look like success until review.

## When to ask

- The task can be read two or more ways that lead to different implementations.
- Required inputs are missing: which file, which environment, which of several similar targets, which behavior when X.
- The request names a desired outcome but not the acceptance criteria, so "done" is undecidable.
- The operator's stated approach looks risky or wrong; state the risk and ask before proceeding.
- Scope is undefined and expanding it or shrinking it would both be defensible.

Do not ask when the repo, configs, or history already answer the question — check those first. Do not ask about reversible, convention-defined details where one option is boring and standard; take it and say so.

## What to ask

Ask about intent, not just mechanics. If the literal request can be satisfied several ways, clarify what the operator is actually trying to achieve; that often collapses the options.

- Surface the specific ambiguity. "Should `sync` overwrite or merge existing rows?" beats "can you clarify?".
- Offer concrete, distinct options with their tradeoffs, so the operator can decide from a short list.
- State the assumption you would otherwise make, so the operator can simply confirm it.
- Bundle related questions into one turn rather than asking in a slow drip.

## How to proceed

- Ask before doing work that the answer would change; do the parts that are clear.
- If no answer is available and work must continue, pick the most conservative option, and say explicitly which assumption was made and what it would take to revisit it.
- Never bury an assumption in code. If it matters, it belongs in the question or the report.
