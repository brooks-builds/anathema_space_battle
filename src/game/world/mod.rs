mod vector;

use std::str::FromStr;

use crate::{api::GameStreamPlayer, game::world::vector::Vector};
use anathema::{default_widgets::Canvas, state::Color, widgets::Style};

#[derive(Debug, Default)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub cell_width: i32,
    pub player_characters: Vec<char>,
    pub player_positions: Vec<Vector>,
    pub player_styles: Vec<Style>,
    pub player_ids: Vec<String>,
    pub player_names: Vec<String>,
    pub player_max_speeds: Vec<i32>,
    pub players_ready: Vec<bool>,
    pub player_ship_classnames: Vec<String>,
    pub host_id: String,
}

impl World {
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.cell_width = 2;
        self.width = width;
        self.height = height;
    }

    pub fn add_player(&mut self, player: GameStreamPlayer) {
        let position = Vector::new(player.position_x, player.position_y);
        let mut style = Style::new();

        style.set_fg(Color::from_str(&player.color).unwrap());

        self.player_characters.push(player.ship);
        self.player_positions.push(position);
        self.player_styles.push(style);
        self.player_ids.push(player.id);
        self.player_names.push(player.name);
        self.player_max_speeds.push(player.ship_max_speed);
        self.players_ready.push(player.ready);
        self.player_ship_classnames.push(player.ship_classname);
    }

    pub fn no_players(&self) -> bool {
        self.player_characters.is_empty()
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for (index, player_character) in self.player_characters.iter().enumerate() {
            let player_position = &self.player_positions[index];
            let style = self.player_styles[index];

            canvas.put(
                *player_character,
                style,
                (player_position.x * self.cell_width, player_position.y),
            );
        }
    }
}
