/*================================================================*\
** Butterscotch | Copyright 2021 NotVeryMoe, projects@notvery.moe **
\*================================================================*/

use bevy::ecs::world::World;

pub struct ExtractionWorld(pub World);

impl std::ops::Deref for ExtractionWorld {
    type Target = World;
    fn deref(&self) -> &Self::Target { 
        &self.0 
    }
}

impl std::ops::DerefMut for ExtractionWorld {
    fn deref_mut(&mut self) -> &mut Self::Target { 
        &mut self.0 
    }
}