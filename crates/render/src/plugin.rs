/*================================================================*\
** Butterscotch | Copyright 2021 NotVeryMoe, projects@notvery.moe **
\*================================================================*/

use bevy::{
    app::{
        App,
        Plugin,
    },
};

use crate::RenderApp;

#[derive(Default)]
pub struct RenderPlugin;


impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_app(RenderApp, RenderApp::create());
    }
}
