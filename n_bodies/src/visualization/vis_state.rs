use krabmaga::bevy::ecs as bevy_ecs;
use krabmaga::bevy::ecs::system::Commands;
use krabmaga::bevy::prelude::Resource;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;
use crate::model::body::Body;
use crate::model::state::NBodyState;
use crate::visualization::body_vis::BodyVis;

#[derive(Clone, Resource)]
pub struct NBodyVis;

impl VisualizationState<NBodyState> for NBodyVis {
    fn on_init(
        &self,
        _commands: &mut Commands,
        _factory: &mut AssetHandleFactoryResource,
        _state: &mut NBodyState,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {}

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &NBodyState,
    ) -> Option<Box<dyn AgentRender>> {
        let b = agent.downcast_ref::<Body>().unwrap();
        Some(Box::new(BodyVis { id: b.id, radius: b.radius, is_star: b.id == 0 }))
    }

    fn get_agent(
        &self,
        agent_render: &dyn AgentRender,
        state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        let s = state.as_any().downcast_ref::<NBodyState>().unwrap();
        let id = agent_render.get_id() as usize;
        Some(Box::new(s.bodies[id]))
    }
}