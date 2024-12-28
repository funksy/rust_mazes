#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Cell {
    pub visited: bool,
    pub walls: [bool; 4],
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Cell {
            visited: false,
            walls: [true, true, true, true],
            x,
            y,
        }
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn remove_wall(&mut self, dir: usize) {
        self.walls[dir] = false;
    }
}
