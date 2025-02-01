#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Cell {
    state: CellState,
    walls: [bool; 4],
    coord: Coord,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Coord {
    pub y: usize,
    pub x: usize
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum CellState {
    Unvisited,
    Frontier,
    Path,
    Solution,
    Start,
    Finish,
}

impl Cell {
    pub fn new(coord: Coord) -> Self {
        Self {
            state: CellState::Unvisited,
            walls: [true, true, true, true],
            coord,
        }
    }

    pub fn walls(&self) -> &[bool; 4] {
        &self.walls
    }

    pub fn coord(&self) -> &Coord {
        &self.coord
    }

    pub fn visited(&self) -> bool {
        match self.state {
            CellState::Path => true,
            _ => false
        }
    }

    pub fn visit(&mut self) {
        self.state = CellState::Path;
    }

    pub fn change_state(&mut self, state: CellState) {
        self.state = state;
    }

    pub fn state(&self) -> CellState {
        self.state
    }

    fn remove_wall_by_int(&mut self, dir: usize) {
        self.walls[dir] = false;
    }

    pub fn remove_wall(&mut self, dir: &str) {
        match dir {
            "top" => self.remove_wall_by_int(0),
            "right" => self.remove_wall_by_int(1),
            "bottom" => self.remove_wall_by_int(2),
            "left" => self.remove_wall_by_int(3),
            _ => {},
        }
    }
}
