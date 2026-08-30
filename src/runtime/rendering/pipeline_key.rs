/// Identifies a compiled render pipeline. Add variants here for each new shader.
#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, PartialEq, Eq, Hash)]
pub enum PipelineKey {
    /// Standard alpha-blended sprite shader.
    Sprite,
    /// Additive blending good for particles, glows, fire.
    Additive,
    /// Screen space ui
    GuiShape,
    GuiText,
    // Custom/user-registered pipeline identified by an arbitrary string.
    //Custom(String)
}
impl Default for PipelineKey {
    fn default() -> Self {
        Self::Sprite
    }
}