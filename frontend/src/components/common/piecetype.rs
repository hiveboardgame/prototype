#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PieceType {
    Board,
    Covered,
    Reserve,
    Spawn,
    Inactive,
}

impl PieceType {
    pub fn to_string(&self) -> String {
        match self {
            PieceType::Board => "board",
            PieceType::Covered => "covered",
            PieceType::Inactive => "inactive",
            PieceType::Reserve => "reserve",
            PieceType::Spawn => "spawn",
        }
        .to_owned()
    }
}
