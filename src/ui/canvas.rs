use crate::{primitives::textures::Texture, render::render_engine::RenderEngine, utils::resources::Handle};

#[derive(Debug)]
pub struct UICanvas {
    pub blank_texture: Handle<Texture>
}

impl UICanvas {
    /// Creates new canvas
    /// 
    /// Arguments:
    /// * engine: &RenderEngine - The rener engine to create the canvas for.
    /// 
    /// Returns a instance of `UICanvas`
    pub fn new(engine: &mut RenderEngine) -> Self {
        let blank_texture = engine.create_texture("ui.blank", include_bytes!("empty.png"));
        Self { blank_texture }
    }
}