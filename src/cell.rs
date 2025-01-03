#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Cell {
    visited: bool,
    walls: [bool; 4],
    coord: Coord,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Coord {
    pub y: usize,
    pub x: usize
}

impl Cell {
    pub fn new(coord: Coord) -> Self {
        Self {
            visited: false,
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
        self.visited
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
