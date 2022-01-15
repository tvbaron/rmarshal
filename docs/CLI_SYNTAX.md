# CLI Syntax - rmarshal

Inputs and outputs are expressed in the same way.
Before the first command, everything is interpreted as an input.
After the last command, everything is interpreted as an output.

A PATH may be replaced by - to express either stdin or stdout depending on the context.

## Syntax

    <syntax>                  ::= "--help" | "--version" | <pipeline>
    <pipeline>                ::= <unit> | <pipeline> <whitespace> <unit>
    <unit>                    ::= <path>
                                | <format> <opt_format_modifiers> <whitespace> <path>
                                | <document>
                                | <command>
    <format>                  ::= "--plain"
                                | "--json"
                                | "--lua"
                                | "--toml"
                                | "--yaml"
    <opt_format_modifiers>    ::= ""
                                | <whitespace> "--dots" <opt_format_modifiers>
                                | <whitespace> "--eol" <opt_format_modifiers>
                                | <whitespace> "--fix" <opt_format_modifiers>
                                | <whitespace> "--pretty" <opt_format_modifiers>
                                | <whitespace> "--stream" <opt_stream_limit> <opt_format_modifiers>
    <opt_stream_limit>        ::= ""
                                | "=" <integer>
    <document>                ::= "--document" <whitespace> <document_hint_long> <whitespace> <text>
                                | "-D" <whitespace> <document_hint_long> <whitespace> <text>
                                | "-D" <document_hint_short> <opt_whitespace> <text>
    <document_hint_long>      ::= "any"
                                | "nil"
                                | "boolean"
                                | "integer"
                                | "float"
                                | "string"
                                | "json"
                                | "lua"
    <document_hint_short>     ::= "_" | "N" | "B" | "I" | "F" | "S" | "J" | "L"
    <command>                 ::= "--check"
                                | "--concat"
                                | "--copy"
                                | "--merge" <merge_modifiers>
                                | "--pack"
                                | "--unpack"
                                | "--render" <whitespace> <path>
                                | "--transform" <whitespace> <path>
    <merge_modifiers>         ::= ""
                                | <whitespace> "--depth" <whitespace> <signed_integer> <merge_modifiers>
    <path>                    ::= <character> | <character> <path>
    <text>                    ::= <character> | <character> <text>
    <character>               ::= <letter> | <digit> | <symbol>
    <signed_integer>          ::= "-" <integer> | <integer>
    <integer>                 ::= <digit> | <integer> <digit>
    <opt_whitespace>          ::= "" | <whitespace>
    <whitespace>              ::= " " | <whitespace> " "
