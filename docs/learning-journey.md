# Rust MAF learning journey

This is the portable coaching checkpoint for this repository. Commit and push
it with each completed x.x section so a new Copilot conversation on another
computer can continue without access to earlier chats.

## Resume here

- Last completed section: **5.4, offline provider response-status handling**,
  completed 16 November 2026. `cargo test --quiet` passes **129 tests**:
  74 library, 9 public API, 8 async-practice, 3 Azure config, 33
  request/response, and 2 session integration tests.
- **Resume at 5.5.1:** build an offline `reqwest` request for the Azure
  Responses API. The learner selected direct HTTP with `reqwest` instead of an
  OpenAI-compatible SDK on 16 November 2026, and chose to support both API-key
  and Entra ID authentication. Use an injected async token-provider hook for
  Entra; when both sources are configured, the token provider takes precedence,
  matching the Python reference. No reqwest dependency, transport, or live
  call exists yet. Earlier exercises and the class/data map are recorded below
  and in `docs/rust-maf-map.md`.
- **Chapter 4 status:** complete for the initial Rust subset, not full Python
  content, metadata, or JSON parity. No changes have been staged, committed,
  or pushed by the coach.
- Section 5.1 was read-only source inspection and contract planning. Since
  then, the learner has added explicit configuration and offline request and
  response mappers. Chapter 5 is still in progress; there is no transport,
  provider SDK, Azure resource, credential, or live call.
- **5.2.1 verified (13 November 2026):** `cargo test --test
  azure_openai_config` passes the single config_rejects_empty_deployment test.
  The learner created `src/azure_openai.rs` with AzureOpenAIConfig owning
  private endpoint/deployment strings and AzureOpenAIConfigError with
  EmptyDeployment. Its public constructor returns the typed error when
  deployment.is_empty(), otherwise moves both strings into the configuration.
  Both types are publicly re-exported from src/lib.rs. Compiler warnings
  remain, including unread configuration fields at this stage.
- **5.2.2 verified (13 November 2026):** `cargo test --test
  azure_openai_config` passes both configuration tests. The learner added
  public endpoint(&self) -> &str and deployment(&self) -> &str getters while
  keeping both owned strings private. config_preserves_valid_settings checks
  that successful construction retains the supplied endpoint and deployment
  exactly. The getters borrow the strings without cloning or consuming the
  configuration. The unread-configuration-fields warning is now gone;
  unrelated existing warnings remain.
- **5.2.3 verified (14 November 2026):** `cargo test --test
  azure_openai_config` passes all three tests. The learner added
  `AzureOpenAIConfigError::EmptyEndpoint` and rejects an exactly empty endpoint.
  The configuration now has private owned endpoint/deployment strings, borrowed
  getters, and typed errors for either empty required setting.
- The Python Azure settings path in `python/packages/openai/agent_framework_openai/_shared.py`
  requires endpoint or base_url when there is no supplied client, and supports
  environment-based settings. This Rust configuration deliberately has only
  explicit endpoint/deployment, with no alternate base URL or supplied-client
  fallback. Rejecting an empty endpoint is a local subset rule, not a claim
  of full Python settings parity. `_resolve_named_setting` selects a nonempty
  deployment string without trimming; that does not prove Azure will accept
  the name. The Rust constructor checks only empty strings, not whitespace or
  full URL validity.
- **5.2 regression checkpoint (14 November 2026):** `cargo test --quiet` passes
  all **96 tests**: 74 library, 9 public API, 8 async-practice, 3 Azure config,
  and 2 session integration tests. Existing compiler warnings remain. The
  ignored local `learning-book` directory was absent on the other computer,
  so no PDF update was possible there. A later update through 5.3.1 on this
  computer validated at 128 pages, 96 bookmarks, and 199 links. No
  credentials, environment reads, SDK, HTTP calls, or dependency changes
  were needed.
- **5.3.1 - map one user Text message to Responses input.**
  The learner pasted the offline test in `tests/azure_openai_request.rs`.
  It asserts that the configured deployment becomes the top-level
  `model`, and `"Hello"` becomes one `input` message with role `"user"` and
  content `[{ "type": "input_text", "text": "Hello" }]`. This is pure request
  JSON construction, not an HTTP call. Keep endpoint out of the JSON body;
  transport will use it separately. The learner drafted `build_request_body`,
  corrected the import, and uses `message.contents()` and `message.role()`.
  At the learner's explicit request, the coach fixed only the annotation
  handling and JSON macro syntax in the draft: it now rejects nonempty
  annotations with `AnnotatedText`, accepts None or an empty list, and puts
  the borrowed text in the JSON. `cargo test --test azure_openai_request
  request_body_maps_user_text_to_responses_input` passes (1 passed), but it
  covers only unannotated user text.
  The new error has an unused JSON-error conversion, which can be revisited
  rather than adding speculative error paths. A `Content::Text` is internal
  framework data; `input_text` is an Azure Responses JSON label emitted by
  the mapper, not a Rust Content variant.
- **5.3.1 reverified (14 November 2026):** after pulling commit 349abde,
  `cargo test --test azure_openai_request --test azure_openai_config` passes
  one request-mapping test and three configuration tests. This is a focused
  run, not a new full-suite result. The learner's pre-pull journey notes remain
  safely in the Git stash; relevant facts were reconciled here without
  applying or dropping that stash.
- **5.3.2 verified (14 November 2026):** `cargo test --test
  azure_openai_request` passes both request tests, including
  request_body_rejects_text_with_citation. The learner re-exported Annotation
  and AzureOpenAIRequestError from src/lib.rs, allowing the public integration
  test to construct a citation and match the typed AnnotatedText error. The
  existing guard rejects the nonempty list without changes. Python maps user
  text without these annotations; explicit rejection is our deliberately
  stricter first subset, not a claim that Python rejects it.
- **5.3.3 verified (14 November 2026):** `cargo test --test
  azure_openai_request` passes all three request tests. The learner's
  request_body_accepts_empty_annotations test verifies `Some(vec![])`
  produces the same exact user input JSON shape as ordinary unannotated Text.
  The existing guard accepts both None and an empty list, while the separate
  5.3.2 test verifies a nonempty citation list is rejected.
- **5.3.4 verified (14 November 2026):** `cargo test --test
  azure_openai_request` passes all four request tests. The learner's
  request_body_rejects_unsupported_function_call test places ordinary Text
  before a FunctionCall in an assistant message and confirms the mapper
  returns UnsupportedContent rather than a partial successful body. Python has
  broader function-call support; this rejection is a deliberate temporary
  Rust subset, not Python parity.
- **5.3.5 verified (14 November 2026):** `cargo test --test
  azure_openai_request` passes all five request tests. The learner made
  request_body mutable and uses if let Some(instruction) to insert a
  top-level "instructions" JSON string; None leaves the key absent. The
  exact-body test verifies the user input remains separate. Initially an
  unnecessary else returned &Value while assignment returned (), producing
  E0308, and the key was singular "instruction"; the learner removed the
  else and corrected the key. Python's _prepare_options retains non-None
  instructions as a top-level field rather than injecting a system message.
- **5.3.6 verified (14 November 2026):** `cargo test --test
  azure_openai_request` passes all six request tests, including
  request_body_preserves_empty_instructions. None omits the field; Some("")
  produces `"instructions": ""`. Python filters only None from options at
  this point, not empty strings.
- **5.3.7 verified (14 November 2026):** `cargo test --test
  azure_openai_request` passes all seven request tests. The learner added
  `"type": "message"` to message_json and updated the four older exact-body
  test expectations. The new request_body_labels_user_input_as_message test
  passes. The coach had previously omitted this field from those expectations;
  that omission is not established to make a real API request invalid.
  Source inspection shows `_prepare_message_for_openai` in the local Python
  Responses adapter initializes every message object with `"type":
  "message"` and `"role": message.role`. Explain that the outer type labels
  the whole input item, whereas nested input_text labels one content part;
  role says who spoke.
- **5.3.8 verified (15 November 2026):** `cargo test --test
  azure_openai_request` passes all eight request tests. The learner now maps
  user Text to input_text and assistant history Text to output_text with
  annotations: []. The test first exposed that serializing annotations: None
  directly produces JSON null; the learner changed the provider representation
  to the empty array required by the Python Responses adapter.
- **5.3.9 verified (15 November 2026):** `cargo test --test
  azure_openai_request` passes all nine request tests. The learner added
  AzureOpenAIRequestError::UnsupportedRole and checks the borrowed role once
  inside the outer message loop, before iterating content. Only "user" and
  "assistant" proceed to content mapping; a system message returns the
  dedicated error. The integration test imports the public error type.
- **5.3.10 verified (15 November 2026):** `cargo test --test
  azure_openai_request` passes all ten request tests. The learner added
  `AzureOpenAIRequestError::EmptyInput` and rejects an empty message slice.
  This is an outgoing request constraint; it does not change Chapter 4's
  valid empty ChatResponse behavior.
- **5.3.11 test verified (15 November 2026):** `cargo test --test
  azure_openai_request` passes all eleven request tests, including
  request_body_rejects_message_without_content. A lone empty-content message
  must not produce an empty-content provider message; because it contributes
  no prepared item, the whole request returns EmptyInput.
- Review found the implementation currently returns EmptyInput immediately
  for every empty-content message. The Python mapper omits that message and
  rejects only if the entire prepared input list is empty. Thus an empty
  message beside a valid one should not discard or reject the valid input.
- **5.3.12 verified (15 November 2026):** `cargo test --test
  azure_openai_request` passes all twelve request tests. Empty-content
  messages are skipped; the overall prepared input is checked after the
  message loop. A lone empty message returns EmptyInput, while an empty
  message before a valid "Hello" message is omitted and the valid input is
  preserved. The check initially sat inside the loop, where the first valid
  message necessarily saw an empty input vector; moving it after the loop
  resolved the 10 regressions. The three Azure config tests also pass in the
  focused combined run.
- **5.3.13 verified (15 November 2026):** `cargo test --test
  azure_openai_request --test azure_openai_config` passes all sixteen focused
  tests (thirteen request, three config). The new
  `request_body_preserves_text_item_order` test verifies that two Text strings
  in one user Message become two `input_text` content items in the same order.
- **5.3.14 verified (15 November 2026):** the new
  `request_body_preserves_empty_text_item` test verifies that a user Message
  containing one empty Text string maps to one `input_text` item with
  `"text": ""`. This differs from a Message whose `contents` vector has zero
  items, which the mapper skips. The Python Responses mapper emits the
  input-text object without filtering its empty string. This is a request
  mapping contract, not a claim that a live provider accepts that request.
  `cargo test --quiet` passes all 110 tests: 74 library, 9 public API,
  8 async-practice, 3 Azure config, 14 request mapping, and 2 session tests.
- **5.3.15 verified (15 November 2026):** the learner's
  `parse_response(&self, response: &Value)` maps one provider `output_text`
  item into an assistant Message and preserves both the `resp_123` response
  ID and `msg_123` message ID in the happy-path test. The learner fixed the
  missing `impl` closing brace, borrowed-method signature, and `&str` type
  comparison. `cargo test --test azure_openai_request --test azure_openai_config`
  passes all 18 focused tests (15 request/response, 3 config). The new
  `serde_json::json` import is unused. The Python `_parse_response_from_openai`
  likewise maps response output messages and retains the IDs.
- **5.3.16 verified (15 November 2026):** the learner's test places valid
  output text before a refusal content item. The learner changed both loops
  to return String errors for unsupported kinds instead of skipping them.
  `cargo test --test azure_openai_request --test azure_openai_config` passes
  all 19 focused tests (16 request/response, 3 config). The earlier E0308
  occurred because the response parser's declared error is String, not the
  request-specific AzureOpenAIRequestError. The refusal test exercises the
  inner content loop, not the outer output-item loop. Python maps refusals
  with metadata; rejecting them is a deliberate first Rust subset, not parity.
- **5.3.17 verified (15 November 2026):** the learner's test places a supported
  output message before a top-level function call and verifies an error rather
  than a partial ChatResponse. The outer output-item check already rejects
  non-message items. `cargo test --test azure_openai_request --test
  azure_openai_config` passes all 20 focused tests (17 request/response,
  3 config). Python handles function-call output; rejecting it is a
  deliberate Rust subset, not Python parity.
- **5.3.18 verified (15 November 2026):** `cargo test --test
  azure_openai_request --test azure_openai_config` passes all 21 focused
  tests (18 request/response, 3 config). The learner reads annotations as an
  array and uses `!annotations.is_empty()` to reject a nonempty list while
  accepting `[]`. An earlier inverted condition broke both the simple-response
  and citation tests; `!` reverses a boolean, so `!true` is false and `!false`
  is true. Python maps URL citations and their spans; explicit rejection is a
  deliberate temporary Rust subset, not Python parity.
- **5.3.19 verified (15 November 2026):** `cargo test --test
  azure_openai_request --test azure_openai_config` passes all 22 focused
  tests (19 request/response, 3 config). The learner moved the fallback after
  the output loop. When `output` is empty, the mapper creates one assistant
  Message with empty contents and no message ID, and retains the response ID,
  matching Python's fallback. Previously the check was inside the loop, which
  runs zero times for empty output and cannot see an empty list after a push.
  This provider-response behavior differs from rejecting an empty outgoing
  request and from allowing an empty framework ChatResponse in general.
- **5.3.20 verified (15 November 2026):** `cargo test --test
  azure_openai_request --test azure_openai_config` passes all 23 focused
  tests (20 request/response, 3 config). The new test verifies that two
  assistant output messages with distinct IDs and texts remain two messages
  in order with the correct IDs and texts. Python's response adapter also
  creates a new message when the next ID differs; this test does not
  establish parity for repeated or missing IDs.
- **5.3.21 verified (15 November 2026):** `cargo test --test
  azure_openai_request --test azure_openai_config` passes all 24 focused
  tests (21 request/response, 3 config). The learner rejects a non-assistant
  provider output role instead of returning it as a user message. Python's
  response parser creates assistant Messages; rejection is a deliberately
  stricter Rust subset, not a claim that Python rejects the same input.
- **5.3.22 verified (15 November 2026):** the learner's test confirms a
  provider `output` object returns an error instead of being treated as an
  empty output array. The full `cargo test --quiet` suite passes all 118
  tests, including the 22 request/response mapping tests and 3 config tests.
- **Section 5.3 checkpoint:** complete only for the initial offline unannotated
  text subset. Request mapping rejects unsupported content and nonempty
  annotations. Response mapping rejects unsupported kinds, roles, and nonempty
  annotations; its errors are currently String values. It maps empty output
  to one empty assistant message and preserves IDs for the tested cases.
  Other response metadata and status are ignored, and repeated/missing
  message-ID parity is not established. There is no live transport, SDK,
  authentication, or `ChatClient` implementation. The local ignored book was
  updated and rebuilt through 5.3.22; `validate_book.py` passed with 130
  pages, 96 bookmarks, and 199 links. Include the tracked journey and both
  maps in the learner's Git checkpoint; the ignored book is not in Git.
  The Rust MAF map is in `docs/rust-maf-map.md`, with an editable visual in
  `docs/rust-maf-map.excalidraw`.
- **5.4.1 verified (16 November 2026):** `response_rejects_failed_provider_status`
  confirms a response with status `failed` returns `Err` instead of being
  treated as an empty successful assistant message. The parser compares the
  optional status to `Some("failed")`, so a missing status remains accepted;
  all 23 request/response tests pass. This is a deliberate Rust safety rule,
  not a claim that the Python parser raises for this status.
- **5.4.2 verified (16 November 2026):** the test confirms that a failed
  response's `error.message` appears in the returned error. All 24
  request/response tests passed at that point. The learner extracts the
  underlying `&str` with `as_str()` so the returned text does not include JSON
  quotes.
- **5.4.3 verified (16 November 2026):** a failed response without
  `error.message` returns the generic failed-response error. The learner keeps
  the optional message as `Option<&str>` and uses `unwrap_or` before
  constructing the error, so the absent value does not terminate parsing with
  an unrelated missing-field message. All 25 request/response mapper tests
  pass.
- **5.4.4 verified (16 November 2026):** `response_failure_includes_provider_error_code_and_message`
  confirms the failed-response error includes both provider code and message.
  All 26 request/response mapper tests pass. Python's streaming error type also
  retains both and prefixes the message with the code when available.
- **5.4.5 verified (16 November 2026):** a failed response with a message and
  no error code returns just the provider message, without a fabricated code
  label. All 27 request/response mapper tests pass.
- **5.4.6 verified (16 November 2026):** a failed response with an empty
  error code returns the provider message without a blank code prefix. The
  focused test passes, and all 28 request/response mapper tests pass. This
  follows the Python stream-error formatter's rule to prefix only a nonempty
  code; applying it to response-body mapping remains a deliberate subset.
- **5.4.7 verified (16 November 2026):** an `in_progress` response with empty
  output returns an error rather than being mapped to an empty final answer.
  The learner added an explicit status guard because this Rust subset has no
  continuation-token or polling behavior. All 29 request/response mapper
  tests pass.
- **5.4.8 verified (16 November 2026):** a `queued` response with empty output
  returns an error instead of an empty successful answer. Python uses a
  continuation token for this state; Rust does not support resuming it yet.
  All 30 request/response mapper tests pass.
- **5.4.9 verified (16 November 2026):** an `incomplete` response with reason
  `max_output_tokens` and empty output returns an error rather than an empty
  answer. Rust cannot store the corresponding finish reason in `ChatResponse`
  yet; Python can. All 31 request/response mapper tests pass.
- **5.4.10 verified (16 November 2026):** a `cancelled` response returns an
  error rather than an empty success. The learner corrected the status-string
  spelling, and all 32 request/response mapper tests pass. Python recognizes
  `cancelled` as a terminal response status; Rust's explicit rejection is a
  deliberate interim subset.
- **5.4.11 verified (16 November 2026):**
  `response_maps_completed_status_as_successful_response` confirms a
  `completed` response still maps successfully and preserves its response ID
  and assistant text. All 33 request/response mapper tests pass.
- **Section 5.4 checkpoint:** the offline mapper rejects `failed`,
  `in_progress`, `queued`, `incomplete`, and `cancelled` instead of returning
  an empty or misleading successful answer. Failed-response errors include a
  nonempty provider code and message when available, with a generic fallback
  if the message is missing. A missing status remains accepted for existing
  fixtures, and a `completed` response follows normal output mapping. These
  rejections are a deliberate Rust subset: continuation tokens, finish
  reasons, and cancelled outcomes are not modeled. HTTP, authentication,
  transport, and service errors are not yet handled because no transport
  exists. The full `cargo test --quiet` suite passes all 129 tests.
- **Next exercise: 5.5.1 - build an offline reqwest request.** Test that when
  both an API key and an already-resolved Entra token are available, the
  request uses bearer authentication and omits the API-key header. Build and
  inspect the request only; do not send it or use a real credential. The async
  token-provider hook will be a separate follow-up.
- **Live endpoint plan:** Chapter 5, near section 5.5, may include a separate
  opt-in live Azure OpenAI smoke test after request/response mapping, transport,
  authentication, and offline tests exist. Section 5.5 still keeps routine
  tests deterministic and credential-free. No live call has been made.
- The local `python/packages/openai/README.md` recommends the Responses API
  client OpenAIChatClient for new work; that same class supports Azure routing.
  Use this Azure-configured Responses path for the first read-only walkthrough,
  not assumed older Azure-specific class names or Chat Completions schemas.
  No Rust SDK, authentication method, deployment, or API version is selected.
- **5.1.1 reference inspection:** in
  `python/packages/openai/agent_framework_openai/_chat_client.py`,
  `_prepare_options` assembles input and model, `_prepare_message_for_openai`
  maps the framework message, and `_prepare_content_for_openai` maps ordinary
  user text to input_text (assistant history uses output_text). The
  non-streaming `_inner_get_response` path awaits
  client.responses.with_raw_response.create(stream=False, **run_options) for
  a simple request. `_parse_response_from_openai` turns output_text into
  framework Text, creates assistant messages, and copies response.id into
  response_id. `_handle_request_error` raises a service exception rather than
  returning an empty success.
- Azure configuration is resolved by load_openai_service_settings in the
  sibling `_shared.py`; the Responses constructor passes responses_mode=True.
  Model means the Azure deployment name on this route. The source has richer
  options and behaviors than the simplified first text-only example.
- **5.1.2 reference inspection:** Agent stores configured instructions in
  default_options in `python/packages/core/agent_framework/_agents.py` and
  passes chat_options separately from session_messages to client.get_response.
  BaseChatClient forwards those options. The Responses `_prepare_options`
  retains non-None instructions as a top-level request option; it does not
  synthesize a system message for them.
- `test_instructions_passed_natively_not_as_system_message` in
  `python/packages/openai/tests/openai/test_openai_chat_client.py` explicitly
  checks this native field and the absence of an injected system message,
  including continuation cases. This was source inspection, not a Python
  test run or live call.
- The Rust Agent already passes instructions separately to ChatClient, so no
  agent change is needed for this boundary. In the future request mapper,
  None means omit instructions; a supplied empty string is not None and the
  reference retains it. Do not copy the type-tag empty-string rule here.
  User text maps to input_text; replayed assistant text maps to output_text.
- **5.1.3 planned initial contract (not implemented):** Azure Responses,
  non-streaming, explicit endpoint/deployment, ordered user/assistant messages
  with ordinary unannotated Text, and top-level optional instructions.
  Preserve exact text, supported message/content order, and supplied response
  and output-message IDs. Require nonempty prepared request input, as in the
  reference; this is different from allowing empty response messages.
- User text maps to input_text. Replayed assistant text maps to output_text
  with annotations: [] when unannotated, verified in
  `_annotations_to_output_text` and `_prepare_content_for_openai`.
  Completed ordinary text outputs become framework assistant Text messages.
  A valid response with no output items becomes one empty assistant message
  in the reference; this must not be used as recovery for a failed request.
- The first Rust provider subset will reject unsupported content/roles and
  nonempty annotations explicitly rather than silently dropping Chapter 4
  data. Tools, reasoning replay, citations/media, streaming, provider-managed
  continuation, and broader metadata are deferred. The full Python adapter
  supports much more. SDK choice, authentication, API-version handling, and
  storage policy remain undecided before live use.
- **5.1 checkpoint (13 November 2026):** `cargo test --quiet` still passes
  **93 tests**: 74 library, 9 public API, 8 async-practice, and 2 session
  integration tests. Existing compiler warnings remain. The local manuscript
  and outline include structured 5.1 coaching; the rebuilt PDF validates at
  **126 pages, 94 bookmarks, and 195 links**. The only tracked change for
  this section is this journey; no coaching instructions changed. Provide
  scoped review/add/commit/push commands and leave Git writes to the learner.
- Section **4.4 is complete for the initial reasoning-text subset**, not the
  full Microsoft Agent Framework (MAF) reasoning-content contract.
- Section **4.5 is complete for the deliberate Rust subset**: Citation
  annotations have optional title and URL, are stored on Text content, and
  survive Message and AgentSession JSON round trips. This is not full MAF
  annotation parity; attachment to other content kinds and additional
  annotation fields remain deferred.
- **4.7.1 verified (13 November 2026):** `cargo test --lib
  response_json_round_trip_preserves_structured_messages --quiet` passes
  (1 passed, 70 filtered out). A whole response round trip preserves its ID,
  message count, ordered reasoning and cited-text contents, and answer-only
  text view. This verifies the existing Rust subset, not full Python parity.
- **4.7.2 verified (13 November 2026):** the learner added
  `#[serde(default)]` to ChatResponseInput.messages. The test
  `response_json_defaults_missing_messages` passes; `cargo test --lib
  response --quiet` passes all 12 selected library tests.
  The local Python ChatResponse constructor defaults messages to None and
  normalizes it to an empty list; SerializationMixin.from_dict forwards the
  supplied keyword arguments to that constructor. An omitted messages field
  now produces an empty Rust response too. This covers omission, not null.
- **4.7.3 verified (13 November 2026):** `cargo test --lib response --quiet`
  passes all 13 selected library tests, including
  `response_json_accepts_null_messages`. ChatResponseInput.messages is now
  Option<Vec<Message>>, while ChatResponse.messages remains Vec<Message>.
  TryFrom uses input.messages.unwrap_or_default(): Some yields its owned
  vector, and None yields Vec's default empty vector. This is not a panic or
  a catch for deserialization errors.
- During 4.7.3, the learner initially changed the actual response field
  instead of the helper, causing E0624/E0608 in older tests. They restored the
  actual field, constructor, and getter rather than weakening those tests.
  The null-messages test now verifies empty stored messages, the preserved
  response ID, empty text, and serialization as an ordinary empty array.
- **4.7.4 verified (13 November 2026):** `cargo test --lib response --quiet`
  passes all 14 selected library tests, including
  `response_json_rejects_invalid_messages`. It rejects a string, number,
  Boolean, object, and an array containing a number. Missing/null normalization
  does not hide these deserialization errors. This stricter typed-input rule
  is a deliberate Rust subset, not a claim about Python rejecting every shape.
- **4.7.5 compatibility review:** direct message text joins Text items with
  spaces; response text joins message texts with newlines and trims outer
  whitespace, as in the local reference. Other metadata and content fields
  remain deferred; unknown properties are not generally retained.
  Rust Message does not emit a message type tag and emits absent author_name
  and message_id as null, unlike Python's default tagged, None-omitting
  serialization. A Rust round trip is not complete Python/provider wire
  compatibility. Future provider adapters must explicitly map supported data.
- **4.7.6 checkpoint (13 November 2026):** `cargo test --quiet` passes
  **93 tests**: 74 library, 9 public API, 8 async-practice, and 2 session
  integration tests. Existing compiler warnings remain. The local manuscript
  and outline include structured 4.7 coaching; the rebuilt PDF validates at
  **121 pages, 92 bookmarks, and 191 links**. The scoped Git changes are
  src/types.rs, src/types_tests.rs, and this journey; no coaching instructions
  changed during this section. Git writes remain the learner's responsibility.
- **Explanation requested:** the earlier null/empty-type test sets `type` to
  null while messages is an ordinary empty array. The new test sets messages
  to null while type is the correct string. Null in one JSON field does not
  establish null handling in a different field.
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
- **4.6 limits at that checkpoint:** this completed only the initial
  response-ID/JSON subset.
  Other response metadata remains deferred. Sessions store messages, not
  response IDs. Its response JSON tests used empty message lists, and Rust
  still required a messages array. Section 4.7 subsequently verified a
  nonempty structured response and added missing/null list normalization;
  full nested-response JSON compatibility is still not established.
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
  companion-example tests passed. The current 5.1 PDF is 126 pages and remains
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
  type tag. Its supplied ID round-trips with an empty list and with a message
  containing reasoning and cited text; an absent ID is omitted. A wrong
  nonempty string type is rejected; missing, null, empty, and correct string
  tags are accepted through a private input helper and TryFrom conversion.
  Omitted and explicit-null messages normalize to an empty vector through
  optional helper input and unwrap_or_default. Invalid message types must
  still fail deserialization. Full response/message JSON parity is not
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
- **4.7:** complete for the initial structured-response and normalized-input
  subset; remaining compatibility boundaries are documented above.
- **5:** in progress; Azure OpenAI selected, with the local reference's
  Responses client traced in completed section 5.1 and explicit offline
  configuration completed in section 5.2. Continue with learner-authored
  request mapping, then transport and deterministic boundary tests. No Rust
  provider implementation or live Azure call exists yet.
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
