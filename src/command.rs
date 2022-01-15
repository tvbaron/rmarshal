pub const LUA_PRELUDE: &str = r#"
-- Tests whether a given table can be treated as an array.
-- @param tab [table]
-- @return [boolean]
function is_table_array(tab)
    local idx = 1
    for k, _ in pairs(tab) do
        if k ~= idx then
            return false
        end
        idx = idx + 1
    end

    return true
end

-- Returns the type of a given value.
-- @return [string] Either 'null', 'object', 'array' or any of what the function type() returns.
function typeof(v)
    local t = type(v)
    if t == 'table' then
        if v._classname == 'NullClass' then
            return 'null'
        elseif v._classname == 'Object' then
            return 'object'
        elseif v._classname == 'Array' or is_table_array(v) then
            return 'array'
        end

        return 'table'
    end

    return t
end

-- Represents the NULL value (different than nil).
NullClass = {
    _classname = 'NullClass',
}
NullClass.__index = NullClass
NULL = {}
setmetatable(NULL, NullClass)

-- Represents an Array.
Array = {
    _classname = 'Array',
}
Array.__index = Array

-- Constructs a new Array.
-- @return [Array]
function Array:new(init)
    local arr = {}
    if type(init) == 'table' then
        for _, v in ipairs(init) do
            table.insert(arr, v)
        end
    end
    setmetatable(arr, self)

    return arr
end

-- Constructs a new Array from either an other one or a table.
-- @param other [Array|table]
-- @return [Array]
function Array:from(other)
    if type(other) == 'table' then
        return Array:new(other)
    end

    error('wrong format')
end

function Array:iterator()
    return ipairs(self)
end

-- Removes the last value of the Array represented by this one.
-- @return [any]
function Array:pop()
    local last_val = self[#self]
    table.remove(self)

    return last_val
end

-- Adds a new value at the end of the Array represented by this one.
-- @param value [any]
-- @return [any]
function Array:push(value)
    table.insert(self, value)

    return value
end

-- Removes the first value of the Array represented by this one.
-- @param value [any]
-- @return [any]
function Array:shift()
    local first_val = self[1]
    table.remove(self, 1)

    return first_val
end

-- Adds a new value at the front of the Array represented by this one.
-- @param value [any]
-- @return [any]
function Array:unshift(value)
    table.insert(self, 1, value)

    return value
end

-- Creates a new Array populated with the results of calling a provided function
-- on every element in the Array represented by this one.
-- @param callback [function]
-- @return [Array]
function Array:map(callback)
    local res = Array:new()
    for _, v in ipairs(self) do
        res:push(callback(v))
    end

    return res
end

-- Represents an Object with key insertion order iterator.
Object = {
    _classname = 'Object',
}
Object.__index = Object

-- Constructs a new Object.
-- @return [Object]
function Object:new(init)
    local obj = {
        _keys = {},     -- Array of keys (for insertion order).
        _values = {},   -- Map of key-value pairs.
    }
    if type(init) == 'table' then
        for _, tuple in ipairs(init) do
            local k = tuple[1]
            if type(k) ~= 'string' or k:len() <= 0 then
                error('wrong key format')
            end

            if obj._values[k] == nil then
                table.insert(obj._keys, k)
            end
            obj._values[k] = tuple[2]
        end
    end
    setmetatable(obj, self)

    return obj
end

-- Constructs a new Object from either an other one or a table.
-- @param other [Object|table]
-- @return [Object]
function Object:from(other)
    local t = typeof(other)
    if t == 'object' then
        local new_obj = Object:new()
        for _, e in other:iterator() do
            new_obj:set(e.key, e.value)
        end
        return new_obj
    elseif t == 'table' then
        local new_obj = Object:new()
        for k, v in pairs(other) do
            new_obj:set(k, v)
        end
        return new_obj
    end

    error('wrong format')
end

-- Deletes a given key and its value.
-- @param key [string]
-- @return [any] The previous value if any, nil otherwise.
function Object:delete(key)
    for i, k in ipairs(self._keys) do
        if k == key then
            table.remove(self._keys, i)
            break
        end
    end

    local prev_val = self._values[key]
    self._values[key] = nil

    return prev_val
end

-- Returns the value of a given key.
-- @param key [string]
-- @return [any]
function Object:get(key)
    return self._values[key]
end

-- Tests whether a given key exists.
-- @param key [string]
-- @return [boolean]
function Object:has(key)
    return self._values[key] ~= nil
end

function _object_iterator(obj, idx)
    local next_idx = idx + 1
    local key = obj._keys[next_idx]
    if key == nil then
        return
    end

    local val = obj._values[key]
    local entry = {
        key = key,
        value = val,
    }

    return next_idx, entry
end

-- Returns an entry iterator.
function Object:iterator()
    return _object_iterator, self, 0
end

-- Sets a new value of a given key.
-- If the value is nil then the previous key-value pair is deleted.
-- @param key [string]
-- @param value [any]
-- @return [any] The previous value if any, nil otherwise.
function Object:set(key, value)
    if type(key) ~= 'string' or key:len() <= 0 then
        error('wrong key format')
    end

    if value == nil then
        return self:delete(key)
    end

    local prev_val = self._values[key]
    if prev_val == nil then
        -- The given key does not exist yet.
        table.insert(self._keys, key)
    end

    self._values[key] = value

    return prev_val
end

-- Merges the Object represented by this one with another Object.
-- @param other [Object]
-- @return [Object] The Object represented by this one (self).
function Object:merge(other)
    for _, e in other:iterator() do
        self:set(e.key, e.value)
    end

    return self
end

-- Returns an Array containing all keys in insertion order.
-- @return [Array]
function Object:keys()
    local keys = Array:new()
    for _, k in ipairs(self._keys) do
        keys:push(k)
    end

    return keys
end

-- Returns an Array containing all values in insertion order.
-- @return [Array]
function Object:values()
    local vals = Array:new()
    for _, k in ipairs(self._keys) do
        vals:push(self._values[k])
    end

    return vals
end

-- Represents a Context for accessing inputs and outputs.
Context = {
    _classname = "Context",
}
Context.__index = Context

function Context:new(ctx)
    local ctx = {
        inputs = {},
        outputs = {},
    }
    setmetatable(ctx, self)
    return ctx
end

function Context:get_input(idx)
    return self.inputs[idx]
end

function Context:get_inputs()
    return self.inputs
end

function Context:merge_inputs()
    local ret = Object:new()
    for _, input in ipairs(self.inputs) do
        ret:merge(input)
    end
    return ret
end

function Context:set_output(output)
    table.insert(self.outputs, output)
end

ctx = Context:new()
"#;
