use std::fmt::Display;


#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    body_background: String,
    font_family: String,
    font_size: String,
    font_color: String,
}


impl Theme {
    pub fn light() -> Self {
        Self {
            body_background: "#ffffff".to_owned(),
            font_color: "#000000".to_owned(),
            ..Default::default()
        }
    }
    pub fn dark() -> Self {
        Self {
            body_background: "#2d2d2d".to_owned(),
            font_color: "#ffffff".to_owned(),
            ..Default::default()
        }
    }
}


impl Default for Theme {
    fn default() -> Self {
        Self {
            body_background: "#000000".to_owned(),
            font_family: "Open Sans".to_owned(),
            font_size: "18px".to_owned(),
            font_color: "#ffffff".to_owned()
        }
    }
}


impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "body_background={}, ", self.body_background)?;
        write!(f, "font_family={}, ", self.font_family)?;
        write!(f, "font_size={}, ", self.font_size)?;
        write!(f, "font_color={}", self.font_color)
    }
}