local Extension = {
    prefix = "cpf_"
}

local function is_all_same_digits(s)
    local first = s:sub(1, 1)
    for i = 2, #s do
        if s:sub(i, i) ~= first then
            return false
        end
    end
    return true
end

local function validate_cpf_math(cpf)
    if #cpf ~= 11 or not cpf:match("^%d+$") then
        return false, "CPF deve conter exatamente 11 digitos numericos"
    end

    if is_all_same_digits(cpf) then
        return false, "CPF invalido (digitos repetidos)"
    end

    -- Primeiro dígito verificador
    local sum1 = 0
    for i = 1, 9 do
        local digit = tonumber(cpf:sub(i, i))
        sum1 = sum1 + digit * (11 - i)
    end
    local d1 = (sum1 * 10) % 11
    if d1 == 10 then d1 = 0 end

    if d1 ~= tonumber(cpf:sub(10, 10)) then
        return false, "CPF invalido (primeiro digito verificador incorreto)"
    end

    -- Segundo dígito verificador
    local sum2 = 0
    for i = 1, 10 do
        local digit = tonumber(cpf:sub(i, i))
        sum2 = sum2 + digit * (12 - i)
    end
    local d2 = (sum2 * 10) % 11
    if d2 == 10 then d2 = 0 end

    if d2 ~= tonumber(cpf:sub(11, 11)) then
        return false, "CPF invalido (segundo digito verificador incorreto)"
    end

    return true, nil
end

function Extension.add(key, value)
    local is_valid, err_msg = validate_cpf_math(value)
    if not is_valid then
        return false, err_msg
    end

    -- Regra de Unicidade: consulta genérica ao banco via callback exposed pelo Rust
    if db and type(db.find_key_by_value) == "function" then
        local existing_key = db.find_key_by_value(value)
        if existing_key and existing_key ~= key then
            return false, "CPF ja cadastrado na chave " .. existing_key
        end
    end

    return true, value
end

function Extension.get(key, value)
    if #value == 11 then
        local formatted = string.format("%s.%s.%s-%s",
            value:sub(1, 3),
            value:sub(4, 6),
            value:sub(7, 9),
            value:sub(10, 11)
        )
        return true, formatted
    end
    return true, value
end

return Extension
