use anyhow::Result;
use bevy::{
    prelude::App, render::texture::ImageSettings, window::WindowDescriptor, DefaultPlugins,
};

pub fn run() -> Result<()> {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "cheso".to_string(),
        ..WindowDescriptor::default()
    })
    .add_system(bevy::window::close_on_esc)
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins)
    .run();
    Ok(())
}
