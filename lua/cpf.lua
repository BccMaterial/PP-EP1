function add(key, value)
    --verifica se o formato da chave tá correto
    if not key:match("^(cpf_)(.*)$") then
        return false
    end

    -- Valida se possui apenas dígitos
    if not value:match("^%d+$") then
        error("Formato do CPF inválido (Insira apenas dígitos no banco)")
    end

    --verifica o tamanho
    if #value ~= 11 then
        error("Tamanho do CPF inválido (Deve ser 11 dígitos)")
    end

    local numeros = {}
    for i = 1, #value do --quebra a string em um array
        local digito = tonumber(value:sub(i, i))
        table.insert(numeros, digito)
    end

    local todos_iguais = true --verifica se todos sao iguais
    for i = 1, 11 do
        if numeros[i] ~= numeros[1] then
            todos_iguais = false
            break
        end
    end

    if todos_iguais then
        error("CPF inválido")
    end

    local soma = 0 --verifica o primeiro digito verificador

    for i = 1, 9 do
        soma = soma + numeros[i] * (10 - i + 1)
    end

    resto = soma % 11
    if (resto == 0 or resto == 1) then x = 0 else x = 11 - resto end


    if numeros[10] ~= x then
        error("CPF Inválido")
    end

    soma = 0

    for i = 1, 10 do --verifica o segundo digito verificador
        soma = soma + numeros[i] * (11 - i + 1)
    end
    resto2 = soma % 11
    if (resto2 == 0 or resto2 == 1) then
        y = 0
    else
        y = 11 - resto2
    end

    if numeros[11] ~= y then
        error("CPF Inválido")
    end

    return true
end

function get(key, value)
    -- Valida se a chave é CPF
    if not key:match("^(cpf_)(.*)$") then
        return false
    end

    -- Valida o tamanho
    if #value ~= 11 then
        error("Tamanho do CPF inválido")
    end
    formato =
        string.sub(value, 1, 3) ..
        "." ..
        string.sub(value, 4, 6) ..
        "." ..
        string.sub(value, 7, 9) ..
        "-" ..
        string.sub(value, 10, 11)

    return formato
end
