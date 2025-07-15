use bevy::{asset::AssetEvents, prelude::*, scene::SceneInstance};

/// Plugin that respawns scenes that have [`AssetEvent::Modified`] sent out.
pub struct SceneHotReloadingPlugin;
impl Plugin for SceneHotReloadingPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Last, Self::hot_reload_scenes.after(AssetEvents));
	}
}
impl SceneHotReloadingPlugin {
	/// Respawn scenes that have [`AssetEvent::Modified`] sent out.
	pub fn hot_reload_scenes(
		mut commands: Commands,
		mut scene_events: EventReader<AssetEvent<Scene>>,
		query: Query<(Entity, &SceneRoot)>,
	) {
		for event in scene_events.read() {
			let AssetEvent::Modified { id } = event else { continue };

			for (entity, scene_root) in &query {
				if scene_root.0.id() != *id {
					continue;
				}

				commands
					.entity(entity)
					.despawn_related::<Children>()
					.remove::<SceneInstance>()
					.insert(SceneRoot(scene_root.0.clone()));

				#[cfg(feature = "log")]
				if let Some(path) = scene_root.0.path() {
					info!("Reloaded scene {} in world.", path);
				} else {
					info!("Reloaded scene {id} in world.");
				}
			}
		}
	}
}
