use stylist::{style, Style};

pub struct FlatPieceStyle{
}

impl FlatPieceStyle {
    pub fn style() -> Style {
        style!(r#"
            .inactive{
                opacity: 0.5;
            }
        "#
            ).expect("Stylesheet inactive failed")
    }
}
