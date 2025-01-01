use std::collections::HashSet;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use crate::maze::Maze;

pub fn generate_svg(maze: &Maze) {
    let scale = 100;
    let stroke_width = scale / 10;

    let y_size: i32 = maze.height as i32 * scale;
    let x_size: i32 = maze.width as i32 * scale;

    let outer_border_data = Data::new()
        .move_to((scale, scale))
        .line_by((x_size, 0))
        .line_by((0, y_size))
        .line_by((-x_size, 0))
        .line_by((0, -y_size))
        .close();

    let outer_border_path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", stroke_width)
        .set("d", outer_border_data);

    let mut cell_walls = HashSet::new();

    for cell in &maze.grid {
        let x = cell.x as i32 * scale + scale;
        let y = cell.y as i32 * scale + scale;

        if cell.walls[0] {
            cell_walls.insert(((x - stroke_width / 2, y), (x + scale + stroke_width / 2, y)));
        }
        if cell.walls[1] {
            cell_walls.insert(((x + scale, y - stroke_width / 2), (x + scale, y + scale + stroke_width / 2)));
        }
    }

    let mut cell_wall_data = Data::new();
    for ((x1, y1),(x2, y2)) in cell_walls {
        cell_wall_data = cell_wall_data.move_to((x1, y1))
            .line_to((x2, y2));
    }
    let cell_wall_path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", stroke_width)
        .set("d", cell_wall_data);

    let document = Document::new()
        .set("viewBox", (0, 0, x_size + scale * 2, y_size + scale * 2))
        .add(outer_border_path)
        .add(cell_wall_path);

    svg::save("test.svg", &document).unwrap()
}