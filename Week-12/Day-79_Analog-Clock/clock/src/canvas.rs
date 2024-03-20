pub struct Canvas {
    pub cols: usize,
    pub lines: usize,
    fill_char: char,
    canvas: Vec<Vec<char>>
}

impl Canvas {
    pub fn new(cols:usize, lines:usize, fill_char:char) -> Self {
        let mut result = Self{ cols, lines, fill_char, canvas: vec![]};
        result.clear();
        result
    }

    pub fn clear(&mut self) {
        self.canvas = vec![];
        for _ in 0..self.lines {
            self.canvas.push(std::iter::repeat(self.fill_char).take(self.cols).collect::<Vec<char>>());
        }
    }

    pub fn print_out(&self) {
        print!("{}", self.get_canvas_as_str());
    }

    pub fn add_line(&mut self, x0: isize, y0: isize, x1: isize, y1:isize, fill_char: char) {
        let (mut x0, mut y0, mut x1, mut y1) = (x0, y0, x1, y1);
        if x0 > x1 {
            (x1, x0) = (x0, x1);
            (y1, y0) = (y0, y1);
        }
        let (dx,dy) = (x1 - x0, y1 - y0);
        if dx == 0 && dy == 0 {
            if self.check_coord_in_range(x0, y0) { self.canvas[y0 as usize][x0 as usize] = fill_char }
            return
        }

        if isize::abs((dx)) >= isize::abs(dy) {
            for x in x0..x1+1 {
                let y = if dx == 0 { y0 } else { y0 + f32::round((x as f32 - x0 as f32) * dy as f32 / dx as f32) as isize};
                if self.check_coord_in_range(x, y) {
                    self.canvas[y as usize][x as usize] = fill_char
                }
            }
        } else {
            if y0 < y1 {
                for y in y0..y1+1 {
                    let x = if dy == 0 { x0 } else { x0 + f32::round((y as f32 - y0 as f32)* dx as f32 / dy as f32) as isize};
                    if self.check_coord_in_range(x, y) {
                        self.canvas[y as usize][x as usize] = fill_char
                    }
                }
            } else {
                for y in y1..y0+1 {
                    let x = if dy == 0 { x0 } else { x1 + f32::round((y as f32 - y1 as f32)* dx as f32 / dy as f32) as isize};
                    if self.check_coord_in_range(x, y) {
                        self.canvas[y as usize][x as usize] = fill_char
                    }
                }
            }
        }
    }

    pub fn add_text(&mut self, x: isize, y:isize, text: &str) {
        text.chars().enumerate().for_each(|(index, ch)| {
            if self.check_coord_in_range(x + index as isize, y) {
                self.canvas[y as usize][(x as usize)+index] = ch
            }
        })
    }

    pub fn add_rect(&mut self, x: isize, y:isize, w:isize, h:isize, fill_char: char, outline_char: char) {
        for px in x..x+w {
            for py in y..y+h {
                if self.check_coord_in_range(px, py) {
                    if px == x || px == x + w - 1 || py == y || py == y + h - 1 {
                        self.canvas[py as usize][px as usize] = outline_char
                    } else {
                        self.canvas[py as usize][px as usize] = fill_char
                    }
                }
            }
        }
    }

    fn get_canvas_as_str(&self) -> String {
        self.canvas.iter().fold("".to_string(),|mut result, row| {
            result += (row.iter().map(|ch| ch.to_string()).collect::<Vec<String>>().join("")+"\n").as_str();
            result
        })
    }

    fn check_coord_in_range(&self, x: isize, y: isize) -> bool {
        x >=0 && (x as usize) < self.cols && y>=0 && (y as usize) < self.lines
    }
}
