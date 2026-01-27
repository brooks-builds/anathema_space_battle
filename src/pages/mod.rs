mod lobby;
mod splash;

use crate::pages::{lobby::LobbyPage, splash::SplashPage};
use anathema::runtime::Builder;
use bb_anathema_components::BBAppComponent;

pub fn register_pages(builder: &mut Builder<()>) -> eyre::Result<()> {
    SplashPage::register_to(builder)?;
    LobbyPage::register_to(builder)?;

    Ok(())
}
