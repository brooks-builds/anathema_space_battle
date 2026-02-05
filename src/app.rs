use crate::{
    api::{self, CreateGameResponse, JoinGameResponse, LobbyStream, Ship, ShipColor},
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
    possible_ship_color_names: Value<List<String>>,
    possible_ship_names: Value<List<String>>,
    possible_ship_chars: Value<List<char>>,
}

#[derive(Debug, Default)]
pub struct AppData {
    game_id: Option<String>,
    player_id: Option<String>,
    token: Option<String>,
    possible_ship_colors: Vec<ShipColor>,
    possible_ships: Vec<Ship>,
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
        api::get_possible_colors(context.widget_id, context.emitter.clone());
        api::get_possible_ships(context.widget_id, context.emitter.clone());
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
                context
                    .components
                    .by_name(LobbyPage::ident())
                    .send(AppMessage::LobbyUpdate(lobby_stream));
            }
            AppMessage::PossibleShipColors(ship_colors) => {
                self.0.possible_ship_colors = ship_colors.clone();

                state.possible_ship_color_names.set(List::from_iter(
                    ship_colors.into_iter().map(|ship_color| ship_color.name),
                ));
            }
            AppMessage::ChangingShipColor(color_name) => {
                let Some(color) = self
                    .0
                    .possible_ship_colors
                    .iter()
                    .find(|color| color.name == color_name)
                else {
                    dbg!("Changing to a ship color that we don't have", color_name);
                    return;
                };
                let color_id = &color.id;
                let Some(token) = &self.0.token else {
                    dbg!("attempting to change ship color without a token");
                    return;
                };

                api::set_ship_color(token.clone(), color_id);
            }
            AppMessage::PossibleShips(ships) => {
                self.0.possible_ships = ships.clone();

                let ship_names = ships.iter().map(|ship| ship.class_name.clone());
                let ship_chars = ships.iter().map(|ship| ship.character);

                state.possible_ship_names.set(List::from_iter(ship_names));
                state.possible_ship_chars.set(List::from_iter(ship_chars));
            }
            AppMessage::ChangeShip(ship_name) => {
                let Some(ship) = self
                    .0
                    .possible_ships
                    .iter()
                    .find(|ship| ship.class_name == ship_name)
                else {
                    eprintln!("Attempting to switch to ship name that doesn't exist: {ship_name}");
                    return;
                };
                let Some(token) = &self.0.token else {
                    eprintln!("Missing token when attempting to change ship");
                    return;
                };
                api::change_ship(&ship.id, token.clone());
            }
            AppMessage::ReadyUp => {
                let Some(token) = &self.0.token else {
                    eprintln!("Trying to ready up without a player token");
                    return;
                };

                api::ready_up(token.clone());
            }
            AppMessage::Quit => {
                context.stop_runtime();
            }
        }
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "quit" {
            if let Some(token) = &self.0.token {
                api::quit(token.clone(), context.widget_id, context.emitter.clone());
            } else {
                context
                    .components
                    .by_name(Self::ident())
                    .send(AppMessage::Quit);
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
    PossibleShipColors(Vec<ShipColor>),
    ChangingShipColor(String),
    PossibleShips(Vec<Ship>),
    ChangeShip(String),
    ReadyUp,
    Quit,
}
