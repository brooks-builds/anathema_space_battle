use crate::app::AppMessage;
use anathema::{component::Emitter, store::slab::Key};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{
    io::{BufRead, BufReader, Read},
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
}

pub fn get_lobby_sse(widget_id: Key, game_id: &str, emitter: Emitter) {
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
        }
    });
}

#[derive(Debug, Deserialize)]
pub struct LobbyStream {
    pub players: Vec<PlayerResponse>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerResponse {
    pub name: String,
    pub id: String,
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
