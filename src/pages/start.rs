use crate::app::{App, AppMessage};
use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct StartPage;

#[derive(Debug, State, Default)]
pub struct StartPageState {
    game_code: Value<String>,
}

impl Component for StartPage {
    type State = StartPageState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        event.stop_propagation();

        match event.name() {
            "create_game" => context
                .components
                .by_name(App::ident())
                .send(AppMessage::CreateGame),
            "game_code_changed" => {
                let game_code = event.data_checked::<String>().cloned().unwrap_or_default();

                state.game_code.set(game_code);
            }
            "join_game" => {
                let code = state
                    .game_code
                    .to_ref()
                    .parse::<i32>()
                    .ok()
                    .unwrap_or_default();
                let message = AppMessage::JoinGame(code);

                context.components.by_name(App::ident()).send(message);
            }
            _ => unreachable!(),
        }
    }
}

impl BBAppComponent for StartPage {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "StartPage",
            "templates/pages/start.aml",
            Self,
            StartPageState::default(),
        )?;

        Ok(())
    }
}
