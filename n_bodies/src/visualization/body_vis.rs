use krabmaga::bevy::prelude::{Quat, Transform, Visibility};
use krabmaga::engine::agent::Agent;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};
use crate::model::body::Body;

#[derive(Clone)]
pub struct BodyVis {
    pub id: u32,
    pub radius: f32,
    pub is_star: bool,
}

// to control sprite size
const SPRITE_PX: f32 = 512.0; 

impl AgentRender for BodyVis {
    fn sprite(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> SpriteType {
        if self.is_star {
            SpriteType::Emoji(String::from("sun")) // sun
        } else {
            SpriteType::Emoji(String::from("planet")) // full moon
        }
    }

    fn location(&self, agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> (f32, f32, f32) {
        let b = agent.downcast_ref::<Body>().unwrap();
        (b.pos.x, b.pos.y, if self.is_star { 1.0 } else { 0.0 })
    }


    fn scale(&self, _a: &Box<dyn Agent>, _s: &Box<&dyn State>) -> (f32, f32) {
        let k = (self.radius * 2.0) / SPRITE_PX * 2.0;
        (k, k)
    }

    fn rotation(&self, _agent: &Box<dyn Agent>, _state: &Box<&dyn State>) -> f32 {
        0.0
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn update(
        &mut self,
        agent: &Box<dyn Agent>,
        transform: &mut Transform,
        _state: &Box<&dyn State>,
        visible: &mut Visibility,
    ) {
        let b = agent.downcast_ref::<Body>().unwrap();
        transform.translation.x = b.pos.x;
        transform.translation.y = b.pos.y;
        transform.rotation = Quat::from_rotation_z(0.0);
        *visible = Visibility::Visible;
    }
}