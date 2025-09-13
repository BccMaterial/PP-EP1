use mlua::{Lua, Result};

fn main() -> Result<()> {
    let lua = Lua::new();

    let lua_code = r#"
        function soma(a, b)
            return a + b
        end
        
        function concatena(str1, str2)
            return str1 .. " " .. str2
        end
    "#;

    println!("Executando código em lua...");
    lua.load(lua_code).exec()?;

    println!("Acessando globals...");
    let globals = lua.globals();

    let soma_func: mlua::Function = globals.get("soma")?;
    let resultado_soma: i32 = soma_func.call((5, 3))?;
    println!("Soma: {}", resultado_soma);

    // Chamar função concatena
    let concatena_func: mlua::Function = globals.get("concatena")?;
    let resultado_concatena: String = concatena_func.call(("Bom", "Dia"))?;
    println!("Concatenação: {}", resultado_concatena);

    Ok(())
}
