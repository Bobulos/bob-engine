#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
/// Color values are stored on a 0-1 scale internally
pub struct Color {
    pub value: [f32; 4],
}

impl Color {
    /// From normalized 0.0-1.0 values
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { value: [r, g, b, a] }
    }

    /// From 0-255 rgba values
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            value: [
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ],
        }
    }

    /// From 0-255 rgb values, fully opaque
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba(r, g, b, 255)
    }

    /// From a hex string. Accepts "#RRGGBB", "#RRGGBBAA", "RRGGBB", "RRGGBBAA"
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "invalid hex")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "invalid hex")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "invalid hex")?;
                Ok(Self::from_rgba(r, g, b, 255))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "invalid hex")?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "invalid hex")?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "invalid hex")?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "invalid hex")?;
                Ok(Self::from_rgba(r, g, b, a))
            }
            _ => Err("hex must be RRGGBB or RRGGBBAA"),
        }
    }

    /// From a packed u32 in 0xRRGGBBAA format
    pub fn from_u32(hex: u32) -> Self {
        Self::from_rgba(
            ((hex >> 24) & 0xFF) as u8,
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8)  & 0xFF) as u8,
            (hex         & 0xFF) as u8,
        )
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.value[3] = a;
        self
    }


    pub fn red()     -> Self { Self::from_rgba(255, 0,   0,   255) }
    pub fn green()   -> Self { Self::from_rgba(0,   255, 0,   255) }
    pub fn blue()    -> Self { Self::from_rgba(0,   0,   255, 255) }
    pub fn white()   -> Self { Self::from_rgba(255, 255, 255, 255) }
    pub fn black()   -> Self { Self::from_rgba(0,   0,   0,   255) }
    pub fn yellow()  -> Self { Self::from_rgba(255, 255, 0,   255) }
    pub fn cyan()    -> Self { Self::from_rgba(0,   255, 255, 255) }
    pub fn magenta() -> Self { Self::from_rgba(255, 0,   255, 255) }
    pub fn transparent() -> Self { Self::new(0.0, 0.0, 0.0, 0.0) }
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}