use crate::{app::AppMessage, game::TurnCommand};
use anathema::{component::Emitter, store::slab::Key};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read},
    sync::mpsc::Receiver,
    thread,
};

const BASE_API_URL: &str = "http://localhost:3000";

pub fn create_game(app_key: Key, player_name: String, emitter: Emitter) {
    thread::spawn(move || {
        let api_url = format!("{BASE_API_URL}/api/games");
        let data = serde_json::json!({
            "player_name": player_name
        });
        let client = reqwest::blocking::Client::new();
        let result = client.post(api_url).json(&data).send().unwrap();
        let created_game = result.json::<CreateGameResponse>().unwrap();

        emitter
            .try_emit(app_key, AppMessage::GameCreated(created_game))
            .unwrap();
    });
}

#[derive(Debug, Deserialize)]
pub struct CreateGameResponse {
    pub game_id: String,
    pub status: String,
    pub player_id: String,
    pub token: String,
    pub game_code: i32,
}

pub fn join_game(widget_id: Key, player_name: String, emitter: Emitter, game_code: i32) {
    thread::spawn(move || {
        let api_url = format!("{BASE_API_URL}/api/games/join");
        let data = serde_json::json!({
            "player_name": player_name,
            "code": game_code
        });
        let client = Client::new();
        let joined_game = client
            .post(api_url)
            .json(&data)
            .send()
            .unwrap()
            .json::<JoinGameResponse>()
            .unwrap();
        let message = AppMessage::GameJoined(joined_game);

        emitter.try_emit(widget_id, message).unwrap();
    });
}

#[derive(Debug, Deserialize)]
pub struct JoinGameResponse {
    pub token: String,
    pub game_id: String,
    pub player_id: String,
}

pub fn get_lobby_sse(
    widget_id: Key,
    game_id: &str,
    emitter: Emitter,
    end_connection: Receiver<()>,
) {
    let url = format!("{BASE_API_URL}/api/games/{game_id}/lobby/stream");

    thread::spawn(move || {
        let client = Client::new();
        let stream = client.get(url).send().unwrap();
        let mut stream_reader = BufReader::new(stream);

        loop {
            // stripping the leading data from the line. This is added by axum Event.
            let mut header = [0u8; 6];
            stream_reader.read_exact(&mut header).unwrap();

            let mut line = String::new();
            stream_reader.read_line(&mut line).unwrap();

            let lobby_data = serde_json::from_str::<LobbyStream>(&line).unwrap();
            let message = AppMessage::LobbyUpdate(lobby_data);
            emitter.try_emit(widget_id, message).unwrap();

            if let Ok(()) = end_connection.try_recv() {
                break;
            }
        }
    });
}

#[derive(Debug, Deserialize)]
pub struct LobbyStream {
    pub players: Vec<PlayerResponse>,
    pub game_status: GameStatus,
}

#[derive(Debug, Deserialize)]
pub struct PlayerResponse {
    pub name: String,
    pub ship_class: String,
    pub ship_character: char,
    pub ship_color: String,
    pub ready: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ShipColor {
    pub id: String,
    pub name: String,
}

pub fn get_possible_colors(widget_id: Key, emitter: Emitter) {
    thread::spawn(move || {
        let url = format!("{BASE_API_URL}/api/players/colors");
        let client = Client::new();
        let ship_colors = client
            .get(url)
            .send()
            .unwrap()
            .json::<Vec<ShipColor>>()
            .unwrap();
        let message = AppMessage::PossibleShipColors(ship_colors);

        emitter.try_emit(widget_id, message).unwrap();
    });
}

pub fn set_ship_color(token: String, color_id: &str) {
    let body = serde_json::json!({
        "color_id": color_id
    });

    thread::spawn(move || {
        let client = Client::new();
        let url = format!("{BASE_API_URL}/api/players/colors");

        client
            .put(url)
            .header("token", token)
            .json(&body)
            .send()
            .unwrap();
    });
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ship {
    pub id: String,
    pub class_name: String,
    pub character: char,
    pub max_speed: i32,
    pub max_torpedo_count: i32,
}

pub fn get_possible_ships(widget_id: Key, emitter: Emitter) {
    thread::spawn(move || {
        let client = Client::new();
        let url = format!("{BASE_API_URL}/api/ships");
        let possible_ships = client.get(url).send().unwrap().json::<Vec<Ship>>().unwrap();
        let message = AppMessage::PossibleShips(possible_ships);

        emitter.try_emit(widget_id, message).ok();
    });
}

pub fn change_ship(ship_id: &str, token: String) {
    let url = format!("{BASE_API_URL}/api/players/ship/{ship_id}");

    thread::spawn(move || {
        let client = Client::new();

        client.put(url).header("token", token).send().unwrap();
    });
}

pub fn ready_up(token: String) {
    thread::spawn(move || {
        let client = Client::new();
        let url = format!("{BASE_API_URL}/api/players/ready_up");

        client.put(url).header("token", token).send().unwrap();
    });
}

pub fn quit(token: String, widget_id: Key, emitter: Emitter) {
    thread::spawn(move || {
        let client = Client::new();
        let url = format!("{BASE_API_URL}/api/players");
        let message = AppMessage::Quit;

        client.delete(url).header("token", token).send().unwrap();
        emitter.try_emit(widget_id, message).ok();
    });
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
pub enum GameStatus {
    #[default]
    Lobby,
    Playing,
    GameOver,
}

#[derive(Debug, Deserialize)]
pub struct GameStream {
    pub game: GameStreamGame,
    pub players: Vec<GameStreamPlayer>,
}

#[derive(Debug, Deserialize)]
pub struct GameStreamGame {
    pub id: String,
    pub host_id: String,
    pub width: i32,
    pub height: i32,
    pub turn_number: i32,
}

#[derive(Debug, Deserialize)]
pub struct GameStreamPlayer {
    pub id: String,
    pub name: String,
    pub ship: char,
    pub ship_max_speed: i32,
    pub color: String,
    pub ready: bool,
    pub position_x: i32,
    pub position_y: i32,
    pub ship_classname: String,
}

pub fn get_game_sse(widget_id: Key, game_id: &str, emitter: Emitter, end_connection: Receiver<()>) {
    let url = format!("{BASE_API_URL}/api/games/{game_id}/stream");

    thread::spawn(move || {
        let client = Client::new();
        let stream = client.get(url).send().unwrap();
        let mut stream_reader = BufReader::new(stream);

        loop {
            // stripping the leading data from the line. This is added by axum Event.
            let mut header = [0u8; 6];
            stream_reader.read_exact(&mut header).unwrap();

            let mut line = String::new();
            stream_reader.read_line(&mut line).unwrap();

            let game_data = serde_json::from_str::<GameStream>(&line).unwrap();
            let message = AppMessage::GameUpdate(game_data);
            emitter.try_emit(widget_id, message).unwrap();

            if let Ok(()) = end_connection.try_recv() {
                break;
            }
        }
    });
}

pub fn get_player(widget_id: Key, emitter: Emitter, player_token: String) {
    thread::spawn(move || {
        if player_token.is_empty() {
            eprintln!("Missing player token");
            return;
        }

        let url = format!("{BASE_API_URL}/api/players");
        let client = Client::new();
        match client
            .get(url)
            .header("token", player_token)
            .send()
            .unwrap()
            .json::<DBPlayer>()
        {
            Ok(player) => {
                emitter
                    .try_emit(widget_id, AppMessage::GotPlayerFromServer(player))
                    .ok();
            }
            Err(error) => {
                eprintln!("{error:?}");
            }
        };
    });
}

#[derive(Debug, Deserialize)]
pub struct DBPlayer {
    pub speed: i32,
    pub id: String,
    pub torpedo_count: i32,
}

pub fn submit_game_turn(
    widget_id: Key,
    emitter: Emitter,
    token: &str,
    game_id: &str,
    turn_command: TurnCommand,
) {
    let url = format!("{BASE_API_URL}/api/games/{game_id}/command");
    let token = token.to_owned();

    thread::spawn(move || {
        let client = Client::new();
        let body = SubmitTurnCommandBody {
            speed_change: turn_command.speed_change,
            destination: turn_command
                .destination
                .map(|destination| (destination.x, destination.y)),
        };
        let response = client
            .post(url)
            .header("token", token)
            .json(&body)
            .send()
            .unwrap();

        if response.status().is_success() {
            emitter
                .try_emit(widget_id, AppMessage::GameTurnSubmitted)
                .ok();
        }
    });
}

#[derive(Debug, Serialize)]
pub struct SubmitTurnCommandBody {
    pub speed_change: i8,
    pub destination: Option<(i32, i32)>,
}
