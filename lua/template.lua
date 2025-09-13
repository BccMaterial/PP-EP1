-- Sempre retorna (string, bool)
function get(key, value)
    print("get")
    return value, true
end

-- Sempre retorna (bool)
function add(key, value)
    print("add")
    return true
end
