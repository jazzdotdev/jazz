local function try(f, catch_f)
    local status, result = pcall(f)
    if not status then
        return catch_f(result)
    end

    return result
end

local uuid_pattern = "%x%x%x%x%x%x%x%x%-%x%x%x%x%-%x%x%x%x%-%x%x%x%x%-%x%x%x%x%x%x%x%x%x%x%x%x"

return {
    try = try,
    uuid_pattern = uuid_pattern,
}
