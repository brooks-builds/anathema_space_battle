use anathema::component::Component;
use bb_anathema_components::BBAppComponent;

pub struct Router;

impl Component for Router {
    type State = ();

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        event.stop_propagation();

        match event.name() {
            "nav_to" => {
                let route = event.data_checked::<Route>().copied().unwrap_or_default();

                context.publish("nav_to", route);
            }
            _ => unimplemented!(),
        }
    }
}

impl BBAppComponent for Router {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component("router", "templates/router.aml", Self, ())?;

        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub enum Route {
    #[default]
    Home,
    Lobby,
    Game,
    GameOver,
}

impl From<Route> for String {
    fn from(value: Route) -> Self {
        match value {
            Route::Home => "Home",
            Route::Lobby => "Lobby",
            Route::Game => "Game",
            Route::GameOver => "GameOver",
        }
        .to_owned()
    }
}
