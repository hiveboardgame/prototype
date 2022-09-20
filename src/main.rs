use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Color {
    Black,
    White,
}

impl Color {
    fn opposite(&self) -> Color {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Color::White => "w",
            Color::Black => "b",
        };
        write!(f, "{}", color)
    }
}

// https://entomology.gitlab.io/notation.html
// Each piece used during a game has a unique name. This name is made of one lowercase letter for
// the color (w for white or b for black) followed by one uppercase letter for the bug type (A for
// Ant, B for Beetle, G for Grasshopper, L for Ladybug, M for Mosquito, P for Pillbug, Q for Queen
// Bee or S for Spider). In case the bug appears multiple times, the piece name also contains one
// digit indicating in which order that piece came into play. For instance, wA1 is the first white
// Ant that has been added to the hive while bB2 is the second black Beetle. Since there is only
// one copy of it, the white Pillbug is simply named wP and not wP1.
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Bug {
    Ant(Color),
    Beetle(Color),
    Grasshopper(Color),
    Ladybug(Color),
    Mosquito(Color),
    Pillbug(Color),
    Queen(Color),
    Spider(Color),
}

impl Bug {
    fn color(&self) -> Color {
        return match self {
            Bug::Ant(c)
            | Bug::Beetle(c)
            | Bug::Grasshopper(c)
            | Bug::Ladybug(c)
            | Bug::Mosquito(c)
            | Bug::Pillbug(c)
            | Bug::Queen(c)
            | Bug::Spider(c) => c.clone(),
        };
    }
}

impl fmt::Display for Bug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (c, p) = match self {
            Bug::Ant(c) => (c, "A"),
            Bug::Beetle(c) => (c, "B"),
            Bug::Grasshopper(c) => (c, "G"),
            Bug::Ladybug(c) => (c, "L"),
            Bug::Mosquito(c) => (c, "M"),
            Bug::Pillbug(c) => (c, "P"),
            Bug::Queen(c) => (c, "Q"),
            Bug::Spider(c) => (c, "S"),
        };
        write!(f, "{}{}", c, p)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Piece {
    bug: Bug,
    order: i8,
}

impl Piece {
    fn new(bug: Bug, order: i8) -> Piece {
        Piece { bug, order }
    }

    fn color(&self) -> Color {
        self.bug.color()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.bug, self.order)
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.bug, self.order)
    }
}

impl Bug {
    fn all(color: Color) -> HashMap<Bug, i8> {
        let mut bugs = HashMap::new();
        bugs.insert(Bug::Ant(color), 3);
        bugs.insert(Bug::Beetle(color), 2);
        bugs.insert(Bug::Grasshopper(color), 3);
        bugs.insert(Bug::Ladybug(color), 1);
        bugs.insert(Bug::Mosquito(color), 1);
        bugs.insert(Bug::Pillbug(color), 1);
        bugs.insert(Bug::Queen(color), 1);
        bugs.insert(Bug::Spider(color), 3);
        return bugs;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i8, i8);

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.0, self.1)
    }
}

struct Board {
    board: HashMap<Position, Vec<Piece>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut positions = self.board.keys().cloned().collect::<Vec<Position>>();
        positions.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        let min_x = positions
            .iter()
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap_or(&Position(0, 0))
            .0;
        let max_x = positions
            .iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap_or(&Position(0, 0))
            .0;
        let min_y = positions
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or(&Position(0, 0))
            .1;
        let max_y = positions
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap_or(&Position(0, 0))
            .1;
        let mut s = "".to_string();
        for y in min_y..=max_y {
            if y.rem_euclid(2) == 1 {
                write!(s, "{}", "  ")?;
            }
            for x in min_x..=max_x {
                match self.board.get(&Position(x, y)) {
                    Some(piece) => write!(s, "{} ", piece.last().unwrap())?,
                    None => write!(s, "{}", "    ")?,
                };
            }
            write!(s, "{}", "\n")?;
        }
        write!(f, "{}", s)
    }
}

impl Board {
    fn new() -> Board {
        Board {
            board: HashMap::new(),
        }
    }

    fn neighbor_positions(&self, position: &Position) -> Vec<Position> {
        return vec![
            Position(position.0 - 1, position.1 - 1), // North West
            Position(position.0, position.1 - 1),     // North East
            Position(position.0 + 1, position.1),     // East
            Position(position.0, position.1 + 1),     // South East
            Position(position.0 - 1, position.1 + 1), // South West
            Position(position.0 - 1, position.1),     // West
        ];
    }

    fn neighbors(&self, position: &Position) -> Vec<Vec<Piece>> {
        return self
            .neighbor_positions(&position)
            .iter()
            .filter_map(|pos| self.board.get(&pos))
            .cloned()
            .collect();
    }

    fn top_layer_neighbors(&self, position: &Position) -> Vec<Piece> {
        return self
            .neighbor_positions(&position)
            .iter()
            .filter_map(|pos| self.board.get(&pos).and_then(|v| v.last()))
            .cloned()
            .collect();
    }

    fn negative_space(&self) -> Vec<Position> {
        let taken = self.board.keys().cloned().collect::<HashSet<Position>>();
        let mut all_neighbors = HashSet::new();
        for pos in taken.iter() {
            for pos in self.neighbor_positions(pos) {
                all_neighbors.insert(pos);
            }
        }
        all_neighbors
            .difference(&taken)
            .into_iter()
            .cloned()
            .collect()
    }

    fn spawnable(&self, color: Color, position: &Position) -> bool {
        !self
            .top_layer_neighbors(position)
            .iter()
            .map(|piece| piece.color())
            .collect::<Vec<Color>>()
            .contains(&color.opposite())
    }

    fn spawn(&mut self, position: &Position, bug: Bug, order: i8) {
        let piece = Piece::new(bug, order);
        self.board
            .entry(position.clone())
            .and_modify(|v| v.push(piece.clone()))
            .or_insert(vec![piece]);
    }
}

struct State {
    board: Board,
    player: (Player, Player),
    history: Vec<String>,
}

impl State {
    fn new() -> State {
        return State {
            board: Board::new(),
            player: (Player::new(Color::Black), Player::new(Color::White)),
            history: Vec::new(),
        };
    }
}

struct Player {
    color: Color,
    bugs: HashMap<Bug, i8>,
}

impl Player {
    fn new(color: Color) -> Player {
        return Player {
            color: color.clone(),
            bugs: Bug::all(color),
        };
    }
}

fn main() {
    let mut state = State::new();
    println!(
        "{}",
        state
            .player
            .0
            .bugs
            .get(&Bug::Ant(state.player.0.color))
            .unwrap()
    );
    state
        .board
        .spawn(&Position(0, 0), Bug::Queen(Color::Black), 1);
    println!("{}", state.board);
    for (i, pos) in state
        .board
        .neighbor_positions(&Position(0, 0))
        .iter()
        .enumerate()
    {
        state.board.spawn(pos, Bug::Ant(Color::Black), i as i8);
        println!("{}", state.board);
    }
}

#[cfg(test)]
mod tests {
    use crate::Bug;
    use crate::Color;
    use crate::Position;
    use crate::State;

    #[test]
    fn spawn() {
        let mut state = State::new();
        state
            .board
            .spawn(&Position(0, 0), Bug::Ant(Color::Black), 1);
        let result = state.board.spawnable(Color::White, &Position(1, 0));
        assert!(!result);
    }
}
