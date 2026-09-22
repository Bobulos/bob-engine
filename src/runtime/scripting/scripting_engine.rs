use rhai::{Engine, EvalAltResult};


pub struct ScriptingEngine {
    pub engine: Option<Engine>, 
}
/// Call this after exposing structs
impl ScriptingEngine {
    pub fn new() -> Self {
        Self { engine: None }
    }
    pub fn initialize_scripting() -> Result<(), Box<EvalAltResult>> {
        let mut engine = Engine::new();
    
        // engine.register_type_with_name::<TestStruct>("TestStruct")
        //       .register_fn("new_ts", TestStruct::new)
        //       .register_fn("update", TestStruct::update);
    
        // let result = engine.eval::<TestStruct>(
        //     "let x = new_ts(); x.update(41); x"
        // )?;
    
        println!("Result field: {}", result.field); // Prints 42
        Ok(())
    }   
}