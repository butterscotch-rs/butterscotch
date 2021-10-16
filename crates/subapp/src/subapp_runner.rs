/*================================================================*\
** Butterscotch | Copyright 2021 NotVeryMoe, projects@notvery.moe **
\*================================================================*/

use bevy::{
    app::App, 
    ecs::{
        schedule::{
            Stage, 
            SystemStage,
            ShouldRun
        },
        world::World
    }
};

use crate::ExtractionWorld;

pub fn run_subapp(parent: &mut World, app: &mut App) {
    loop {
        match app.schedule.should_run(&mut app.world) {
            ShouldRun::YesAndCheckAgain => run_subapp_once(parent, app),
            ShouldRun::NoAndCheckAgain  => panic!("`NoAndCheckAgain` would loop infinitely in this situation."),
            ShouldRun::Yes              => { run_subapp_once(parent, app); return; },
            ShouldRun::No               => return,
        }
    }
}

pub fn run_subapp_once(parent: &mut World, app: &mut App) {
    let world    = &mut app.world;
    let schedule = &mut app.schedule;

    /* Reserve game entities in child world */ {
        let meta_len = parent.entities().meta.len();
        world.entities().reserve_entities(meta_len as u32);
        world.entities_mut().flush_as_invalid();
    }

    schedule.for_each_stages_mut(|_, stage| {
        if stage.is_extract_stage() {
            run_extract_stage_with_app_world(parent, stage, world);
        } else {
            stage.run(world)
        }
    });

}




struct ExtractionWorldScratch(World);

fn run_extract_stage_with_app_world(parent: &mut World, stage: &mut dyn Stage, stage_world: &mut World) {    

    // TODO deal with system stage more generically or something?

    if let Some(stage) = stage.downcast_mut::<SystemStage>() {
        stage.set_apply_buffers(false); // TODO panic if not set instead?
    }

    // temporarily add the child world to the app world
    let scratch_world = parent.remove_resource::<ExtractionWorldScratch>().unwrap();
    let render_world = std::mem::replace(stage_world, scratch_world.0);
    parent.insert_resource(ExtractionWorld(render_world));

    stage.run(parent);

    // add the child world back to the subapp
    let render_world = parent.remove_resource::<ExtractionWorld>().unwrap();
    let scratch_world = std::mem::replace(stage_world, render_world.0);
    parent.insert_resource(ExtractionWorldScratch(scratch_world));

    if let Some(stage) = stage.downcast_mut::<SystemStage>() {
        stage.apply_buffers(stage_world);
    }
}
