use crate::data::components::Player;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct CameraPlugin;

const RGB_MAX: f32 = 255.;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_fit_inside_current_level)
            .insert_resource(ClearColor(Color::srgb(
                36. / RGB_MAX,
                39. / RGB_MAX,
                58. / RGB_MAX,
            )));
    }
}
#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(&mut Projection, &mut Transform), Without<Player>>,
    level_query: Query<(&Transform, &LevelIid), (Without<Projection>, Without<Player>)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    //resolution: &WindowResolution,
) -> Result {
    //if let Ok(Transform {
    //     translation: player_translation,
    //     ..
    //}) = player_query.get_single()
    //{
    //let player_translation = *player_translation;

    let (mut projection, mut camera_transform) = camera_query.single_mut()?;
    let Projection::Orthographic(orthographic_projection) = &mut *projection else {
        return Err(BevyError::from("non-orthographic projection found"));
    };
    //let aspect_ratio = resolution.width() / resolution.height();

    for (level_transform, level_iid) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single()?)
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        let aspect_ratio = 16.0 / 9.0; // TODO: get window resolution and use that instead
        let level_ratio = (level.px_wid as f32 / level.px_hei as f32).round();
        orthographic_projection.viewport_origin = Vec2::ZERO;
        let level_wider_than_screen = level_ratio > aspect_ratio;
        if level_wider_than_screen {
            let width = level.px_wid as f32;
            let height = (width / aspect_ratio).round();
            orthographic_projection.scaling_mode =
                bevy::render::camera::ScalingMode::FixedHorizontal {
                    viewport_width: width,
                };
            camera_transform.translation.x = -level_transform.translation.x;
            camera_transform.translation.y =
                -level_transform.translation.y + ((level.px_hei as f32 - height) / 2.).round();
        } else {
            let height = level.px_hei as f32;
            let width = (height * aspect_ratio).round();
            orthographic_projection.scaling_mode =
                bevy::render::camera::ScalingMode::FixedHorizontal {
                    viewport_width: width,
                };
            camera_transform.translation.y = -level_transform.translation.y;
            camera_transform.translation.x =
                (-level_transform.translation.x + (level.px_wid as f32 - width) / 2.).round();
        }

        camera_transform.translation.x += level_transform.translation.x;
        camera_transform.translation.y += level_transform.translation.y;
    }
    Ok(())
}
