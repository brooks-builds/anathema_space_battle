mod api;
mod app;
mod game;
mod pages;
mod router;

use anathema::{
    prelude::{Backend, Document, TuiBackend},
    runtime::Runtime,
};
use bb_anathema_components::BBAppComponent;

use crate::{app::App, game::Game, router::Router};

pub fn run() -> eyre::Result<()> {
    let doc = Document::new("@app");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()?;

    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    bb_anathema_components::register_all(&mut builder)?;
    App::register_to(&mut builder)?;
    Game::register_to(&mut builder)?;
    Router::register_to(&mut builder)?;
    pages::register_pages(&mut builder)?;

    builder.finish(&mut backend, |runtime, backend| runtime.run(backend))?;

    Ok(())
}
