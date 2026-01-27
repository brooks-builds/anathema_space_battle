use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

use crate::router::Route;

pub struct App;

impl App {
    pub fn ident() -> &'static str {
        "app"
    }
}

#[derive(Debug, State, Default)]
pub struct AppState {
    width: Value<u16>,
    height: Value<u16>,
    started: Value<bool>,
    current_route: Value<String>,
    player_name: Value<String>,
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

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "start_game" => {
                let started = *state.started.to_ref();

                state.started.set(!started);
            }
            "nav_to" => {
                let route = event.data_checked::<Route>().copied().unwrap_or_default();

                state.current_route.set(route.into());
            }
            _ => unimplemented!(),
        }
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            AppMessage::HostGame { player_name } => {
                state.player_name.set(player_name);
                state.current_route.set(Route::Lobby.into());
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
            Self,
            AppState::default(),
        )?;

        Ok(())
    }
}

pub enum AppMessage {
    HostGame { player_name: String },
}
