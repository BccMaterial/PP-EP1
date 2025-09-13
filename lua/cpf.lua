function add(key,value)
    if #value ~= 11 then --verifica o tamanho
        return false
    end
    
    if not value:match("^%d+$") then --checa se tem letra
    return false
    end
    
    local numeros = {}
    local n = 10
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
    if todos_iguais then return false end
    
    local soma = 0 --verifica o primeiro digito verificador
    for i = 1, 9 do
        soma = soma + numeros[i] * (10 - i + 1)
    end
    resto = soma%11
    if (resto == 0 or resto == 1) then x = 0 else x = 11-resto end
    
    
    if numeros[10] ~= x then return false end
    soma = 0
    
    for i = 1, 10 do --verifica o segundo digito verificador
        soma = soma + numeros[i] * (11 - i + 1)
    end
    resto2 = soma%11
    if (resto2 == 0 or resto2 == 1) then y = 0 else y = 11-resto2 end
    if numeros[11] ~= y then return false end
    
return key,true
    
end


function get(key,value)
    return key,value
end