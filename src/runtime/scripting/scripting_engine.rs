use rhai::{AST, Engine, ParseError, EvalAltResult};

pub struct ScriptingEngine {
    engine: Engine,
    // Stores the successfully compiled script content for retrieval/inspection
    compiled_script: Option<String>,
}
/// Call this after exposing structs
impl ScriptingEngine {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            compiled_script: None,
        }
    } 
    pub fn include_script(script: String) -> Result {
        
    }
    fn compile_script_to_ast(&mut self, script: &str) ->  Result<AST, ParseError> {
        self.compiled_script = Some(script.to_string());
        let code = self.engine.compile(script);
        code
    }
}
