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
