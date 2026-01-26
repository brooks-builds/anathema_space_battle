use anathema::{default_widgets::Canvas, state::Color, widgets::Style};

#[derive(Debug, Default)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub cell_width: i32,
}

impl World {
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.cell_width = 2;
        self.width = width;
        self.height = height;
    }

    pub fn print_grid(&self, canvas: &mut Canvas) {
        let character = ' ';
        let mut styles = [Style::new(), Style::new()];

        styles[0].set_bg(anathema::state::Color::Black);
        styles[1].set_bg(Color::Rgb(25, 25, 25));

        let mut styles_iter = styles.iter().cycle();

        for row_index in 0..self.height {
            for col_index in 0..self.width {
                let position = (col_index, row_index);
                let style = styles_iter.next().unwrap();

                canvas.put(character, *style, position);
            }

            styles_iter.next();
        }
    }
}
