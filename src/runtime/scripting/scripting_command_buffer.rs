use rhai::CustomType;

#[derive(Clone, CustomType)]
pub struct ScriptingCommandBuffer {
    pub commands: Vec<Command>
}

enum CommandType {
}
#[derive(Clone, CustomType)]
struct Command {
    
}