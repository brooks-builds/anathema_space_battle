use anathema::{component::Component, state::State};
use bb_anathema_components::BBAppComponent;

use crate::app::{App, AppMessage};

pub struct StartPage;

#[derive(Debug, State, Default)]
pub struct StartPageState {}

impl Component for StartPage {
    type State = StartPageState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "create_game" => context
                .components
                .by_name(App::ident())
                .send(AppMessage::CreateGame),
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
