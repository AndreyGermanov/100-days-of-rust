mod canvas;

use std::ops::Div;
use canvas::Canvas;
use chrono::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        draw_clock(70, 40);
        thread::sleep(Duration::from_secs(1));
    }
}

fn draw_clock(cols: usize, lines:usize) {
    let second_hand_char = '.';
    let minute_hand_char = 'o';
    let hour_hand_char= 'O';
    let mark_char = '`';
    let mut ascii_canvas = Canvas::new(cols,lines, ' ');
    let center_y = lines.div_ceil(2) as isize;
    let radius = center_y - 5;
    let second_hand_length = (radius as f32 / 1.17) as isize;
    let minute_hand_length = (radius as f32 / 1.25) as isize;
    let hour_hand_length = (radius as f32 / 1.95) as isize;
    ascii_canvas.add_rect(5, 3, cols.div(2) as isize * 2 - 9, lines.div(2) as isize * 2 - 5, ' ', 'o');
    draw_clock_face(&mut ascii_canvas, radius, mark_char);
    let dt = Local::now();
    let (hour, minute, second) = (dt.hour() as isize, dt.minute() as isize, dt.second() as isize);
    draw_hand(&mut ascii_canvas, second, second_hand_length, second_hand_char, None);
    draw_hand(&mut ascii_canvas, minute, minute_hand_length, minute_hand_char, None);
    draw_hand(&mut ascii_canvas, hour, hour_hand_length, hour_hand_char, Some(minute));
    ascii_canvas.print_out();
}

fn draw_hand(ascii_canvas:&mut Canvas, value: isize, length: isize, fill_char: char,value2: Option<isize> ) {
    let mut value = value;
    let mut mult = 6.0;
    let x0 = f32::ceil(ascii_canvas.cols as f32 / 2.0) as isize;
    let y0 = f32::ceil(ascii_canvas.lines as f32 / 2.0) as isize;
    if value2.is_some() {
        value = value + (value2.unwrap() as f32 / 60.0) as isize;
        mult = 30.0;
    }
    let x1 = x0 + (f32::cos((value as f32 + 45.0) * mult * PI / 180.0) * length as f32 * 1.75) as isize;
    let y1 = y0 + (f32::sin((value as f32 + 45.0) * mult * PI / 180.0) * length as f32) as isize;

    ascii_canvas.add_line(x0, y0, x1, y1,fill_char);
}

fn draw_clock_face(ascii_canvas:&mut Canvas, radius: isize, mark_char: char) {
    let x0 = ascii_canvas.cols.div_ceil(2) as isize;
    let y0 = ascii_canvas.lines.div_ceil(2) as isize;
    for mark in 1..12*5+1 {
        let x1 = x0 + (f32::cos((mark as f32 + 45.0) * 6.0 * PI / 180.0) * radius as f32 * 1.75) as isize;
        let y1 = y0 + (f32::sin((mark as f32 + 45.0) * 6.0 * PI / 180.0) * radius as f32) as isize;
        if mark % 5 != 0 {
            ascii_canvas.add_text(x1, y1, mark_char.to_string().as_str());
        }
    }
    for mark in 1..12+1 {
        let x1 = x0 + (f32::cos((mark as f32 + 45.0) * 30.0 * PI / 180.0) * radius as f32 * 1.75) as isize;
        let y1 = y0 + (f32::sin((mark as f32 + 45.0) * 30.0 * PI / 180.0) * radius as f32) as isize;
        ascii_canvas.add_text(x1, y1, mark.to_string().as_str());
    }
}
