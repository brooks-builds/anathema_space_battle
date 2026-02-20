pub mod vector;

use crate::{
    api::{DBPlayer, GameStream, GameStreamPlayer},
    game::world::vector::Vector,
};
use anathema::{default_widgets::Canvas, geometry::Pos, state::Color, widgets::Style};
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Default, Serialize)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub player_characters: Vec<char>,
    pub player_positions: Vec<Vector>,
    #[serde(skip)]
    pub player_styles: Vec<Style>,
    pub player_ids: Vec<String>,
    pub player_names: Vec<String>,
    pub player_max_speeds: Vec<i32>,
    pub players_ready: Vec<bool>,
    pub player_ship_classnames: Vec<String>,
    pub host_id: String,
    pub turn: i32,
    pub player_speed: i32,
    pub possible_destinations: Vec<Vector>,
    pub player_id: String,
    pub display_possible_destinations: bool,
    pub setting_torpedo_target: bool,
    pub mouse_position: Vector,
}

impl World {
    pub fn set_size(&mut self, width: i32, height: i32) {
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
        if self.display_possible_destinations {
            for Vector { x, y } in &self.possible_destinations {
                let character = ' ';
                let mut style = Style::new();

                style.set_bg(Color::DarkGrey);
                canvas.put(character, style, (*x, *y));
            }
        }

        for (index, player_character) in self.player_characters.iter().enumerate() {
            let player_position = &self.player_positions[index];
            let style = self.player_styles[index];

            canvas.put(
                *player_character,
                style,
                (player_position.x, player_position.y),
            );
        }

        if self.setting_torpedo_target {
            let character = ' ';
            let mut style = Style::new();
            let position = canvas.translate(Pos::new(self.mouse_position.x, self.mouse_position.y));

            style.set_bg(Color::LightRed);

            canvas.put(character, style, position);
        }
    }

    pub fn update_players_ready(&mut self, players: Vec<GameStreamPlayer>) {
        for player in players {
            let Some(player_index) = self.find_player_index(&player.id) else {
                continue;
            };

            self.players_ready[player_index] = player.ready;
        }
    }

    fn find_player_index(&self, player_id: &str) -> Option<usize> {
        for (index, id) in self.player_ids.iter().enumerate() {
            if player_id == id {
                return Some(index);
            }
        }

        None
    }

    pub fn player_updated(&mut self, player: DBPlayer) {
        self.player_speed = player.speed;
        self.player_id = player.id;

        self.calculate_legal_destinations();
    }

    fn calculate_legal_destinations(&mut self) -> Option<()> {
        let index = self.find_player_index(&self.player_id)?;
        let position = &self.player_positions[index];
        let speed = self.player_speed;

        self.possible_destinations.clear();

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = Vector::new(x, y);
                let distance = position.distance_to(&cell);

                if distance == speed {
                    self.possible_destinations.push(cell);
                }
            }
        }

        None
    }

    pub fn update_after_turn(&mut self, game: &GameStream) {
        for player in game.players.iter() {
            let Some(index) = self.find_player_index(&player.id) else {
                continue;
            };

            self.player_positions[index] = Vector::new(player.position_x, player.position_y);
        }
    }
}
