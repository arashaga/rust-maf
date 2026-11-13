# Rust MAF learning journey

This is the portable coaching checkpoint for this repository. Commit and push
it with each completed x.x section so a new Copilot conversation on another
computer can continue without access to earlier chats.

## Resume here

- Last completed section: **4.6, optional response ID and response JSON
  validation subset**, completed 13 November 2026. All **89 project tests**
  pass. Section 4.7 has not started; no commit or push was performed by the coach.
- Section **4.4 is complete for the initial reasoning-text subset**, not the
  full Microsoft Agent Framework (MAF) reasoning-content contract.
- Section **4.5 is complete for the deliberate Rust subset**: Citation
  annotations have optional title and URL, are stored on Text content, and
  survive Message and AgentSession JSON round trips. This is not full MAF
  annotation parity; attachment to other content kinds and additional
  annotation fields remain deferred.
- Next lesson: **4.7.1 - preserve a nonempty structured response**. Explain
  why an empty-message JSON round trip does not verify preservation of nested
  content. Provide one complete `response_json_round_trip_preserves_structured_messages`
  test in chat for the learner to paste; do not insert it.
- The next test should use a response with an ID and a message containing
  reasoning text followed by cited ordinary text. Round-trip the whole
  ChatResponse and check the response ID, message count, ordered serialized
  contents, and answer-only text view. This verifies the existing Rust subset,
  not full Python wire compatibility. No new production feature is prescribed.
- **4.6 validation design verified:** Serde reads `ChatResponseInput`, then
  invokes `TryFrom<ChatResponseInput>`. The conversion moves messages and
  response_id into ChatResponse and returns a descriptive error for a nonempty
  mismatched string. Missing, null, empty, and correct string tags are accepted.
  The helper expects string-or-null tags; other Python truthiness cases are
  outside this deliberate subset. The serialized type tag remains unchanged.
- **4.6.1 reference inspection:** the available local Python `ChatResponse`
  has response-level `response_id`, `conversation_id`, `model`, `created_at`,
  `finish_reason`, `usage_details`, structured-output and continuation fields.
  Its response ID is optional, while author and message IDs belong to Message.
  Its text view joins message texts; metadata is not inserted into that text.
  The local snapshot revision is not recorded, so do not claim full parity.
- **4.6.2 verified:** `ChatResponse` stores `response_id: Option<String>`.
  The existing constructor starts at None; `response_id(&self)` borrows it
  as `Option<&str>`, and `set_response_id` can set or clear it. The learner's
  test verifies both cases and an unchanged text view. `cargo test --lib
  response --quiet` passed all 6 selected tests at this exercise. Response
  serialization was added later in 4.6.4; sessions still store messages only.
- **4.6.3 verified:** `cargo test --test agent_public_api
  agent_returns_response_id_from_client --quiet` passes. A fake public
  `ChatClient` returns an ID, `Agent::run` returns it with the answer, and the
  session history contains the conversation messages. The test does not show
  that a session persists the response ID; the current session stores messages.
- **4.6.4 verified:** the learner derived Serialize and Deserialize for
  ChatResponse and configured the emitted `type: "chat_response"` tag. The
  new empty-message JSON round trip preserves a supplied response ID; `cargo
  test --lib response --quiet` passes all 7 selected tests. This does not yet
  prove nested-message JSON parity or that deserialization validates the type
  tag; Python rejects a supplied nonempty mismatched type.
- **4.6.5 verified:** `response_id` now has
  `#[serde(skip_serializing_if = "Option::is_none")]`. The absent-ID JSON test
  passes and restores None; `cargo test --lib response --quiet` passes all 8
  selected tests. An independent scratch probe confirmed that the struct tag
  emits the correct type but does not reject a wrong type during reading.
- **4.6.6 verified (13 November 2026):** the learner implemented TryFrom and
  the condition `kind != "chat_response" && !kind.is_empty()`. Both
  `response_json_rejects_mismatched_type` and
  `response_json_accepts_null_and_empty_type` pass. The learner asked why an
  empty string passes: the condition controls rejection, so `true && false`
  skips Err and reaches Ok. Missing and null tags become None; an empty string
  becomes Some("") and needs the inner nonempty check.
- **4.6.7 checkpoint (13 November 2026):** `cargo test response --quiet`
  passes 10 library, 1 public API, and 1 async-practice test. The complete
  `cargo test --quiet` suite passes **89 tests**: 70 library, 9 public API,
  8 async-practice, and 2 session integration tests. Existing compiler warnings
  remain. The local manuscript and outline include structured 4.6 coaching;
  the rebuilt PDF validates at **116 pages, 91 bookmarks, and 189 links**.
- **4.6 limits:** this completes only the initial response-ID/JSON subset.
  Other response metadata remains deferred. Sessions store messages, not
  response IDs. The response JSON tests so far use empty message lists; full
  nested-response JSON compatibility is not established. Rust requires a
  messages array while the Python constructor can default to an empty list.
- **Response ID explanation:** it identifies a particular provider response,
  unlike a session ID for the ongoing conversation or a message ID for one
  message. It can help correlate a response with provider logs or supported
  follow-up operations. IDs are optional; the Rust constructor does not
  generate one, and their presence does not itself fetch or persist anything.
- **4.5.1 contract inspection (11 November 2026):** inspected the current upstream
  Python reference because the ignored local Python snapshot is absent.
  `Annotation` is a `TypedDict` with optional citation fields; Content accepts
  `annotations: Sequence[Annotation] | None` as a common field and stores it.
  Annotations attach to a content item, not directly to the whole message and
  not as a separate citation content variant. They describe sources without
  replacing the answer text. This is a fresh upstream check, not proof of
  parity with the unversioned snapshot used in earlier lessons.
- The 4.5 exercise entries below are chronological: earlier failing states are
  historical, and later entries record their fixes and passing tests.
- **4.5.1 comprehension checked:** the learner correctly identified that
  `Message::text()` returns only the answer and storing only that string loses
  citations. Reading the text view itself does not delete stored annotations.
- Initial planned Rust subset: a Serde-tagged `Annotation` enum with a Citation
  variant and optional `title` and `url` strings, omitted when absent.
  This is a deliberate subset of the Python fields, not full annotation parity;
  other source fields, spans, coordinate spaces, and arbitrary metadata are
  deferred. Annotation attachment to other content kinds is also deferred.
- **4.5.2 implementation reviewed:** the learner added `Annotation::Citation`
  with `Option<String>` title and URL fields and pasted
  `citation_annotation_round_trips`. The targeted test compiles but fails during
  deserialization: Serde expects an externally tagged, single-key enum object.
  Next action: compare Annotation's enum attributes with Content's internal
  `type` tag and snake_case naming, add the missing configuration, then rerun
  `cargo test --lib citation_annotation_round_trips`.
  The populated-citation test has not passed yet. Optional-field omission is
  also not configured and needs a subsequent test. No production code or tests
  were edited by the coach. Existing unrelated compiler warnings remain.
- **4.5.2 follow-up review:** the learner added the correct Serde tag and
  snake_case configuration, but placed it before the derive attribute.
  The targeted command now fails compilation with `legacy_derive_helpers`.
  Move derive above the Serde helper attribute. Restore the Rust variant name
  from `citation` to `Citation`; `rename_all` supplies lowercase JSON naming.
  Rerun the same test after these changes; no passing result is verified yet.
- **4.5.2 verified (11 November 2026):** derive now precedes the Serde helper
  attribute. `cargo test --lib citation` passes the populated-citation test
  (1 passed, 60 filtered out). The variant is still lowercase `citation`;
  rename it to `Citation` to follow Rust naming conventions. The existing
  snake_case Serde configuration preserves `"type": "citation"`.
  Next: paste the 4.5.3 absent-field round-trip test and configure omission
  independently on title and URL. Expect an assertion mismatch because None
  currently serializes as null. Section 4.5 remains in progress.
- **4.5.3 verified (11 November 2026):** `cargo test --lib
  citation_annotation` passes both the populated and absent-field round trips
  (2 passed, 60 filtered out). Missing title and URL are omitted, while explicit
  empty strings remain present. The Rust variant is still lowercase
  `citation`; rename it to `Citation` as a small naming cleanup, preserving the
  JSON tag via `rename_all`. Next, test attaching and round-tripping one
  citation on Text content while confirming `Content::text()` still returns
  only the answer text. Section 4.5 remains in progress.
- **4.5.4 verified (11 November 2026):** `Content::Text` now stores optional
  annotations, `from_text` initializes the field to None, and the text view
  ignores metadata. `cargo test --lib
  text_content_preserves_optional_citation_annotations` passes (1 passed, 62
  filtered out): an annotated Text item round-trips, the text view is unchanged,
  and older plain-Text JSON remains unchanged. Next, test the same guarantee
  through Message serialization and its answer-text view.
- **4.5.5 verified (11 November 2026):** `cargo test --lib
  message_preserves_citation_without_adding_it_to_answer_text` passes (1 passed,
  63 filtered out). A Message retains its citation in serialized contents, while
  `Message::text()` returns only `"Rust source"`. Next, test preserving that
  annotated message through AgentSession JSON save and restore.
- **4.5.6 verified (11 November 2026):** after moving the test to
  `src/sessions_test.rs`, `cargo test --lib
  session_json_round_trip_preserves_citation_annotation` passes. It verifies
  session identity, answer text, and exact citation JSON after save/restore.
- **4.5 checkpoint (11 November 2026):** `cargo test --quiet` passes all
  **83 tests**: 65 library, 8 public API, 8 async-practice, and 2 session
  integration tests. Existing compiler warnings remain, including the lowercase
  Rust variant name `citation`; tests are green despite those warnings.
- The ignored `learning-book` directory was absent on the machine used for the
  4.5 checkpoint. On resuming on a PC where it is available, the 4.5 manuscript
  and outline were updated; that PDF validated at 111 pages, and all 13
  companion-example tests passed. The current 4.6 PDF is 116 pages and remains
  ignored by Git; pushing this journey does not transfer the book.
- Do not infer completion of later lessons from the roadmap below.

## Coaching agreement

The learner wants to become a stronger engineer by implementing the Rust
framework, not receiving an unexplained finished solution.

1. Label lessons with chapter and subsection numbers.
2. Explain behavior first with concrete examples. Define Rust/Serde terms in
   plain language before using shorthand (for example, explain that
   `rename_all = "snake_case"` turns the Rust variant `Citation` into the JSON
   tag `"citation"`). When a test fails, distinguish compiler errors from
   assertion failures, explain the exact message and where it points, and state
   a concrete next edit. Avoid vague phrases such as “metadata is not preserved”;
   say exactly whether the JSON field is dropped or the `text()` view omits it.
   Show why a compatibility choice follows the Python
   source; do not make the novice choose compatibility requirements.
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
| 4.5.1 | Inspected upstream annotation attachment and distinguished answer text from stored citation metadata |
| 4.5.2-4.5.3 | Citation JSON representation, optional fields, and omitted-versus-empty behavior |
| 4.5.4-4.5.6 | Store annotations on Text and preserve them through Message and AgentSession JSON round trips; complete for the deliberate subset |

## Current implementation contract

Read [the implementation](../src/types.rs) before extending it.

- `Content` is a Serde-tagged enum using
  `#[serde(tag = "type", rename_all = "snake_case")]`.
- `Text` stores `text: String`.
- `Text` also stores optional `annotations: Vec<Annotation>`; absent annotations
  are omitted from JSON. The implemented subset supports citations with optional
  title and URL. Annotations remain stored but are not included in
  `Content::text()` or `Message::text()`.
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
- `ChatResponse::new` initializes its optional response ID to None. Its public
  getter borrows the ID; `set_response_id` accepts an optional owned ID. A
  session currently stores messages, not the complete ChatResponse.
- `ChatResponse` derives Serde serialization and emits the `chat_response`
  type tag. Its supplied ID round-trips with an empty message list, and an
  absent ID is omitted. A wrong nonempty string type is rejected; missing,
  null, empty, and correct string tags are accepted through a private input
  helper and TryFrom conversion. Full response/message JSON parity is not
  implemented, and unknown fields are not generally preserved.
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

- **4.5:** initial citations and annotations subset complete.
- **4.6:** initial response-ID and response-JSON validation subset complete.
- **4.7:** next; begin with 4.7.1, a nonempty structured-response round trip,
  then review rich-content compatibility and serialization gaps.
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
