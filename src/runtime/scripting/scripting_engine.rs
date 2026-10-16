use rhai::{AST, Engine, ParseError, EvalAltResult};

pub struct ScriptingEngine {
    engine: Engine,
    compiled_scripts: Vec<AST>,
}

impl ScriptingEngine {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            compiled_scripts: Vec::new(),
        }
    }

    pub fn evaluate_ast(
        &self,
        ast: &AST,
    ) -> Result<(), Box<EvalAltResult>> {
        self.engine.eval_ast::<()> (ast)
    }

    pub fn include_script(
        &mut self,
        script: String,
    ) -> Result<(), ParseError> {
        let ast = self.compile_script_to_ast(&script)?;
        self.compiled_scripts.push(ast);
        Ok(())
    }

    fn compile_script_to_ast(
        &self,
        script: &str,
    ) -> Result<AST, ParseError> {
        self.engine.compile(script)
    }
}