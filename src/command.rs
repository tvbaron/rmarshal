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

Object = {
    _classname = "Object",
}

function Object:new(seq)
    local obj = {}
    if seq ~= nil then
        local field = nil
        for idx, val in ipairs(seq) do
            if (idx % 2) == 0 then
                table.insert(obj, {
                    name = field,
                    value = val,
                })
            else
                field = val
            end
        end
    end
    setmetatable(obj, self)
    self.__index = self
    return obj
end

function Object:has(field)
    for _, entry in ipairs(self) do
        if entry.name == field then
            return true
        end
    end

    return false
end

function Object:get(field)
    for _, entry in ipairs(self) do
        if entry.name == field then
            return entry.value
        end
    end

    return nil
end

function Object:set(field, value)
    for _, entry in ipairs(self) do
        if entry.name == field then
            local old_val = entry.value
            entry.value = value
            return old_val
        end
    end

    table.insert(self, {
        name = field,
        value = value,
    })
    return nil
end

Context = {
    _classname = "Context",
}

function Context:new(ctx)
    setmetatable(ctx, self)
    self.__index = self
    return ctx
end

function Context:get_inputs()
    return self.inputs
end

function Context:set_output(output)
    self.output = output
end

ctx = Context:new({
    inputs = {},
    output = nil,
})
"#;
