use rand::{Rng, thread_rng};

struct Cell {
    ch: char,
    is_mine: bool
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    mine_count: u32,
    mines_marked: u32
}
impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        let mut result = Grid{ cells:vec![], mine_count:0, mines_marked:0};
        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..cols {
                row.push(Cell{ch:'.',is_mine:false});
            }
            result.cells.push(row);
        }
        let min = ((rows*cols) as f32 * 0.1).round() as u32;
        let max = ((rows*cols) as f32 * 0.2).round() as u32;
        result.mine_count = min + thread_rng().gen_range(0..(max-min+1));
        let mut rm = result.mine_count;
        let mut rng = thread_rng();
        while rm > 0 {
            let (x,y) = (rng.gen_range(0..rows),rng.gen_range(0..cols));
            if !result.cells[x][y].is_mine {
                rm -= 1;
                result.cells[x][y].is_mine = true;
            }
        }
        result.mines_marked = 0;
        result
    }

    fn display(&self) {
        for row in &self.cells {
            for col in row {
                print!("{}",col.ch);
            }
            print!("\n");
        }
    }

    fn count_adj_mines(&self, x: i32, y: i32) -> u32 {
        let mut count = 0;
        for idx1 in y-1..y+2 {
            if idx1 < 0 || idx1 >= self.cells[0].len() as i32 { continue }
            for idx2 in x-1..x+2 {
                if idx2 < 0 || idx2 >= self.cells.len() as i32 { continue }
                if self.cells[idx2 as usize][idx1 as usize].is_mine {
                    count +=1
                }
            }
        }
        count
    }

    fn clear_cell(&mut self, x: i32, y: i32) {
        if x<0 || x>=self.cells.len() as i32 || y<0 || y>=self.cells[0].len() as i32 { return }
        if self.cells[x as usize][y as usize].ch != '.' { return }
        if self.cells[x as usize][y as usize].is_mine {
            self.cells[x as usize][y as usize].ch = '*';
            return
        }
        let count = self.count_adj_mines(x, y);
        if count > 0 {
            self.cells[x as usize][y as usize].ch = std::char::from_u32(48+count).unwrap();
            return
        }
        self.cells[x as usize][y as usize].ch = '0';
        self.clear_cell(x+1, y);
        self.clear_cell(x+1, y+1);
        self.clear_cell(x, y+1);
        self.clear_cell(x-1, y+1);
        self.clear_cell(x-1, y);
        self.clear_cell(x-1, y-1);
        self.clear_cell(x, y-1);
        self.clear_cell(x+1, y-1);
    }

    fn reveal_cells(&mut self) {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                self.clear_cell(x as i32,y as i32)
            }
        }
    }
}

fn main() {
    let mut grid = Grid::new(10,10);
    grid.reveal_cells();
    grid.display()
}

