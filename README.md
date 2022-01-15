# rmarshal

_rmarshal_ is a document remarshaller.

## Overview

### Usage

    rmarshal [INPUT...] COMMAND [OUTPUT...]

### Command Line Interface

Inputs and outputs are expressed in the same way.
Before the first command, everything is interpreted as an input.
After the last command, everything is interpreted as an output.

A PATH may be replaced by - to express either stdin or stdout depending on the context.

See the [CLI Syntax](docs/CLI_SYNTAX.md) for more details.

## Commands

Commands consume and produce documents.

### Copy

The __copy__ command produces the same number of documents it consumes without any alteration.
The goal is to change the format of files.

#### Usage

    rmarshal [INPUT...] --copy [OUTPUT...]

#### Example

    $ cat data.json
    {"name":"Althea","fingers":10}
    $ rmarshal data.json --copy out.yaml
    $ cat out.yaml
    ---
    name: Althea
    fingers: 10

### Merge

The __merge__ command consumes multiple documents and produces one.

#### Usage

    rmarshal INPUT... --merge [--depth DEPTH] OUTPUT

#### Depth option

The depth is meant for array and object values. It indicates the merging depth.

For example:
- a depth of value 0 will always applied the second operand.
- a depth of value 1 will merge only the first level of an array or a object value.

No depth option or a negative value indicates an infinite depth.

### Render

The __render__ command consumes multiple documents and produces one string-based one.

#### Usage

    rmarshal [INPUT...] --render PATH OUTPUT

#### Tags

The engine recognizes certain tags in the provided template and converts them based on the following rules:

    <% Lua code. %>
    <%= Lua expression -- replaced with result. %>
    <%# Comment -- not rendered. %>
    % A line of Lua code -- treated as <% line %>
    %% replaced with % if first thing on a line and % processing is used
    <%% or %%> -- replaced with <% or %> respectively.

Any leading whitespace are removed if the directive starts with `<%-`.

Any trailing whitespace are removed if the directive ends with `-%>`.

#### Example

    $ cat data.json
    {"name":"Althea","fingers":10}
    $ cat report
    % local data = ctx:get_input(1)
    My name is <%= data:get('name') %> and I have <%= data:get('fingers') %> fingers!
    $ rmarshal data.json --render report out
    $ cat out
    My name is Althea and I have 10 fingers!

### Transform

The __transform__ command consumes and produces multiple documents.

#### Usage

    rmarshal [INPUT...] --transform PATH [OUTPUT...]

#### Lua Prelude

See the [Lua Prelude](docs/LUA_PRELUDE.md) for more details.

#### Example

    $ cat data.json
    {"name":"Althea","fingers":10}
    $ cat script.lua
    local data = ctx:get_input(1)
    data:set("name", "James Hook")
    data:set("rank", "captain")
    data:set("fingers", 5)
    ctx:set_output(data)
    $ rmarshal data.json --transform script.lua out.yaml
    $ cat out.yaml
    ---
    name: James Hook
    rank: captain
    fingers: 5

### Other commands

Other commands are __check__, __concat__, __pack__ and __unpack__.

## File Format

Available file formats are __plain__, __json__, __toml__, __yaml__ and __lua__.

The __plain__ format is the unformatted format.

## Version History

[Changelog](CHANGELOG.md).

## License

MIT.
