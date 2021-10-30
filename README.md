# rmarshal

## Parameter Syntax

    <syntax>    ::= <unit> | <unit> <parameters>
    <unit>      ::= <command> | <modifier> <format> <path>
    <command>   ::= "--check" | "--merge"
    <modifier>  ::= "" | "--pretty" <modifier>
    <format>    ::= "" | "--json" | "--toml" | "--yaml" | "--lua" | "--template"
    <path>      ::= "" | <character> <path>
    <character> ::= <letter> | <digit> | <symbol>

## Examples

### Convert JSON to YAML

    rmarshal sample.json --yaml

### Convert YAML to pretty JSON

    rmarshal sample.yaml --pretty --json

### Merge multiple files into one

    rmarshal in1.json in2.toml in3.yaml --merge out.json

### Process an object with a Lua script

    rmarshal sample.json script.lua

### Transform a template with an object

    rmarshal sample.json --template report.templ
