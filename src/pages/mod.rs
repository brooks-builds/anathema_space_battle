pub mod lobby;
mod splash;
mod start;

use crate::pages::{lobby::LobbyPage, splash::SplashPage, start::StartPage};
use anathema::runtime::Builder;
use bb_anathema_components::BBAppComponent;

pub fn register_pages(builder: &mut Builder<()>) -> eyre::Result<()> {
    SplashPage::register_to(builder)?;
    LobbyPage::register_to(builder)?;
    StartPage::register_to(builder)?;

    Ok(())
}
