# Lua Prelude - rmarshal

## Types and Values

Besides the standard Lua types, there are the __Array__ and the __Object__ types.

Since the __nil__ value is not insertable into a __table__, there is the special __NULL__ value.

## API

### NULL

The __NULL__ value represents the __null__ value in _JSON_ or the __~__ value in _YAML_.

### Array

#### Array:new

Create a new Array with optional initial elements.

##### Usage

    local arr1 = Array:new()    -- An empty array.
    local arr2 = Array:new({    -- An array with initial elements.
        1, 2, 3
    })

#### Array:from

Create a new Array from either an other one or a table.

##### Usage

    local arr1 = Array:from({ 1, 2, 3 })

#### Array:len

Return the length of the Array.

##### Usage

    local arr1 = Array:new({ 1, 2, 3 })
    arr1:len()      -- 3

#### Array:empty

Test whether the Array is empty.

##### Usage

    local arr1 = Array:new()
    arr1:empty()    -- true
    local arr2 = Array:new({ 1, 2, 3 })
    arr2:empty()    -- false

#### Array:iterator

Return an iterator of all elements exactly like _ipairs_.

##### Usage

    local arr1 = Array:from({ "a", "b", "c" })
    for i, v in arr1:iterator() do
        i       -- index
        v       -- value
    end

#### Array:pop

Remove the last element. Return the element or _nil_ if the Array is empty.

##### Usage

    local arr1 = Array:new({ "a", "b", "c" })
    arr1:pop()      -- "c"
    arr1:pop()      -- "b"
    arr1:pop()      -- "a"
    arr1:pop()      -- nil

#### Array:push

Add a new element at the end of the Array.

##### Usage

    local arr1 = Array:new()
    arr1:push("a")      -- "a"
    arr1:push("b")      -- "b"
    arr1:push("c")      -- "c"

#### Array:shift

Remove the first element. Return the element or _nil_ if the Array is empty.

##### Usage

    local arr1 = Array:new({ "a", "b", "c" })
    arr1:shift()    -- "a"
    arr1:shift()    -- "b"
    arr1:shift()    -- "c"
    arr1:shift()    -- nil

#### Array:unshift

Add a new element at the front of the Array.

##### Usage

    local arr1 = Array:new()
    arr1:unshift("c")       -- "c"
    arr1:unshift("b")       -- "b"
    arr1:unshift("a")       -- "a"

#### Array:map

Creates a new Array populated with the results of calling a provided function on
every element in the Array.

##### Usage

    local arr1 = Array:from({ 1, 2, 3 })
    arr1:map(function (e)
        return (e * 2) + 1
    end)
    arr1[1]     -- 3
    arr1[2]     -- 5
    arr1[3]     -- 7

### Object

#### Object:new

Create a new Object with optional initial entries.

##### Usage

    local obj1 = Object:new()       -- An empty object.
    local obj2 = Object:new({       -- An object with initial entries.
        { "name", "Althea" },
        { "fingers", 10 },
    })

#### Object:from

Create a new Object from either an other one or a table.

##### Usage

    local other1 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    local obj1 = Object:from(other1)    -- An new Object from an other Object.
    local other2 = { name = "Althea", fingers = 10 }
    local obj2 = Object:from(other2)    -- An new Object from a table.

#### Object:len

Return the number of entries of the Object.

##### Usage

    local obj1 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    obj1:len()      -- 2

#### Array:empty

Test whether the Object is empty.

##### Usage

    local obj1 = Object:new()
    obj1:empty()    -- true
    local obj2 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    obj2:empty()    -- false

#### Object:delete

Delete the entry for a given key. Return the value of deleted entry if it exists or nil otherwise.

##### Usage

    local obj1 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    local fingers = obj1:delete("fingers")

#### Object:get

Retrieve the value associated to a given key or nil if the entry does not exist.

##### Usage

    local obj1 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    local fingers = obj1:get("fingers")

#### Object:has

Test whether a given key exists. Return _true_ of the key exist or _false_ otherwise.

##### Usage

    local obj1 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    obj1:has("fingers")     -- true
    obj1:has("toes")        -- false

#### Object:set

Set a new value for a given key. Return the previous value if any, _nil_ otherwise.

##### Usage

    local obj1 = Object:new()
    obj1:set("name", "Althea")
    obj1:set("fingers", 10)

#### Object:iterator

Return an iterator of all entries in insertion order.

##### Usage

    local obj1 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    for _, e in obj1:iterator() do
        e.key       -- Entry key.
        e.value     -- Entry value.
    end

#### Object:merge

Merge the entries with the ones of an other Object.

##### Usage

    local obj1 = Object:new({ { "name", "Althea" }, { "fingers", 9 } })
    local obj2 = Object:new({ { "fingers", 10 }, { "toes", 10 } })
    obj1:merge(obj2)
    obj1:get("name")        -- "Althea"
    obj1:get("fingers")     -- 10
    obj1:get("toes")        -- 10

#### Object:keys

Create an Array with all the keys in insertion order.

##### Usage

    local obj1 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    local keys = obj1:keys()
    keys[1]     -- "name"
    keys[2]     -- "fingers"

#### Object:values

Create an Array with all the values in insertion order.

##### Usage

    local obj1 = Object:new({ { "name", "Althea" }, { "fingers", 10 } })
    local values = obj1:values()
    values[1]       -- "Althea"
    values[2]       -- 10

### other functions

#### typeof

The __typeof__ function tests the type of the given value.

Possibles types are:
- nil
- boolean
- number
- string
- function
- userdata
- thread
- null
- object
- array

### Context

#### Context:get_input

Retrieve the document at a given index.

##### Usage

    local input1 = ctx:get_input(1)     -- Get the first input document.
    local input2 = ctx:get_input(2)     -- Get the second input document.

#### Context:set_output

Set an output document.

##### Usage

    ctx:set_output("Hello")     -- Set the first output document.
    ctx:set_output("World")     -- Set the second output document.
