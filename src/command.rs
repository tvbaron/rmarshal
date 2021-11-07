pub const LUA_PRELUDE: &str = r#"
-- Tests whether a given table can be treated as an array.
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

function Array:new(init)
    local arr = {}
    if (type(init) == 'table') then
        for _, v in ipairs(init) do
            table.insert(arr, v)
        end
    end
    setmetatable(arr, self)
    return arr
end

function Array:iterator()
    return ipairs(self)
end

function Array:pop()
    local last_val = self[#self]
    table.remove(self)
    return last_val
end

function Array:push(value)
    table.insert(self, value)
    return value
end

function Array:shift()
    local first_val = self[1]
    table.remove(self, 1)
    return first_val
end

function Array:unshift(value)
    table.insert(self, 1, value)
    return value
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
        keys = {},
        values = {},
    }
    if (type(init) == 'table') then
        if (is_table_array(init)) then
            for _, entries in ipairs(init) do
                for k, v in pairs(entries) do
                    table.insert(obj.keys, k)
                    obj.values[k] = v
                end
            end
        else
            for k, v in pairs(init) do
                table.insert(obj.keys, k)
                obj.values[k] = v
            end
        end
    end
    setmetatable(obj, self)
    return obj
end

-- Deletes a given key and its value.
-- @param key [string]
-- @return [any] The previous value if any, nil otherwise.
function Object:delete(key)
    for i, k in ipairs(self.keys) do
        if k == key then
            table.remove(self.keys, i)
            break
        end
    end

    local prev_val = self.values[key]
    self.values[key] = nil

    return prev_val
end

-- Returns the value of a given key.
-- @param key [string]
-- @return [any]
function Object:get(key)
    return self.values[key]
end

-- Tests whether a given key exists.
-- @param key [string]
-- @return [boolean]
function Object:has(key)
    return self.values[key] ~= nil
end

function _object_iterator(obj, idx)
    local next_idx = idx + 1
    local key = obj.keys[next_idx]
    if key == nil then
        return
    end

    local val = obj.values[key]
    local entry = {
        key = key,
        value = val,
    }

    return next_idx, entry
end

function Object:iterator()
    return _object_iterator, self, 0
end

-- Sets a new value of a given key.
-- If the value is nil then the previous key-value pair is deleted.
-- @param key [string]
-- @param value [any]
-- @return [any] The previous value if any, nil otherwise.
function Object:set(key, value)
    if (type(key) ~= 'string' or key:len() <= 0) then
        error('wrong key format')
    end

    if value == nil then
        return self:delete(key)
    end

    local prev_val = self.values[key]
    if prev_val == nil then
        -- The given key does not exist yet.
        table.insert(self.keys, key)
    end

    self.values[key] = value

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

Context = {
    _classname = "Context",
}
Context.__index = Context

function Context:new(ctx)
    local ctx = {
        inputs = {},
        output = nil,
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
    self.output = output
end

ctx = Context:new()
"#;
