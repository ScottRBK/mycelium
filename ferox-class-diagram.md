# Mermaid class diagrams

Declared types and signatures; unknown types are `unknown`. Receivers are omitted.
Fields are associations, not lifetime ownership. Calls are static heuristic estimates.
Views contain at most 40 members per box. Connections within each view are uncapped.
Repeated boxes continue their members; parallel arrows are summarized.
Cross-diagram relationships are retained in the complete relationship list.

Included: 48 boxes. Calls without in-scope member endpoints: 0.

## Test filtering

Mode: exclude.
Saved detector version: 1.
Test paths: `tests`.
Keep paths: none.
Calls removed by test filtering: 165. Type relationships removed: 17.

- `examples/cli.rs`: 5 source occurrences (rust.cfg-test).
- `examples/cli_s.rs`: 5 source occurrences (rust.cfg-test).
- `src/adapters/providers/openai_compatible/client.rs`: 10 source occurrences (rust.cfg-test).
- `tests/e2e/openai_compatible_e2e.rs`: 17 source occurrences (test-path).
- `tests/integration/openai_compatible_error_integration.rs`: 13 source occurrences (test-path).
- `tests/integration/openai_compatible_gateway_integration.rs`: 5 source occurrences (test-path).
- `tests/integration/openai_compatible_reasoning_effort_integration.rs`: 6 source occurrences
  (test-path).
- `tests/integration/openai_compatible_tools_integration.rs`: 3 source occurrences (test-path).

## Diagram 1

```mermaid
classDiagram
    direction TB
    class c0000["AddTwoNumbersArguments"] {
        <<struct>>
        -first_number: i32
        -second_number: i32
    }
    class c0001["examples/cli.rs"] {
        <<module>>
        -main() Type1
        -required_env(name: Type3) Type2
        -repl(gateway: Gateway~P~) Type1
        -build_tools() Vec~Tool~
        -get_user_input() Type2
        -render_completion(render_text: Type3, is_reasoning: bool) Type1
        -get_agent_input(Signature1)
        -chat_session(Signature2)
        -handle_tool_calls(tool_calls: Type8) Type7
        -execute_tool_call(tool_call: Type10) Type9
        -parse_tool_arguments(arguments: Type3) Type11
        -add_two_numbers(first_number: i32, second_number: i32) Type12
        -get_current_unix_epoch_datetime() Type13
        -print_models(models: Type15) Type14
        -select_models(models: Type15) Type16
        -select_reasoning_effort() Type17
    }
    class c0002["AddTwoNumbersArguments"] {
        <<struct>>
        -first_number: i32
        -second_number: i32
    }
    class c0003["examples/cli_s.rs"] {
        <<module>>
        -main() Type1
        -required_env(name: Type3) Type2
        -repl(gateway: Gateway~P~) Type1
        -build_tools() Vec~Tool~
        -get_user_input() Type2
        -render_stream_chunk(Signature3)
        -get_agent_input(Signature1)
        -chat_session(Signature2)
        -handle_tool_calls(tool_calls: Type8) Type7
        -execute_tool_call(tool_call: Type10) Type9
        -parse_tool_arguments(arguments: Type3) Type11
        -add_two_numbers(first_number: i32, second_number: i32) Type12
        -get_current_unix_epoch_datetime() Type13
        -print_models(models: Type15) Type14
        -select_models(models: Type15) Type16
        -select_reasoning_effort() Type17
    }
    class c0004["examples/simple_completion.rs"] {
        <<module>>
        -main() Type1
    }
    class c0005["OpenAiCompatibleClient"] {
        <<struct>>
        -http: Client
        -base_url: String
        -api_key: Option~String~
        +builder() OpenAiCompatibleClientBuilder
        -fetch_models() Type21
        -fetch_models_body() Type22
        -parse_models(body: Type3) Type21
        -create_chat_completion(chat_request: Type24) Type23
        -parse_chat_completions_response(body: Type3) Type23
        -generate_chat_response(chat_request: Type24) Type25
        -handle_sse_line(line: Type3) Type26
        -stream_chat_response(response: reqwest::Response) Type27
        -parse_chat_completions_stream_response(chunk: Type3) Type28
        +complete(request: Type30) Type29
        +stream(request: Type30) Type31
        +list_models() Type32
    }
    class c0006["OpenAiCompatibleClientBuilder"] {
        <<struct>>
        -base_url: Option~String~
        -api_key: Option~String~
        -timeout: Duration
        +default() Self
        +new() Self
        +base_url(url: impl Into~String~) Self
        +api_key(key: impl Into~String~) Self
        +timeout(timeout: Duration) Self
        +build() Type33
    }
    class c0008["ClientBuildError"] {
        <<enum>>
        -MissingBaseUrl: unknown
        -HttpClient: Type34
    }
    class c0009["OpenAiClientError"] {
        <<enum>>
        -Transport: Type35
        -Status: Type36
        -Decode: Type37
        -Utf8: Type38
    }
    class c0010["src/adapters/providers/openai_compatible/mapping.rs"] {
        <<module>>
        -to_domain_model_modality(modality: Type3) Type39
        -to_domain_model_modalities(modalities: Vec~String~) Type40
        +to_domain_model(provider_model: ProviderModel) Type41
        +to_domain_toolcall(tool_call: Type42) ToolCall
        +to_provider_message(message: Type43) ChatCompletionsMessageRequest
        +to_provider_reasoning_effort(reasoning_effort: ReasoningEffort) String
        -to_provider_toolcall(tool_call: Type10) ChatCompletionToolCall
        +to_provider_tools(tool: Tool) ChatCompletionTool
        -to_provider_parameters(parameter: ToolParameters) ChatCompletionToolParameters
        -to_provider_parameter_property(property: ToolParameterProperty) Type44
        +to_provider_property_type(Signature4)
        +to_domain_finish_reason(reason: Type45) Option~FinishReason~
    }
    class c0011["ChatCompletionChoices"] {
        <<struct>>
        +message: ChatCompletionsMessageResponse
    }
    class c0012["ChatCompletionFunction"] {
        <<struct>>
        +name: String
        +description: String
        +parameters: Option~ChatCompletionToolParameters~
    }
    class c0013["ChatCompletionTool"] {
        <<struct>>
        +tool_type: String
        +function: ChatCompletionFunction
    }
    class c0014["ChatCompletionToolCall"] {
        <<struct>>
        +id: String
        +tool_type: String
        +function: ChatCompletionToolCallFunction
    }
    class c0015["ChatCompletionToolCallFunction"] {
        <<struct>>
        +name: String
        +arguments: String
    }
    class c0016["ChatCompletionToolParameterProperty"] {
        <<struct>>
        +property_type: ChatCompletionToolParameterPropertyType
        +description: String
        +property_enum: Option~Vec~String~~
    }
    class c0017["ChatCompletionToolParameterPropertyType"] {
        <<enum>>
        -String: unknown
        -Number: unknown
        -Integer: unknown
        -Boolean: unknown
    }
    class c0018["ChatCompletionToolParameters"] {
        <<struct>>
        +parameter_type: String
        +properties: Type46
        +required: Vec~String~
    }
    class c0019["ChatCompletionsMessageRequest"] {
        <<enum>>
        -System: Type47
        -User: Type47
        -Assistant: Type48
        -Tool: Type49
    }
    class c0020["ChatCompletionsMessageResponse"] {
        <<enum>>
        -System: Type47
        -User: Type47
        -Assistant: Type50
        +content() Type51
        +reasoning_content() Type51
        +tool_calls() Type52
    }
    class c0021["ChatCompletionsRequest"] {
        <<struct>>
        +model: String
        +messages: Vec~ChatCompletionsMessageRequest~
        +stream: bool
        +tools: Option~Vec~ChatCompletionTool~~
        +reasoning_effort: Option~String~
    }
    class c0022["ChatCompletionsResponse"] {
        <<struct>>
        +choices: Vec~ChatCompletionChoices~
        +model: String
    }
    class c0023["ChatCompletionsStreamChoice"] {
        <<struct>>
        +delta: ChatCompletionsStreamDelta
        +finish_reason: Option~ChoicesFinishReason~
    }
    class c0024["ChatCompletionsStreamDelta"] {
        <<struct>>
        +content: Option~String~
        +reasoning_content: Option~String~
        +tool_calls: Vec~ChatCompletionsToolCallDelta~
    }
    class c0025["ChatCompletionsStreamResponse"] {
        <<struct>>
        +choices: Vec~ChatCompletionsStreamChoice~
    }
    class c0026["ChatCompletionsToolCallDelta"] {
        <<struct>>
        +index: usize
        +id: Option~String~
        +function: ChatCompletionsToolCallFunctionDelta
    }
    class c0027["ChatCompletionsToolCallFunctionDelta"] {
        <<struct>>
        +name: Option~String~
        +arguments: Option~String~
    }
    class c0028["ChoicesFinishReason"] {
        <<enum>>
        -Stop: unknown
        -Length: unknown
        -ToolCalls: unknown
        -ContentFilter: unknown
    }
    class c0029["ModelsResponse"] {
        <<struct>>
        +data: Vec~ProviderModel~
    }
    class c0030["PendingToolCall"] {
        <<struct>>
        +id: Option~String~
        +name: Option~String~
        +arguments: String
        +apply(delta: Type53) Type14
        +finish() Type54
    }
    class c0031["ProviderModel"] {
        <<struct>>
        +id: String
        +input_modalities: Vec~String~
        +output_modalities: Vec~String~
    }
    class c0032["GatewayError"] {
        <<enum>>
        -UnsupportedModel: Type55
        -ProviderNotConfigured: Type55
        -PolicyDenied: unknown
        -Llm: Type56
    }
    class c0033["LlmError"] {
        <<enum>>
        +from(error: OpenAiClientError) Self
        -InvalidRequest: Type57
        -AuthenticationFailed: unknown
        -PermissionDenied: unknown
        -RateLimited: Type58
        -Timeout: unknown
        -ProviderUnavailable: Type59
        -ProviderFailure: Type60
        -Transport: Type61
        -InvalidResponse: Type62
        -InvalidModelModality: Type63
    }
    class c0034["Gateway"] {
        <<struct>>
        -provider: P
        +new(provider: P) Self
        +complete(request: Type30) Type64
        +stream(request: Type30) Type65
        +list_models() Type66
    }
    class c0035["CompletionChunk"] {
        <<struct>>
        +text: Option~String~
        +reasoning: Option~String~
        +tool_calls: Vec~ToolCall~
        +finished_reason: Option~FinishReason~
        +new(tool_calls: Vec~ToolCall~, finished_reason: Option~FinishReason~) Self
    }
    class c0036["CompletionRequest"] {
        <<struct>>
        +model: String
        +messages: Type67
        +tools: Option~Vec~Tool~~
        +reasoning_effort: Option~ReasoningEffort~
        +new(model: String, messages: Type67) Self
    }
    class c0037["CompletionResponse"] {
        <<struct>>
        +model: String
        +text: Option~String~
        +reasoning: Option~String~
        +tool_calls: Vec~ToolCall~
        +new(model: String, tool_calls: Vec~ToolCall~) Self
    }
    class c0038["FinishReason"] {
        <<enum>>
        -Stop: unknown
        -Length: unknown
        -ToolCalls: unknown
        -ContentFilter: unknown
    }
    class c0039["Message"] {
        <<enum>>
        -System: Type68
        -User: Type69
        -Assistant: Type70
        -Tool: Type71
    }
    class c0040["Model"] {
        <<struct>>
        +id: String
        +input_modalities: Vec~ModelModality~
        +output_modalities: Vec~ModelModality~
        +new(Signature5)
    }
    class c0041["ModelModality"] {
        <<enum>>
        -Text: unknown
        -Image: unknown
        -Video: unknown
        -Audio: unknown
    }
    class c0042["ReasoningEffort"] {
        <<enum>>
        -None: unknown
        -Minimal: unknown
        -Low: unknown
        -Medium: unknown
        -High: unknown
        -XHigh: unknown
        -Max: unknown
    }
    class c0043["Tool"] {
        <<struct>>
        +name: String
        +description: String
        +parameters: Option~ToolParameters~
        +new(name: impl Into~String~, description: impl Into~String~) Self
        +required_parameter(parameter_property: ToolParameterProperty) Self
        +optional_parameter(parameter_property: ToolParameterProperty) Self
        -parameter(parameter_property: ToolParameterProperty, required: bool) Self
    }
    class c0044["ToolCall"] {
        <<struct>>
        +id: String
        +name: String
        +arguments: String
        +new(id: String, name: String, arguments: String) Self
    }
    class c0045["ToolParameterProperty"] {
        <<struct>>
        +name: String
        +property_type: ToolParameterPropertyType
        +description: String
        +property_enum: Option~Vec~String~~
        +new(Signature6)
    }
    class c0046["ToolParameterPropertyType"] {
        <<enum>>
        -String: unknown
        -Number: unknown
        -Integer: unknown
        -Boolean: unknown
    }
    class c0047["ToolParameters"] {
        <<struct>>
        +properties: Vec~ToolParameterProperty~
        +required: Vec~String~
        +new(properties: Vec~ToolParameterProperty~, required: Vec~String~) Self
    }
    class c0048["LlmProvider"] {
        <<trait>>
        +complete(request: CompletionRequest) Type72
        +stream(request: CompletionRequest) Type73
        +list_models() Type74
    }
    c0001 ..> c0001 : 15 relationships (see list)
    c0001 ..> c0005 : 3 relationships (see list)
    c0001 ..> c0006 : 6 relationships (see list)
    c0001 ..> c0034 : 3 relationships (see list)
    c0001 ..> c0039 : 3 relationships (see list)
    c0001 ..> c0040 : 4 relationships (see list)
    c0001 ..> c0042 : 3 relationships (see list)
    c0001 ..> c0043 : type in build_tools
    c0001 ..> c0044 : 2 relationships (see list)
    c0003 ..> c0003 : 15 relationships (see list)
    c0003 ..> c0005 : 3 relationships (see list)
    c0003 ..> c0006 : 6 relationships (see list)
    c0003 ..> c0034 : 3 relationships (see list)
    c0003 ..> c0035 : type in render_stream_chunk
    c0003 ..> c0039 : 3 relationships (see list)
    c0003 ..> c0040 : 4 relationships (see list)
    c0003 ..> c0042 : 3 relationships (see list)
    c0003 ..> c0043 : type in build_tools
    c0003 ..> c0044 : 2 relationships (see list)
    c0004 ..> c0005 : 2 relationships (see list)
    c0004 ..> c0006 : 3 relationships (see list)
    c0005 ..> c0005 : 8 relationships (see list)
    c0005 ..> c0006 : 3 relationships (see list)
    c0005 ..> c0009 : 9 relationships (see list)
    c0005 ..> c0020 : 3 relationships (see list)
    c0005 ..> c0021 : 2 relationships (see list)
    c0005 ..> c0022 : 2 relationships (see list)
    c0005 ..> c0025 : 3 relationships (see list)
    c0005 ..> c0030 : 2 relationships (see list)
    c0005 ..> c0031 : 2 relationships (see list)
    c0005 ..> c0033 : 3 relationships (see list)
    c0005 ..> c0036 : 2 relationships (see list)
    c0005 ..> c0037 : type in complete
    c0005 ..> c0040 : type in list_models
    c0005 ..|> c0048 : implements
    c0006 ..> c0005 : 2 relationships (see list)
    c0006 ..> c0006 : default() calls new()
    c0006 ..> c0008 : type in build
    c0010 ..> c0010 : 3 relationships (see list)
    c0010 ..> c0013 : type in to_provider_tools
    c0010 ..> c0014 : 2 relationships (see list)
    c0010 ..> c0016 : type in to_provider_parameter_property
    c0010 ..> c0017 : type in to_provider_property_type
    c0010 ..> c0018 : type in to_provider_parameters
    c0010 ..> c0019 : type in to_provider_message
    c0010 ..> c0028 : type in to_domain_finish_reason
    c0010 ..> c0031 : type in to_domain_model
    c0010 ..> c0033 : 3 relationships (see list)
    c0010 ..> c0038 : type in to_domain_finish_reason
    c0010 ..> c0039 : type in to_provider_message
    c0010 ..> c0040 : type in to_domain_model
    c0010 ..> c0041 : 2 relationships (see list)
    c0010 ..> c0042 : type in to_provider_reasoning_effort
    c0010 ..> c0043 : type in to_provider_tools
    c0010 ..> c0044 : 2 relationships (see list)
    c0010 ..> c0045 : type in to_provider_parameter_property
    c0010 ..> c0046 : type in to_provider_property_type
    c0010 ..> c0047 : type in to_provider_parameters
    c0011 --> c0020 : field message
    c0012 --> c0018 : field parameters
    c0013 --> c0012 : field function
    c0014 --> c0015 : field function
    c0016 --> c0017 : field property_type
    c0018 --> c0016 : field properties
    c0019 --> c0014 : field Assistant
    c0020 --> c0014 : field Assistant
    c0020 ..> c0014 : type in tool_calls
    c0021 --> c0013 : field tools
    c0021 --> c0019 : field messages
    c0022 --> c0011 : field choices
    c0023 --> c0024 : field delta
    c0023 --> c0028 : field finish_reason
    c0024 --> c0026 : field tool_calls
    c0025 --> c0023 : field choices
    c0026 --> c0027 : field function
    c0029 --> c0031 : field data
    c0030 ..> c0026 : type in apply
    c0030 ..> c0033 : type in finish
    c0030 ..> c0044 : type in finish
    c0032 --> c0033 : field Llm
    c0033 ..> c0009 : type in from
    c0034 ..> c0032 : 3 relationships (see list)
    c0034 ..> c0035 : type in stream
    c0034 ..> c0036 : 2 relationships (see list)
    c0034 ..> c0037 : type in complete
    c0034 ..> c0040 : type in list_models
    c0035 --> c0038 : field finished_reason
    c0035 ..> c0038 : type in new
    c0035 --> c0044 : field tool_calls
    c0035 ..> c0044 : type in new
    c0036 --> c0039 : field messages
    c0036 ..> c0039 : type in new
    c0036 --> c0042 : field reasoning_effort
    c0036 --> c0043 : field tools
    c0037 --> c0044 : field tool_calls
    c0037 ..> c0044 : type in new
    c0039 --> c0043 : field Tool
    c0039 --> c0044 : field Assistant
    c0040 --> c0041 : 2 relationships (see list)
    c0040 ..> c0041 : type in new
    c0043 ..> c0043 : 2 relationships (see list)
    c0043 ..> c0045 : 3 relationships (see list)
    c0043 --> c0047 : field parameters
    c0045 --> c0046 : field property_type
    c0045 ..> c0046 : type in new
    c0047 --> c0045 : field properties
    c0047 ..> c0045 : type in new
    c0048 ..> c0033 : 3 relationships (see list)
    c0048 ..> c0036 : 2 relationships (see list)
    c0048 ..> c0037 : type in complete
    c0048 ..> c0040 : type in list_models
```

## Source index

- c0000: `AddTwoNumbersArguments` — `examples/cli.rs`:239
- c0001: `examples/cli.rs` — `examples/cli.rs`:1
- c0002: `AddTwoNumbersArguments` — `examples/cli_s.rs`:270
- c0003: `examples/cli_s.rs` — `examples/cli_s.rs`:1
- c0004: `examples/simple_completion.rs` — `examples/simple_completion.rs`:1
- c0005: `OpenAiCompatibleClient` — `src/adapters/providers/openai_compatible/client.rs`:98
- c0006: `OpenAiCompatibleClientBuilder` — `src/adapters/providers/openai_compatible/client.rs`:26
- c0008: `ClientBuildError` — `src/adapters/providers/openai_compatible/errors.rs`:53
- c0009: `OpenAiClientError` — `src/adapters/providers/openai_compatible/errors.rs`:5
- c0010: `src/adapters/providers/openai_compatible/mapping.rs` —
  `src/adapters/providers/openai_compatible/mapping.rs`:1
- c0011: `ChatCompletionChoices` — `src/adapters/providers/openai_compatible/models.rs`:160
- c0012: `ChatCompletionFunction` — `src/adapters/providers/openai_compatible/models.rs`:133
- c0013: `ChatCompletionTool` — `src/adapters/providers/openai_compatible/models.rs`:126
- c0014: `ChatCompletionToolCall` — `src/adapters/providers/openai_compatible/models.rs`:25
- c0015: `ChatCompletionToolCallFunction` — `src/adapters/providers/openai_compatible/models.rs`:20
- c0016: `ChatCompletionToolParameterProperty` —
  `src/adapters/providers/openai_compatible/models.rs`:109
- c0017: `ChatCompletionToolParameterPropertyType` —
  `src/adapters/providers/openai_compatible/models.rs`:100
- c0018: `ChatCompletionToolParameters` — `src/adapters/providers/openai_compatible/models.rs`:118
- c0019: `ChatCompletionsMessageRequest` — `src/adapters/providers/openai_compatible/models.rs`:35
- c0020: `ChatCompletionsMessageResponse` — `src/adapters/providers/openai_compatible/models.rs`:57
- c0021: `ChatCompletionsRequest` — `src/adapters/providers/openai_compatible/models.rs`:141
- c0022: `ChatCompletionsResponse` — `src/adapters/providers/openai_compatible/models.rs`:165
- c0023: `ChatCompletionsStreamChoice` — `src/adapters/providers/openai_compatible/models.rs`:232
- c0024: `ChatCompletionsStreamDelta` — `src/adapters/providers/openai_compatible/models.rs`:184
- c0025: `ChatCompletionsStreamResponse` — `src/adapters/providers/openai_compatible/models.rs`:238
- c0026: `ChatCompletionsToolCallDelta` — `src/adapters/providers/openai_compatible/models.rs`:171
- c0027: `ChatCompletionsToolCallFunctionDelta` —
  `src/adapters/providers/openai_compatible/models.rs`:178
- c0028: `ChoicesFinishReason` — `src/adapters/providers/openai_compatible/models.rs`:152
- c0029: `ModelsResponse` — `src/adapters/providers/openai_compatible/models.rs`:15
- c0030: `PendingToolCall` — `src/adapters/providers/openai_compatible/models.rs`:193
- c0031: `ProviderModel` — `src/adapters/providers/openai_compatible/models.rs`:6
- c0032: `GatewayError` — `src/error.rs`:71
- c0033: `LlmError` — `src/error.rs`:9
- c0034: `Gateway` — `src/gateway.rs`:10
- c0035: `CompletionChunk` — `src/models.rs`:330
- c0036: `CompletionRequest` — `src/models.rs`:265
- c0037: `CompletionResponse` — `src/models.rs`:290
- c0038: `FinishReason` — `src/models.rs`:315
- c0039: `Message` — `src/models.rs`:379
- c0040: `Model` — `src/models.rs`:6
- c0041: `ModelModality` — `src/models.rs`:75
- c0042: `ReasoningEffort` — `src/models.rs`:220
- c0043: `Tool` — `src/models.rs`:172
- c0044: `ToolCall` — `src/models.rs`:357
- c0045: `ToolParameterProperty` — `src/models.rs`:104
- c0046: `ToolParameterPropertyType` — `src/models.rs`:89
- c0047: `ToolParameters` — `src/models.rs`:134
- c0048: `LlmProvider` — `src/ports/llm.rs`:10

## Relationships

- c0001 ..> c0001: `chat_session() calls get_agent_input()`
- c0001 ..> c0001: `chat_session() calls get_user_input()`
- c0001 ..> c0001: `execute_tool_call() calls add_two_numbers()`
- c0001 ..> c0001: `execute_tool_call() calls get_current_unix_epoch_datetime()`
- c0001 ..> c0001: `execute_tool_call() calls parse_tool_arguments()`
- c0001 ..> c0001: `get_agent_input() calls build_tools()`
- c0001 ..> c0001: `get_agent_input() calls handle_tool_calls()`
- c0001 ..> c0001: `get_agent_input() calls render_completion()`
- c0001 ..> c0001: `handle_tool_calls() calls execute_tool_call()`
- c0001 ..> c0001: `main() calls repl()`
- c0001 ..> c0001: `main() calls required_env()`
- c0001 ..> c0001: `repl() calls chat_session()`
- c0001 ..> c0001: `repl() calls select_models()`
- c0001 ..> c0001: `repl() calls select_reasoning_effort()`
- c0001 ..> c0001: `select_models() calls print_models()`
- c0001 ..> c0005: `get_agent_input() calls complete()`
- c0001 ..> c0005: `main() calls builder()`
- c0001 ..> c0005: `repl() calls list_models()`
- c0001 ..> c0006: `chat_session() calls new()`
- c0001 ..> c0006: `get_agent_input() calls new()`
- c0001 ..> c0006: `main() calls api_key()`
- c0001 ..> c0006: `main() calls base_url()`
- c0001 ..> c0006: `main() calls build()`
- c0001 ..> c0006: `main() calls new()`
- c0001 ..> c0034: `type in chat_session`
- c0001 ..> c0034: `type in get_agent_input`
- c0001 ..> c0034: `type in repl`
- c0001 ..> c0039: `type in execute_tool_call`
- c0001 ..> c0039: `type in get_agent_input`
- c0001 ..> c0039: `type in handle_tool_calls`
- c0001 ..> c0040: `type in chat_session`
- c0001 ..> c0040: `type in get_agent_input`
- c0001 ..> c0040: `type in print_models`
- c0001 ..> c0040: `type in select_models`
- c0001 ..> c0042: `type in chat_session`
- c0001 ..> c0042: `type in get_agent_input`
- c0001 ..> c0042: `type in select_reasoning_effort`
- c0001 ..> c0043: `type in build_tools`
- c0001 ..> c0044: `type in execute_tool_call`
- c0001 ..> c0044: `type in handle_tool_calls`
- c0003 ..> c0003: `chat_session() calls get_agent_input()`
- c0003 ..> c0003: `chat_session() calls get_user_input()`
- c0003 ..> c0003: `execute_tool_call() calls add_two_numbers()`
- c0003 ..> c0003: `execute_tool_call() calls get_current_unix_epoch_datetime()`
- c0003 ..> c0003: `execute_tool_call() calls parse_tool_arguments()`
- c0003 ..> c0003: `get_agent_input() calls build_tools()`
- c0003 ..> c0003: `get_agent_input() calls handle_tool_calls()`
- c0003 ..> c0003: `get_agent_input() calls render_stream_chunk()`
- c0003 ..> c0003: `handle_tool_calls() calls execute_tool_call()`
- c0003 ..> c0003: `main() calls repl()`
- c0003 ..> c0003: `main() calls required_env()`
- c0003 ..> c0003: `repl() calls chat_session()`
- c0003 ..> c0003: `repl() calls select_models()`
- c0003 ..> c0003: `repl() calls select_reasoning_effort()`
- c0003 ..> c0003: `select_models() calls print_models()`
- c0003 ..> c0005: `get_agent_input() calls stream()`
- c0003 ..> c0005: `main() calls builder()`
- c0003 ..> c0005: `repl() calls list_models()`
- c0003 ..> c0006: `chat_session() calls new()`
- c0003 ..> c0006: `get_agent_input() calls new()`
- c0003 ..> c0006: `main() calls api_key()`
- c0003 ..> c0006: `main() calls base_url()`
- c0003 ..> c0006: `main() calls build()`
- c0003 ..> c0006: `main() calls new()`
- c0003 ..> c0034: `type in chat_session`
- c0003 ..> c0034: `type in get_agent_input`
- c0003 ..> c0034: `type in repl`
- c0003 ..> c0035: `type in render_stream_chunk`
- c0003 ..> c0039: `type in execute_tool_call`
- c0003 ..> c0039: `type in get_agent_input`
- c0003 ..> c0039: `type in handle_tool_calls`
- c0003 ..> c0040: `type in chat_session`
- c0003 ..> c0040: `type in get_agent_input`
- c0003 ..> c0040: `type in print_models`
- c0003 ..> c0040: `type in select_models`
- c0003 ..> c0042: `type in chat_session`
- c0003 ..> c0042: `type in get_agent_input`
- c0003 ..> c0042: `type in select_reasoning_effort`
- c0003 ..> c0043: `type in build_tools`
- c0003 ..> c0044: `type in execute_tool_call`
- c0003 ..> c0044: `type in handle_tool_calls`
- c0004 ..> c0005: `main() calls builder()`
- c0004 ..> c0005: `main() calls complete()`
- c0004 ..> c0006: `main() calls base_url()`
- c0004 ..> c0006: `main() calls build()`
- c0004 ..> c0006: `main() calls new()`
- c0005 ..> c0005: `complete() calls create_chat_completion()`
- c0005 ..> c0005: `create_chat_completion() calls parse_chat_completions_response()`
- c0005 ..> c0005: `fetch_models() calls fetch_models_body()`
- c0005 ..> c0005: `fetch_models() calls parse_models()`
- c0005 ..> c0005: `handle_sse_line() calls parse_chat_completions_stream_response()`
- c0005 ..> c0005: `list_models() calls fetch_models()`
- c0005 ..> c0005: `stream() calls generate_chat_response()`
- c0005 ..> c0005: `stream() calls stream_chat_response()`
- c0005 ..> c0006: `builder() calls new()`
- c0005 ..> c0006: `stream() calls new()`
- c0005 ..> c0006: `type in builder`
- c0005 ..> c0009: `type in create_chat_completion`
- c0005 ..> c0009: `type in fetch_models`
- c0005 ..> c0009: `type in fetch_models_body`
- c0005 ..> c0009: `type in generate_chat_response`
- c0005 ..> c0009: `type in handle_sse_line`
- c0005 ..> c0009: `type in parse_chat_completions_response`
- c0005 ..> c0009: `type in parse_chat_completions_stream_response`
- c0005 ..> c0009: `type in parse_models`
- c0005 ..> c0009: `type in stream_chat_response`
- c0005 ..> c0020: `complete() calls content()`
- c0005 ..> c0020: `complete() calls reasoning_content()`
- c0005 ..> c0020: `complete() calls tool_calls()`
- c0005 ..> c0021: `type in create_chat_completion`
- c0005 ..> c0021: `type in generate_chat_response`
- c0005 ..> c0022: `type in create_chat_completion`
- c0005 ..> c0022: `type in parse_chat_completions_response`
- c0005 ..> c0025: `type in handle_sse_line`
- c0005 ..> c0025: `type in parse_chat_completions_stream_response`
- c0005 ..> c0025: `type in stream_chat_response`
- c0005 ..> c0030: `stream() calls apply()`
- c0005 ..> c0030: `stream() calls finish()`
- c0005 ..> c0031: `type in fetch_models`
- c0005 ..> c0031: `type in parse_models`
- c0005 ..> c0033: `type in complete`
- c0005 ..> c0033: `type in list_models`
- c0005 ..> c0033: `type in stream`
- c0005 ..> c0036: `type in complete`
- c0005 ..> c0036: `type in stream`
- c0005 ..> c0037: `type in complete`
- c0005 ..> c0040: `type in list_models`
- c0005 ..|> c0048: `implements`
- c0006 ..> c0005: `build() calls builder()`
- c0006 ..> c0005: `type in build`
- c0006 ..> c0006: `default() calls new()`
- c0006 ..> c0008: `type in build`
- c0010 ..> c0010: `to_domain_model() calls to_domain_model_modalities()`
- c0010 ..> c0010: `to_domain_model_modalities() calls to_domain_model_modality()`
- c0010 ..> c0010: `to_provider_parameter_property() calls to_provider_property_type()`
- c0010 ..> c0013: `type in to_provider_tools`
- c0010 ..> c0014: `type in to_domain_toolcall`
- c0010 ..> c0014: `type in to_provider_toolcall`
- c0010 ..> c0016: `type in to_provider_parameter_property`
- c0010 ..> c0017: `type in to_provider_property_type`
- c0010 ..> c0018: `type in to_provider_parameters`
- c0010 ..> c0019: `type in to_provider_message`
- c0010 ..> c0028: `type in to_domain_finish_reason`
- c0010 ..> c0031: `type in to_domain_model`
- c0010 ..> c0033: `type in to_domain_model`
- c0010 ..> c0033: `type in to_domain_model_modalities`
- c0010 ..> c0033: `type in to_domain_model_modality`
- c0010 ..> c0038: `type in to_domain_finish_reason`
- c0010 ..> c0039: `type in to_provider_message`
- c0010 ..> c0040: `type in to_domain_model`
- c0010 ..> c0041: `type in to_domain_model_modalities`
- c0010 ..> c0041: `type in to_domain_model_modality`
- c0010 ..> c0042: `type in to_provider_reasoning_effort`
- c0010 ..> c0043: `type in to_provider_tools`
- c0010 ..> c0044: `type in to_domain_toolcall`
- c0010 ..> c0044: `type in to_provider_toolcall`
- c0010 ..> c0045: `type in to_provider_parameter_property`
- c0010 ..> c0046: `type in to_provider_property_type`
- c0010 ..> c0047: `type in to_provider_parameters`
- c0011 --> c0020: `field message`
- c0012 --> c0018: `field parameters`
- c0013 --> c0012: `field function`
- c0014 --> c0015: `field function`
- c0016 --> c0017: `field property_type`
- c0018 --> c0016: `field properties`
- c0019 --> c0014: `field Assistant`
- c0020 --> c0014: `field Assistant`
- c0020 ..> c0014: `type in tool_calls`
- c0021 --> c0013: `field tools`
- c0021 --> c0019: `field messages`
- c0022 --> c0011: `field choices`
- c0023 --> c0024: `field delta`
- c0023 --> c0028: `field finish_reason`
- c0024 --> c0026: `field tool_calls`
- c0025 --> c0023: `field choices`
- c0026 --> c0027: `field function`
- c0029 --> c0031: `field data`
- c0030 ..> c0026: `type in apply`
- c0030 ..> c0033: `type in finish`
- c0030 ..> c0044: `type in finish`
- c0032 --> c0033: `field Llm`
- c0033 ..> c0009: `type in from`
- c0034 ..> c0032: `type in complete`
- c0034 ..> c0032: `type in list_models`
- c0034 ..> c0032: `type in stream`
- c0034 ..> c0035: `type in stream`
- c0034 ..> c0036: `type in complete`
- c0034 ..> c0036: `type in stream`
- c0034 ..> c0037: `type in complete`
- c0034 ..> c0040: `type in list_models`
- c0035 --> c0038: `field finished_reason`
- c0035 ..> c0038: `type in new`
- c0035 --> c0044: `field tool_calls`
- c0035 ..> c0044: `type in new`
- c0036 --> c0039: `field messages`
- c0036 ..> c0039: `type in new`
- c0036 --> c0042: `field reasoning_effort`
- c0036 --> c0043: `field tools`
- c0037 --> c0044: `field tool_calls`
- c0037 ..> c0044: `type in new`
- c0039 --> c0043: `field Tool`
- c0039 --> c0044: `field Assistant`
- c0040 --> c0041: `field input_modalities`
- c0040 --> c0041: `field output_modalities`
- c0040 ..> c0041: `type in new`
- c0043 ..> c0043: `optional_parameter() calls parameter()`
- c0043 ..> c0043: `required_parameter() calls parameter()`
- c0043 ..> c0045: `type in optional_parameter`
- c0043 ..> c0045: `type in parameter`
- c0043 ..> c0045: `type in required_parameter`
- c0043 --> c0047: `field parameters`
- c0045 --> c0046: `field property_type`
- c0045 ..> c0046: `type in new`
- c0047 --> c0045: `field properties`
- c0047 ..> c0045: `type in new`
- c0048 ..> c0033: `type in complete`
- c0048 ..> c0033: `type in list_models`
- c0048 ..> c0033: `type in stream`
- c0048 ..> c0036: `type in complete`
- c0048 ..> c0036: `type in stream`
- c0048 ..> c0037: `type in complete`
- c0048 ..> c0040: `type in list_models`

## Type key

- Type1: `Result<(), Box<dyn Error + Send + Sync>>`
- Type2: `Result<String, Box<dyn Error + Send + Sync>>`
- Type3: `&str`
- Type4: `&Model`
- Type5: `&mut Vec<Message>`
- Type6: `&Gateway<P>`
- Type7: `Result<Vec<Message>, Box<dyn Error + Send + Sync>>`
- Type8: `&[ToolCall]`
- Type9: `Result<Message, Box<dyn Error + Send + Sync>>`
- Type10: `&ToolCall`
- Type11: `Result<T, serde_json::Error>`
- Type12: `Result<i32, Box<dyn Error + Send + Sync>>`
- Type13: `Result<u64, Box<dyn Error + Send + Sync>>`
- Type14: `()`
- Type15: `&[Model]`
- Type16: `Result<&Model, Box<dyn Error + Send + Sync>>`
- Type17: `Result<ReasoningEffort, Box<dyn Error + Send + Sync>>`
- Type18: `&CompletionChunk`
- Type19: `&mut bool`
- Type20: `&mut String`
- Type21: `Result<Vec<ProviderModel>, OpenAiClientError>`
- Type22: `Result<String, OpenAiClientError>`
- Type23: `Result<ChatCompletionsResponse, OpenAiClientError>`
- Type24: `&ChatCompletionsRequest`
- Type25: `Result<reqwest::Response, OpenAiClientError>`
- Type26: `Option<Result<ChatCompletionsStreamResponse, OpenAiClientError>>`
- Type27: `impl Stream<Item = Result<ChatCompletionsStreamResponse, OpenAiClientError>>`
- Type28: `Result<ChatCompletionsStreamResponse, OpenAiClientError>`
- Type29: `Result<CompletionResponse, LlmError>`
- Type30: `CompletionRequest<'_>`
- Type31: `Result<Self::CompletionStream, LlmError>`
- Type32: `Result<Vec<Model>, LlmError>`
- Type33: `Result<OpenAiCompatibleClient, ClientBuildError>`
- Type34: `(#[from] reqwest::Error)`
- Type35: `(reqwest::Error)`
- Type36: `{ code: u16, body: String }`
- Type37: `(serde_json::Error)`
- Type38: `(std::str::Utf8Error)`
- Type39: `Result<ModelModality, LlmError>`
- Type40: `Result<Vec<ModelModality>, LlmError>`
- Type41: `Result<Model, LlmError>`
- Type42: `&ChatCompletionToolCall`
- Type43: `&Message`
- Type44: `(String, ChatCompletionToolParameterProperty)`
- Type45: `&ChoicesFinishReason`
- Type46: `HashMap<String, ChatCompletionToolParameterProperty>`
- Type47: `{ content: String, }`
- Type48: `{ content: Option<String>, tool_calls: Vec<ChatCompletionToolCall>,
  #[serde(skip_serializing_if = "Option::is_none")] reasoning_content: Option<String>, }`
- Type49: `{ tool_call_id: String, content: String, }`
- Type50: `{ content: Option<String>, #[serde(default)] tool_calls: Vec<ChatCompletionToolCall>,
  reasoning_content: Option<String>, }`
- Type51: `Option<&str>`
- Type52: `&[ChatCompletionToolCall]`
- Type53: `&ChatCompletionsToolCallDelta`
- Type54: `Result<ToolCall, LlmError>`
- Type55: `(String)`
- Type56: `(#[from] LlmError)`
- Type57: `{ /// Details explaining why the request was rejected. message: String, }`
- Type58: `{ /// Suggested delay before retrying, when available. Ferox does not retry
  automatically. retry_after: Option<Duration>, }`
- Type59: `{ /// HTTP status code returned by the provider. code: u16, /// Error details returned by
  the provider. message: String, }`
- Type60: `{ /// Details of the provider failure. message: String, }`
- Type61: `{ /// Details of the connection or data transfer failure. message: String, }`
- Type62: `{ /// Details of the unexpected response data. message: String, }`
- Type63: `{ /// Unrecognised format name from the provider's model metadata. modality: String, }`
- Type64: `Result<CompletionResponse, GatewayError>`
- Type65: `Result< impl Stream<Item = Result<CompletionChunk, GatewayError>> + Send + use<P>,
  GatewayError, >`
- Type66: `Result<Vec<Model>, GatewayError>`
- Type67: `&'a [Message]`
- Type68: `{ /// Instructions for the model. content: String, }`
- Type69: `{ /// Text supplied by the user. content: String, }`
- Type70: `{ /// Text from the model, if present. content: Option<String>, /// Function calls
  requested in this response. tool_calls: Vec<ToolCall>, /// Reasoning text to include in the
  conversation history, if present. reasoning: Option<String>, }`
- Type71: `{ /// Identifier of the tool call this result answers. tool_call_id: String, /// Tool
  output to return to the model. content: String, }`
- Type72: `impl Future<Output = Result<CompletionResponse, LlmError>> + Send`
- Type73: `impl Future<Output = Result<Self::CompletionStream, LlmError>> + Send`
- Type74: `impl Future<Output = Result<Vec<Model>, LlmError>> + Send`

## Signature key

- Signature1: `-get_agent_input(model: &Model, messages: &mut Vec<Message>, reasoning_effort:
  ReasoningEffort, gateway: &Gateway<P>) Result<(), Box<dyn Error + Send + Sync>>`
- Signature2: `-chat_session(model: &Model, reasoning_effort: ReasoningEffort, gateway: Gateway<P>)
  Result<(), Box<dyn Error + Send + Sync>>`
- Signature3: `-render_stream_chunk(completion: &CompletionChunk, seen_reasoning: &mut bool,
  seen_agent_response: &mut bool, agent_reasoning: &mut String, agent_response: &mut String)
  Result<(), Box<dyn Error + Send + Sync>>`
- Signature4: `+to_provider_property_type(property_type: ToolParameterPropertyType)
  ChatCompletionToolParameterPropertyType`
- Signature5: `+new(id: impl Into<String>, input_modalities: Option<Vec<ModelModality>>,
  output_modalities: Option<Vec<ModelModality>>) Self`
- Signature6: `+new(name: impl Into<String>, property_type: ToolParameterPropertyType, description:
  impl Into<String>) Self`

## Extraction warnings

- `Unresolved or out-of-scope base: LlmError -> From<OpenAiClientError>`
- `Unresolved or out-of-scope base: OpenAiCompatibleClientBuilder -> Default`
