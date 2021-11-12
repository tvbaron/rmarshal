# rmarshal

## CLI Syntax

    <syntax>        ::= "--help" | "--version" | <unit_seq>
    <unit_seq>      ::= "" | <unit> <unit_seq>
    <unit>          ::= <command> | <format> <modifier> <path> | <path>
    <command>       ::= "--check" | "--concat" | "--copy" | "--merge" | "--pack" | "--unpack"
    <format>        ::= "--json" | "--toml" | "--yaml" | "--lua" | "--template"
    <modifier>      ::= "" | "--pretty" <modifier>
    <path>          ::= <character> <character_seq>
    <character_seq> ::= "" | <character> <character_seq>
    <character>     ::= <letter> | <digit> | <symbol>

## Template

The engine recognizes certain tags in the provided template and converts them based on the following rules:

    <% Lua code. %>
    <%= Lua expression -- replaced with result. %>
    <%# Comment -- not rendered. %>
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

    rmarshal sample.json script.lua out.json

### Render a template

    rmarshal sample.json --template report.templ out.txt
