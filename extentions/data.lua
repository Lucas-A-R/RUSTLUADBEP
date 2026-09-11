local Extension = {
    prefix = "data_"
}

local function is_leap_year(year)
    return (year % 4 == 0 and year % 100 ~= 0) or (year % 400 == 0)
end

local function get_days_in_month(month, year)
    local days_per_month = { 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 }
    if month == 2 and is_leap_year(year) then
        return 29
    end
    return days_per_month[month]
end

function Extension.add(key, value)
    -- Validação estrita do formato YYYY-MM-DD
    local year_str, month_str, day_str = value:match("^(%d%d%d%d)%-(%d%d)%-(%d%d)$")
    if not year_str or not month_str or not day_str then
        return false, "formato de data invalido. Use YYYY-MM-DD (ex: 2022-10-23)"
    end

    local year = tonumber(year_str)
    local month = tonumber(month_str)
    local day = tonumber(day_str)

    if month < 1 or month > 12 then
        return false, "mes invalido (deve ser entre 01 e 12)"
    end

    local max_days = get_days_in_month(month, year)
    if day < 1 or day > max_days then
        return false, string.format("dia invalido para o mes %02d/%04d (maximo %d dias)", month, year, max_days)
    end

    return true, value
end

function Extension.get(key, value)
    local year_str, month_str, day_str = value:match("^(%d%d%d%d)%-(%d%d)%-(%d%d)$")
    if year_str and month_str and day_str then
        local formatted = string.format("%s/%s/%s", day_str, month_str, year_str)
        return true, formatted
    end
    return true, value
end

return Extension