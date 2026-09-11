local Extension = {
    prefix = "moeda_"
}

local symbols = {
    BRL = "R$",
    USD = "$",
    EUR = "€",
    GBP = "£",
    JPY = "¥"
}

local function format_number(amount, decimal_sep, thousand_sep)
    local integer_part, decimal_part = string.match(string.format("%.2f", amount), "^(%d+)%.(%d+)$")
    local k
    while true do
        integer_part, k = string.gsub(integer_part, "^(-?%d+)(%d%d%d)", "%1" .. thousand_sep .. "%2")
        if k == 0 then break end
    end
    return integer_part .. decimal_sep .. decimal_part
end

function Extension.add(key, value)
    local curr, amount_str = value:match("^(%a%a%a)%s+(%d+%.?%d*)$")
    if not curr or not amount_str then
        return false, "formato de moeda invalido. Use 'MOEDA VALOR' (ex: BRL 1250.50)"
    end

    curr = curr:upper()
    local amount = tonumber(amount_str)
    if not amount or amount < 0 then
        return false, "valor monetario deve ser um numero nao negativo"
    end

    local dec = amount_str:match("%.(%d+)$")
    if dec and #dec > 2 then
        return false, "valor monetario pode ter no maximo 2 casas decimais"
    end

    return true, string.format("%s %.2f", curr, amount)
end

function Extension.get(key, value)
    local curr, amount_str = value:match("^(%a%a%a)%s+(%d+%.%d%d)$")
    if not curr or not amount_str then
        return true, value
    end

    local amount = tonumber(amount_str)
    local symbol = symbols[curr]

    if curr == "BRL" or curr == "EUR" then
        local formatted_num = format_number(amount, ",", ".")
        if symbol then
            return true, string.format("%s %s", symbol, formatted_num)
        else
            return true, string.format("%s %s", curr, formatted_num)
        end
    else
        local formatted_num = format_number(amount, ".", ",")
        if symbol then
            return true, string.format("%s %s", symbol, formatted_num)
        else
            return true, string.format("%s %s", curr, formatted_num)
        end
    end
end

return Extension