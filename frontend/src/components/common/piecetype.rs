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
            Board => "board",
            Covered => "covered",
            Reserve => "reserve",
            Spawn => "spawn",
            Inactive => "inactive",
        }.to_owned()
    }
}
