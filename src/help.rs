pub const GLOBAL_HELP: &str = r#"Usage: rmarshal [INPUT...] COMMAND [OUTPUT...]

Document remarshaller.

Inputs and outputs are expressed in the same way.
Before the first command, everything is interpreted as an input.
After the last command, everything is interpreted as an output.

A PATH may be replaced by - to express either stdin or stdout depending on the context.

Available help:
        --version                       Print program name and version.
        --help                          Print this help.
        --help TOPIC                    Print help on a given TOPIC.

Available commands:
        --check                         Check multiple documents.
        --concat                        Concatenate multiple array-based documents.
    -C, --copy                          Change the format of multiple documents.
        --merge [OPTION...]             Merge multiple documents.
        --pack                          Create one array-based document from multiple documents.
        --unpack                        Create multiple documents from one array-based document.
    -R, --render PATH                   Render a template with multiple documents.
    -T, --transform PATH                Transform multiple documents with a Lua script.

Available input/output:
        PATH                            A file. The format is inferred from the extension.
        --FORMAT [OPTION...] PATH       A file. FORMAT may be plain, json, lua, toml or yaml.
    -D, --document HINT VALUE           A inline document. Input only.
"#;

pub const TOPIC_HELP: &str = r#"Usage: rmarshal --help TOPIC

Interative help.

Available topics:
    check               Command to check multiple documents.
    concat              Command to concatenate multiple array-based documents.
    copy                Command to change the format of multiple documents.
    document            Define an inline document.
    json                Define a file with a JSON document.
    lua                 Define a file with a Lua document.
    merge               Command to merge multiple documents.
    pack                Command to create one array-based document from multiple documents.
    plain               Define a file with a string-based document.
    render              Command to render a template with multiple documents.
    toml                Define a file with a TOML document.
    transform           Command to transform multiple documents with a Lua script.
    unpack              Command to create multiple documents from one array-based document.
    yaml                Define a file with YAML document(s).
"#;

pub const CHECK_HELP: &str = r#"Usage: rmarshal INPUT... --check

Read multiple documents, at least one.
Do not create anything.

Example:
    rmarshal doc1.json doc2.toml doc3.yaml --check
"#;

pub const CONCAT_HELP: &str = r#"Usage: rmarshal INPUT... --concat OUTPUT

Read multiple documents, at least one.
Write one document.

Example:
    rmarshal doc1.json doc2.toml doc3.yaml --concat out.yaml
"#;

pub const COPY_HELP: &str = r#"Usage: rmarshal INPUT... --copy OUTPUT...

Read multiple documents, at least one.
Write the same number of documents.

Example:
    rmarshal doc1.json doc2.toml doc3.yaml --copy out1.yaml out2.json out3.toml
"#;

pub const DOCUMENT_HELP: &str = r#"Usage: rmarshal --document HINT VALUE COMMAND [OUTPUT...]

Define an inline document. Input only.

Available hints:
    _, any              The actual hint is inferred from the value.
    N, nil              Interpret the value as nil type.
    B, boolean          Interpret the value as boolean type.
    I, integer          Interpret the value as integer type.
    F, float            Interpret the value as float type.
    S, string           Interpret the value as string type.
    L, lua              Interpret the value as a lua document.

Examples:
    rmarshal --document string Hello --copy --yaml -
            ---
            Hello
    rmarshal -D_Hi --copy --yaml -
            ---
            Hi
"#;

pub const JSON_HELP: &str = r#"Usage: rmarshal --json [OPTION...] PATH COMMAND --json [OPTION...] PATH

Define a file with a JSON document.

Available options:
        --eol                   Add a trailing newline character at the end of each document. Output only.
        --pretty                Activate pretty format. Output only.
    -s, --stream[=LIMIT]        Allow multiple documents within a single file. Output only.

Example:
    cat doc.json
            {"msg":"hi","values":{"a":1,"b":2}}
    rmarshal --json doc.json --copy --json --pretty --eol out.json
    cat out.json
            {
              "msg": "hi",
              "values": {
                "a": 1,
                "b": 2
              }
            }
"#;

pub const LUA_HELP: &str = r#"Usage: rmarshal --lua [OPTION...] PATH COMMAND --lua [OPTION...] PATH

Define a file with a LUA document.

Available options:
        --eol                   Add a trailing newline character at the end of each document. Output only.
    -s, --stream[=LIMIT]        Allow multiple documents within a single file. Output only.

Example:
    cat doc.lua
            Object:new({
                { "msg", "Hello" },
                { "values", Array:new({ 1, 2, 3 }) },
            })
    rmarshal --lua doc.lua --copy --lua --eol out.lua
    cat out.lua
            Object:new({{"msg","Hello"},{"values",Array:new({1,2,3,})},})
"#;

pub const MERGE_HELP: &str = r#"Usage: rmarshal INPUT... --merge [--depth VALUE] OUTPUT

Read multiple documents, at least one.
Write one document.

Example:
    cat doc1.json
            {"msg":"hi","values":{"a":1,"b":2}}
    cat doc2.toml
            level = 1
    cat doc3.yaml
            ---
            msg: hello
            values:
              b: 3
              c: 4
    rmarshal doc1.json doc2.toml doc3.yaml --merge out1.yaml
    cat out1.yaml
            ---
            msg: hello
            values:
              a: 1
              b: 3
              c: 4
            level: 1
    rmarshal doc1.json doc2.toml doc3.yaml --merge --depth 1 out2.yaml
    cat out2.yaml
            ---
            msg: hello
            values:
              b: 3
              c: 4
            level: 1
"#;

pub const PACK_HELP: &str = r#"Usage: rmarshal INPUT... --pack OUTPUT

Read multiple documents, at least one.
Write one document.

Example:
    rmarshal doc1.json doc2.toml doc3.yaml --pack out.yaml
"#;

pub const PLAIN_HELP: &str = r#"Usage: rmarshal --plain [OPTION...] PATH COMMAND --plain [OPTION...] PATH

Define a plain file with a string-based document.

Available options:
        --eol                   Add a trailing newline character at the end of each document. Output only.
    -s, --stream[=LIMIT]        Allow multiple documents within a single file. Output only.

Example:
    cat doc.toml
            [package]
            name = "rmarshal"
    rmarshal --plain doc.toml --copy --plain -
    cat out.toml
            [package]
            name = "rmarshal"
"#;

pub const RENDER_HELP: &str = r#"Usage: rmarshal [INPUT...] --render PATH OUTPUT

Read multiple documents, may be none.
Write one string-based document.

Example:
    cat data
            {"name":"Althea","fingers":10}
    cat report
            % local data = ctx:get_input(1)
            My name is <%= data:get('name') %> and I have <%= data:get('fingers') %> fingers!
    rmarshal data.json --render report out
    cat out
            My name is Althea and I have 10 fingers!
"#;

pub const TOML_HELP: &str = r#"Usage: rmarshal --toml [OPTION...] PATH COMMAND --toml [OPTION...] PATH

Define a file with a TOML document.

Available options:
        --fix                   Fix document to circumvent serializer errors.
    -s, --stream[=LIMIT]        Allow multiple documents within a single file. Output only.

Example:
    cat doc.toml
            [package]
            name = "rmarshal"
    rmarshal --toml doc.toml --copy --toml out.toml
    cat out.toml
            [package]
            name = "rmarshal"
"#;

pub const TRANSFORM_HELP: &str = r#"Usage: rmarshal [INPUT...] --transform PATH [OUTPUT...]

Read multiple documents, may be none.
Write multiple documents, may be none.

Example:
    cat doc1.json
            {"value":"hi"}
    cat doc2.toml
            value = "hello"
    cat doc3.yaml
            ---
            value: hey
    cat script.lua
            local doc1 = ctx:get_input(1)
            local doc2 = ctx:get_input(2)
            local doc3 = ctx:get_input(3)
            local out = Object:new()
            out:set('alfa', doc1:get('value'))
            out:set('bravo', doc2:get('value'))
            out:set('charlie', doc3:get('value'))
            ctx:set_output(out)
    rmarshal doc1.json doc2.toml doc3.yaml --transform script.lua out.yaml
    cat out.yaml
            ---
            alfa: hi
            bravo: hello
            charlie: hey
"#;

pub const UNPACK_HELP: &str = r#"Usage: rmarshal INPUT --unpack OUTPUT...

Read one document.
Write multiple documents, at least one.

Example:
    rmarshal doc.yaml --unpack out1.json out2.toml out3.yaml
"#;

pub const YAML_HELP: &str = r#"Usage: rmarshal --yaml [OPTION...] PATH COMMAND --yaml [OPTION...] PATH

Define a file with YAML document(s).

Available options:
        --dots                  Add the trailing 3 dots at the end of each document. Output only.
    -s, --stream[=LIMIT]        Allow multiple documents within a single file.

Example:
    cat doc.yaml
            ---
            msg: hi
    rmarshal --yaml doc.yaml --copy --yaml --dots out.yaml
    cat out.yaml
            ---
            msg: hi
            ...
"#;
