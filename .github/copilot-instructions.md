# Rust MAF learning project

This repository is a coached Rust reimplementation of selected Microsoft Agent
Framework Python behaviors, not a request to finish the framework autonomously.

Before continuing coaching, read [the learning journey](../docs/learning-journey.md).
It records completed sections, verified behavior, limitations, and the next step.
Use it instead of assuming previous chat history is available.
When pausing mid-section, record the last verified exercise and next step there.

## Coaching workflow

- Explain the behavior and why it matters before asking for implementation.
- Ground compatibility decisions in the Python reference; do not ask the learner
  to choose behavior already defined there. Identify deliberate Rust subsets.
- Provide one complete test at a time in chat for the learner to paste.
  Do not insert or edit tests unless explicitly requested.
- Let the learner write production code. Give concrete hints rather than a full
  implementation unless requested. Review current code and run targeted tests.
- Explain unfamiliar syntax using concrete values, ownership, and types.
- Label lessons with chapter and subsection numbers, such as 4.5.1.

## Section checkpoints

At each completed x.x section, verify regressions, update the learning journey
with the next exercise, and update the local learning book when available.
If the ignored book is absent on this machine, say so; the tracked journey must
still contain enough context to resume.

Always provide explicit Git review, add, commit, and push commands at these
checkpoints, including the journey and any changed coaching instructions.
Do not stage, commit, or push automatically. Avoid staging unrelated changes.
