extern crate rand;
use std::path::Path;
use rand::seq::SliceRandom;

#[derive(Clone, PartialEq)]
enum Cell {
    Blocked,
    Free,
}

fn main() {
    let size = (10, 10);
    let maze = create_maze(size);
    let mut buf = image::RgbImage::new(size.0 as u32, size.1 as u32);
    for y in 0..size.1 {
        for x in 0..size.0 {
            let color = match maze[x][y] {
                Cell::Free => 255,
                Cell::Blocked => 0,
            };
            buf.put_pixel(x as u32,y as u32,image::Rgb([color, color, color]));
        }
    }
    buf.save(Path::new("maze.png")).expect("Could not save image");
}

fn create_maze(size: (usize, usize)) -> Vec<Vec<Cell>> {
    let mut cells = vec![vec![Cell::Blocked; size.1]; size.0];
    let mut stack = Vec::<(isize, isize)>::new();

    let mut dirs: [(isize, isize); 4] = [(2, 0), (-2, 0), (0, 2), (0, -2)];
    let mut rng = rand::thread_rng();
    'o: while let Some(&(x, y)) = stack.last() {
        dirs.shuffle(&mut rng);
        for i in 0..4 {
            let (dx, dy) = dirs[i];
            let (nx, ny) = (x+dx, y+dy);
            if nx < 0 || ny < 0 || nx >= (size.0 as isize) || ny >= (size.1 as isize) { continue; }
            if cells[nx as usize][ny as usize] != Cell::Free {
                stack.push((nx, ny));
                cells[nx as usize][ny as usize] = Cell::Free;
                cells[(x+dx/2) as usize][(y+dy/2) as usize] = Cell::Free;
                continue 'o;
            }
        }
        stack.pop();
    }

    cells[0][1] = Cell::Free;
    cells[size.0-1][size.1-2] = Cell::Free;
    return cells;
}

