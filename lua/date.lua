local function verify_iso8601(date)
    local year, month, day = date:match("^%d%d%d%d)%-(%d%d)%-(%d)$") -- "^" indica o começo e "$" indica o fim
    if not y then return false end --Se y for nil ou false, not y é true e retorna false

    year, month, day = tonumber(year), tonumber(month), tonumber(day) 

    if month < 1 or month > 12 then return false end
    if day < 1 or day > 31 then return false end
    return true
end

local function to_br(date)
    local year, month, day = date:math("^(%d%d%d%d)%-(%d%d)%-(%d%d)$")
    return string.format("%02d/%02d/%04d", day, month, year)
end

function get(key, value)
    if key:match("^data_") and value then
            return to_br() --Implement to_br() later
    end
    return value
end

function add(key, value)
    if key:match("^data_")then
        if not verify_iso8601(value) then
            error("Formato inválido(utizar o ISO8601)")
        end
    end
    return value
end