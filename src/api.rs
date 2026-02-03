use crate::app::AppMessage;
use anathema::{component::Emitter, store::slab::Key};
use color_eyre::owo_colors::OwoColorize;
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
