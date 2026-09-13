---
name: comment-discipline
description: Use when writing, editing, or reviewing code in this project, to decide whether a comment belongs. Enforces that code and module comments are added only when the intent is not obvious to the reader.
---

# Comments: default to none

Write a comment only when it tells the reader something the code cannot. If it restates what the code plainly does.

## Do not add

- Comments restating the declaration below/above them: `// Returns the chunk size`, `// The chunk size table`.
- Module docs (`//!`) that paraphrase the filename or the module's own name.
- Doc comments whose whole content is the parameter name restated: `/// file_size: the size of the file`.
- Section banners (`// --- helpers ---`), numbered step narration, `// increment i`, `// loop over chunks`.
- Comments repeating a function or variable name.
- Explanations of standard-library or language behavior.
- Ownerless `// TODO`s — either fix it or leave it out.

## Comments that may be added

- Non-obvious invariants, preconditions, or postconditions, e.g. `// Caller must hold the write lock`.
- Why, when the code only shows what: workarounds, protocol quirks, external-format requirements. Give the reason and a link or source when it comes from outside.
- Units, valid ranges, and magic numbers the type does not express, e.g. `// Sizes are in bytes`.
- Justification for `unsafe` or other deliberately-risky code.
- Anything whose absence would make a competent reader stop and reach for `git blame`.
