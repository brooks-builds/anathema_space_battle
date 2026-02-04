use crate::{
    api::{self, CreateGameResponse, JoinGameResponse, LobbyStream},
    pages::lobby::LobbyPage,
    router::Route,
};
use anathema::{
    component::Component,
    state::{List, State, Value},
};
use bb_anathema_components::BBAppComponent;

#[derive(Debug, Default)]
pub struct App(AppData);

impl App {
    pub fn ident() -> &'static str {
        "app"
    }
}

#[derive(Debug, State, Default)]
pub struct AppState {
    width: Value<u16>,
    height: Value<u16>,
    game_status: Value<String>,
    current_route: Value<String>,
    player_name: Value<String>,
    game_code: Value<i32>,
    player_names: Value<List<String>>,
}

#[derive(Debug, Default)]
pub struct AppData {
    game_id: Option<String>,
    player_id: Option<String>,
    token: Option<String>,
}

impl Component for App {
    type State = AppState;

    type Message = AppMessage;

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let viewport = context.viewport.size();

        state.width.set(viewport.width);
        state.height.set(viewport.height);
        state.current_route.set(Route::Home.into());
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            AppMessage::NameSet(name) => {
                state.player_name.set(name);
                state.current_route.set(Route::Start.into());
            }
            AppMessage::CreateGame => {
                let key = context.widget_id;
                let player_name = state.player_name.to_ref().clone();

                api::create_game(key, player_name, context.emitter.clone());
            }
            AppMessage::GameCreated(game_created_data) => {
                state.current_route.set(Route::Lobby.into());
                self.0.game_id = Some(game_created_data.game_id.clone());
                state.game_status.set(game_created_data.status);
                self.0.player_id = Some(game_created_data.player_id);
                self.0.token = Some(game_created_data.token);
                state.game_code.set(game_created_data.game_code);

                api::get_lobby_sse(
                    context.widget_id,
                    &game_created_data.game_id,
                    context.emitter.clone(),
                );
            }
            AppMessage::JoinGame(code) => {
                api::join_game(
                    context.widget_id,
                    state.player_name.to_ref().clone(),
                    context.emitter.clone(),
                    code,
                );
            }
            AppMessage::GameJoined(join_game_response) => {
                self.0.token = Some(join_game_response.token);
                self.0.game_id = Some(join_game_response.game_id.clone());
                state.current_route.set(Route::Lobby.into());

                api::get_lobby_sse(
                    context.widget_id,
                    &join_game_response.game_id,
                    context.emitter.clone(),
                );
            }
            AppMessage::LobbyUpdate(lobby_stream) => {
                let lobby_page_id = context
                    .components
                    .by_name(LobbyPage::ident())
                    .send(AppMessage::LobbyUpdate(lobby_stream));
            }
        }
    }
}

impl BBAppComponent for App {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            Self::ident(),
            "templates/app.aml",
            Self::default(),
            AppState::default(),
        )?;

        Ok(())
    }
}

pub enum AppMessage {
    NameSet(String),
    CreateGame,
    GameCreated(CreateGameResponse),
    JoinGame(i32),
    GameJoined(JoinGameResponse),
    LobbyUpdate(LobbyStream),
}
