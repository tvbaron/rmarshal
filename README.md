# rmarshal

_rmarshal_ is a document processor.

## CLI Syntax

    <syntax>            ::= "--help" | "--version" | <pipeline>
    <pipeline>          ::= <unit> | <pipeline> <unit>
    <unit>              ::= <command> | <format> <path> | <path>
    <command>           ::= "--check"
                          | "--concat"
                          | "--copy"
                          | "--merge" <merge_modifiers>
                          | "--pack"
                          | "--unpack"
                          | "--lua" <path>
                          | "--template" <path>
    <merge_modifiers>   ::= ""
                          | "--depth" <signed_integer> <merge_modifiers>
    <format>            ::= "--json" <json_modifiers>
                          | "--toml" <toml_modifiers>
                          | "--yaml" <yaml_modifiers>
    <json_modifiers>    ::= ""
                          | "--eol" <json_modifiers>
                          | "--pretty" <json_modifiers>
    <toml_modifiers>    ::= ""
                          | "--fix" <toml_modifiers>
    <yaml_modifiers>    ::= ""
                          | "--dots" <yaml_modifiers>
                          | "--eol" <yaml_modifiers>
    <path>              ::= <character> | <character> <text>
    <character>         ::= <letter> | <digit> | <symbol>
    <signed_integer>    ::= "-" <integer> | <integer>
    <integer>           ::= <digit> | <integer> <digit>

## Concat

Creates an array-based document by concatenating multiple array-based documents.

    rmarshal INPUT... --concat OUTPUT

## Merge

Merges multiple documents into one.

    rmarshal INPUT... --merge [--depth DEPTH] OUTPUT

The depth is meant for array and object values. It indicates the merging depth.

For example:
- a depth of value 0 will always applied the second operand.
- a depth of value 1 will merge only the first level of an array or a object value.

No depth option or a negative value indicates an infinite depth.

## Template

The engine recognizes certain tags in the provided template and converts them based on the following rules:

    <% Lua code. %>
    <%= Lua expression -- replaced with result. %>
    <%# Comment -- not rendered. %>
    % A line of Lua code -- treated as <% line %>
    %% replaced with % if first thing on a line and % processing is used
    <%% or %%> -- replaced with <% or %> respectively.

Any leading whitespace are removed if the directive starts with `<%-`.

Any trailing whitespace are removed if the directive ends with `-%>`.

## Examples

### Convert a JSON file to to a YAML file

    rmarshal sample.json --copy out.yaml

### Convert a YAML file to a pretty JSON file

    rmarshal sample.yaml --copy --json --pretty out.json

### Merge multiple files into one

    rmarshal in1.json in2.toml in3.yaml --merge out.json

### Edit a file with a Lua script

    rmarshal sample.json --lua script.lua out.json

### Render a template

    rmarshal sample.json --template report.templ out.txt
