# Rust MAF learning journey

This is the portable coaching checkpoint for this repository. Commit and push
it with each completed x.x section so a new Copilot conversation on another
computer can continue without access to earlier chats.

## Resume here

- Last verified checkpoint: **4.4.4, session preservation**, followed by the
  **4.4.5 regression and book checkpoint** on 11 November 2026.
- Section **4.4 is complete for the initial reasoning-text subset**, not the
  full Microsoft Agent Framework (MAF) reasoning-content contract.
- Next lesson: **4.5.1 - inspect MAF's citation and annotation contract**.
  Find the reference's `Annotation` definition and how annotations attach to
  Content. Explain with a concrete cited text example before proposing a Rust
  subset or providing one test. No citation implementation or test is added yet.
- Latest `cargo test --quiet`: **78 passed**: 60 library, 8 public API,
  8 async-practice, and 2 session integration tests.
- Local book checkpoint: PDF rebuilt and validated at 108 pages; all 13
  companion-example tests pass.
- Do not infer completion of later lessons from the roadmap below.

## Coaching agreement

The learner wants to become a stronger engineer by implementing the Rust
framework, not receiving an unexplained finished solution.

1. Label lessons with chapter and subsection numbers.
2. Explain behavior first; show small, concrete examples and why a choice follows
   the Python source. Do not make the novice choose compatibility requirements.
3. Give **one complete test at a time in chat**. The learner copies and pastes
   it. Do not insert tests into files without an explicit request.
4. The learner writes production code. Offer clear hints; provide an
   implementation only when asked. Inspect their code before reviewing it.
5. Say whether a test should fail or pass and whether failure is a compilation
   error or an assertion mismatch. A regression test may pass immediately.
6. Run targeted tests during exercises and regression tests at section
   checkpoints. Do not repeatedly run the entire suite for each tiny change.
7. Keep explanations focused. Pause to trace values when syntax is unfamiliar;
   do not advance merely because a test is green.
8. At each completed **x.x** section: record progress here, update the local book
   if available, and give explicit Git review/commit/push commands. Do not execute
   Git writes automatically or stage unrelated work.
9. Preserve learner changes and do not clean up unrelated compiler warnings.

## Completed lessons

| Section | Verified learning and implementation |
| --- | --- |
| Foundations | Behavior contracts, ownership and borrowing, String and str, Option, Result, structs, traits, history, and session identity |
| 1 | Library exports, private modules, and integration tests as external callers |
| 2 | Stable session IDs, JSON save/restore, malformed-input errors, optional metadata, message order, and restored conversation continuation |
| 3 through 3.8 | Async functions and futures, await, channels, controlled clients, pending behavior, error propagation, and successful-history preservation |
| 4.1 | Separate Message, Content, annotations, and response metadata; define text as a view rather than destructive flattening |
| 4.2 | Tagged Text and FunctionCall content, optional/raw arguments, string normalization, mixed messages, public content access, and session compatibility |
| 4.2.9.2 revisit | Trace Vec<String> through into_iter, map, collect, Vec<Content>, storage, and a borrowed message text view |
| 4.3 through 4.3.7 | FunctionResult, string and item constructors, exact JSON shapes, summary filtering, nested-item preservation, session round trips, shared construction, and regression checkpoint |
| 4.4.1 | TextReasoning JSON round trip; preserve returned reasoning-related text without treating it as ordinary answer text |
| 4.4.2 | Omit absent reasoning text while preserving an explicitly empty string |
| 4.4.3 | A message keeps reasoning and answer items in order; its text view contains only the answer |
| 4.4.4 | Session JSON preserves reasoning items, answer text, role, history length, and stable identity |
| 4.4.5 | Full regression verification and learning-book/portable-progress checkpoint |

## Current implementation contract

Read [the implementation](../src/types.rs) before extending it.

- `Content` is a Serde-tagged enum using
  `#[serde(tag = "type", rename_all = "snake_case")]`.
- `Text` stores `text: String`.
- `FunctionCall` stores `call_id`, `name`, and optional JSON `arguments`.
  Absent arguments are omitted. Raw strings, including incomplete fragments,
  stay strings rather than being parsed automatically.
- `FunctionResult` stores `call_id`, optional JSON `result`, and optional
  `items: Vec<Content>`. Both optional fields are omitted when absent.
- `TextReasoning` stores optional `text: String`, omitted when absent.
  `Some("")` remains `"text": ""`; it is not the same as absent text.
- `Content::text()` returns `Some(&str)` **only** for Text. It returns None
  for calls, results, and reasoning content even if they contain text internally.
- `Content::from_function_result_items(call_id, items)` preserves every item in
  order. It derives a summary from non-empty **direct Text items**, joined with
  newlines. Whitespace-only text survives; nested results are not flattened.
- `Content::from_function_result(call_id, result)` wraps the input String as one
  Text item and delegates to the items constructor.
- MAF deliberately stores both the structured items and a compatibility summary.
  Constructors derive that summary rather than maintaining independent copies.
  Public variant construction and JSON input do not enforce global consistency.
- `Message::new` accepts Vec<String> and normalizes it through `Content::from_text`.
  `Message::from_contents` accepts Vec<Content> directly.
- `Message::contents()` borrows the structured items. `Message::text()` joins only
  direct Text items using **spaces**, without deleting non-text items.
- `ChatResponse::text()` joins message text with newlines and trims outer whitespace.
- [AgentSession](../src/sessions.rs) preserves structured message history through
  `to_json` and `from_json`. Unit tests can populate it with `replace_history`;
  external callers use the public agent/session API.

Reasoning-content support here means preserving content explicitly supplied by a
provider or caller. It does not retrieve private hidden model reasoning. Excluding
reasoning from the text helper is not a security or redaction mechanism: complete
content and session JSON still contain that stored data.

## Important explanations to revisit if needed

- Iterator steps receive values from the previous step. The closure parameter
  name in `|text|` is arbitrary; it is not linked to the name `Content::text`.
- `filter_map(Content::text)` removes None, but keeps Some("").
  `.filter(|text| !text.is_empty())` then removes empty summary pieces.
- `.iter()` borrows the original items; filtering that iterator does not delete
  elements from the stored vector. `join` creates a new owned summary.
- A result inside another result was an edge-case test, not the normal template
  for every tool response. Direct text extraction does not recurse into it.
- In an integration test, `super::Content` at crate root is wrong. Import public
  types from `maf_rust`. `cargo test --lib` only runs library tests.

## Tests and commands

- [Type unit tests](../src/types_tests.rs): content shapes and text-view behavior.
- [Session unit tests](../src/sessions_test.rs): structured history restoration.
- [Public API tests](../tests/agent_public_api.rs): outside-library access.
- [Async practice](../tests/async_basics.rs): futures and channel coordination.
- [Session integration](../tests/session_serialization.rs): public save/restore.

Run from the repository root with Rust edition 2024 support:

```powershell
cargo test --lib reasoning
cargo test --lib function_result
cargo test
```

The current warnings concern unused test parameters, a test client, naming, and
public async trait future bounds. Do not silently hide them or mistake them for
test failures. Public async futures do not yet promise Send.

## Reference and book portability

The behavior reference used so far is the locally present, ignored Python
source at `python\packages\core\agent_framework\_types.py`, with tests at
`python\packages\core\tests\core\test_types.py`.

The [upstream types source](https://github.com/microsoft/agent-framework/blob/main/python/packages/core/agent_framework/_types.py)
and [upstream tests](https://github.com/microsoft/agent-framework/blob/main/python/packages/core/tests/core/test_types.py)
can help locate equivalent behavior on another machine. The local snapshot's
upstream revision is not recorded; do not assume upstream main is identical.
If local source is absent, obtain the reference or inspect upstream and clearly
recheck the contract before claiming parity. Avoid relying on old line numbers.

The illustrated PDF, manuscript, outline, renderer, validator, and companion
examples live under **Git-ignored** `learning-book\`. A push does not transfer
them. This tracked document and the repository coaching instructions are the
portable continuation record; no previous chat or PDF is required to resume.
If the learner wants the full book on another PC, it needs separate transfer or
an explicit decision to version its source.

When that local directory is available, the book commands are:

```powershell
python .\learning-book\build_book.py
python .\learning-book\validate_book.py
cargo test --manifest-path .\learning-book\examples\Cargo.toml
```

The renderer requires its Python libraries and local Windows fonts. Do not claim
a book update was performed on a machine without those files or prerequisites.

## Remaining roadmap and limits

- **4.5:** citations and annotations; start by tracing the reference contract.
- **4.6:** response metadata.
- **4.7:** rich-content compatibility and serialization review.
- **5:** one real model provider and deterministic boundary tests.
- **6:** structured outputs and metadata.
- **7:** streaming.

Only Text, FunctionCall, FunctionResult, and the initial TextReasoning subset are
implemented. There is no tool execution, automatic call/result matching, real
provider integration, streaming, or complete Python metadata/content parity.
Reasoning IDs, protected payloads, statuses, annotations, and merge behavior are
not supported yet. Unknown JSON properties are not generally preserved.

## Updating this checkpoint

After each completed x.x section, update the resume point, verified test counts,
new contracts, remaining gaps, and next exact exercise. Keep the chapter history.
When pausing mid-section, record the last verified exercise and next step as
in progress, without marking the section complete.
Use repository-relative links and never depend on machine-local session paths.
Do not mark a Git commit or push as completed unless its result was verified.

On another PC, open the repository in VS Code and tell Copilot:

> Read .github/copilot-instructions.md and docs/learning-journey.md. Check the
> current code and tests, then continue with the next numbered lesson. Give me
> one test at a time to paste; let me implement production code with hints.
