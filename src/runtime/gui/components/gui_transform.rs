use crate::runtime::math::Float2;
use crate::Component;
use crate::app::WINDOW_SIZE;

pub enum GuiAnchor {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    Middle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

#[derive(Component!)]
pub struct GuiTransform {
    pub position: Float2,
    pub size: Float2,
}

impl GuiTransform {
    pub fn from_position(position: Float2, size: Float2) -> Self {
        Self { position, size }
    }

    pub fn at_anchor(anchor: GuiAnchor, size: Float2) -> Self {
        // Normalized anchor point in [0, 1] space relative to the window.
        let anchor_point = match anchor {
            GuiAnchor::TopLeft => Float2::new(0.0, 0.0),
            GuiAnchor::TopMiddle => Float2::new(0.5, 0.0),
            GuiAnchor::TopRight => Float2::new(1.0, 0.0),
            GuiAnchor::MiddleLeft => Float2::new(0.0, 0.5),
            GuiAnchor::Middle => Float2::new(0.5, 0.5),
            GuiAnchor::MiddleRight => Float2::new(1.0, 0.5),
            GuiAnchor::BottomLeft => Float2::new(0.0, 1.0),
            GuiAnchor::BottomMiddle => Float2::new(0.5, 1.0),
            GuiAnchor::BottomRight => Float2::new(1.0, 1.0),
        };

        // Point on screen this anchor refers to.
        let anchor_screen_pos = Float2::new(
            anchor_point.x * WINDOW_SIZE.0 as f32,
            anchor_point.y * WINDOW_SIZE.1 as f32,
        );

        // Offset by half the size so the transform is centered on the anchor point.
        let adjusted_pos = anchor_point * size;
        let position = Float2::new(
            anchor_screen_pos.x - adjusted_pos.x,
            anchor_screen_pos.y - adjusted_pos.y,
        );
        Self { position, size }
    }
}