pub struct Canvas {
    pub canvas: [[char; 50]; 10],
}

impl Canvas {
    pub fn new_canvas() -> Canvas {
        Canvas {
            canvas: [[' '; 50]; 10],
        }
    }

    pub fn set_pixels(&mut self, pixels: &[(usize, usize, u8)]) {
        for &(row, col, c) in pixels {
            if row < 10 && col < 50 {
                self.canvas[row][col] = c as char;
            } else {
                println!("Invalid index!");
            }
        }
    }

    pub fn print_canvas(&self) {
        for row in &self.canvas {
            for &c in row {
                print!("{}", c);
            }
            print!("\n");
        }
    }
}
