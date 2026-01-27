use anathema::runtime::Builder;
use bb_anathema_components::BBAppComponent;

use crate::pages::splash::SplashPage;

pub mod splash;

pub fn register_pages(builder: &mut Builder<()>) -> eyre::Result<()> {
    SplashPage::register_to(builder)?;

    Ok(())
}
