use std::usize;

use rhai::{AST, Engine, ParseError, EvalAltResult};


const LOGGING: bool = true;

type ScriptKey = usize; 

pub struct ScriptingEngine<'a> {
    engine: &'a Engine,
    // Stores the successfully compiled script content for retrieval/inspection
    compiled_scripts: Vec<AST>,
}
/// Call this after exposing structs
impl<'a> ScriptingEngine<'a> {
    pub fn new(engine: &'a Engine) -> Self {
        Self {
            engine,
            compiled_scripts: Vec::new(),
        }
    } 
    pub fn include_script(&mut self, script: String) {
        let compiled = self.compile_script_to_ast(&script);
        match compiled {
            Ok(ast) => {
                self.compiled_scripts.push(ast);
            }
            Err(err) => {
                // Don't want to panic when script fails to compile just log
                // errors in eval shouldn't cause a panic
                println!("{}",err);
            }
        }
    }
    fn compile_script_to_ast(&self, script: &str) ->  Result<AST, ParseError> {
        let code = self.engine.compile(script);
    
        code
    }
    pub fn eval_script<T>(&self, key: ScriptKey) ->  Result<T, Box<EvalAltResult>> 
        where T: 'static + Clone,
    {
        let r = self.engine.eval_ast(self.compiled_scripts.get(key)
            .expect("There is no script found with {} script key"));
        r
    }
}