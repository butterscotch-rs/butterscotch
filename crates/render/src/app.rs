/*================================================================*\
** Butterscotch | Copyright 2021 NotVeryMoe, projects@notvery.moe **
\*================================================================*/

use bevy::{
    app::{App, AppLabel, }, 
    ecs::schedule::{ScheduleStageKind, StageLabel, SystemStage}
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum RenderStage {
    Extract,
    Prepare,
    Render,
    Cleanup,
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, AppLabel)]
pub struct RenderApp;

impl RenderApp {

    pub fn create() -> App {
        let mut render_app = App::empty();

        render_app
            .add_stage(RenderStage::Extract, SystemStage::parallel()).set_stage_kind(&RenderStage::Extract, ScheduleStageKind::Extract)
            .add_stage(RenderStage::Prepare, SystemStage::parallel())
            .add_stage(RenderStage::Render,  SystemStage::parallel()/*with_system(render_system.exclusive_system())*/)
            .add_stage(RenderStage::Cleanup, SystemStage::parallel());
            
        render_app
    }

}

