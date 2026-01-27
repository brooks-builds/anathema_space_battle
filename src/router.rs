use anathema::component::Component;
use bb_anathema_components::BBAppComponent;

pub struct Router;

impl Component for Router {
    type State = ();

    type Message = ();
}

impl BBAppComponent for Router {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component("router", "templates/router.aml", Self, ())?;

        Ok(())
    }
}

pub enum Route {
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
