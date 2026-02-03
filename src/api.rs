use crate::app::AppMessage;
use anathema::{component::Emitter, store::slab::Key};
use serde::Deserialize;
use std::thread;

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
