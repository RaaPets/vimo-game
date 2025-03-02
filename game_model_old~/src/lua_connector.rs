use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

use mlua::Lua;
use mlua::{Value, Variadic};

//  //  //  //  //  //  //  //
pub fn init(source_code: &str) -> Result<Lua> {
    let lua = Lua::new();
    set_printer(&lua)?;
    lua.load(source_code).exec()?;
    Ok(lua)
}

//  //  //  //  //  //  //  //
fn set_printer(lua: &Lua) -> Result<()> {
    fn lua_printer(print_args: &Variadic<Value>) {
        let mut msg = String::from("LUA: ---> print <---\nLUA: ");
        for item in print_args.iter() {
            let item_str = match item.to_string() {
                Ok(s) => s,
                Err(_) => String::from("<error>"),
            };
            msg.push_str(&item_str);
            msg.push('\t');
        }
        info!("{}", msg);
    }
    let lua_print = lua.create_function(move |_, lua_args: Variadic<Value>| {
        lua_printer(&lua_args);
        Ok(())
    })?;
    lua.globals().set("print", lua_print)?;

    trace!("Lua has been loaded and executed");
    Ok(())
}

//  //  //  //  //  //  //  //
//        TEST              //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod lua_tests {
    use super::*;

    #[test]
    fn basic_creating() -> Result<()> {
        let code = "print(1,2,'text #1', 3)";
        let _ = init(code)?;
        Ok(())
    }

    #[test]
    fn basic_fail_loading() -> Result<()> {
        let code = "- comment";
        let lua = init(code);
        assert!(lua.is_err(), "Must be a Lua syntax Error");
        Ok(())
    }
}
