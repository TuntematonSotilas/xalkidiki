use std::time::Duration;

use bevy::prelude::*;
use seldom_pixel::{prelude::*, cursor::PxCursorPosition};
use bevy_ecs_tilemap::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::{states::AppState, Layer, Player};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
			.add_systems(Update, (click) //.run_if(on_timer(Duration::from_millis(100))));
                .run_if(in_state(AppState::MapLoaded)));

    }
}

fn setup(
	mut commands: Commands, 
	mut tilesets: PxAssets<PxTileset>,
    mut next_state: ResMut<NextState<AppState>>
) {
	let map_size = TilemapSize { x: 8, y: 8 };
    let mut storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {

            let isl = y >= 1 && y <= 5 && x >= 1 && x <= 6;
            let idx = if isl {
				x + (6 * (y - 1)) // Island
			} else {
				0 //Sea
			};
			
            // Each tile must be added to the `TileStorage`
            storage.set(
                &TilePos { x, y },
                commands
                    .spawn(PxTileBundle {
                        texture: TileTextureIndex(idx),
                        ..default()
                    })
                    .id(),
            );
        }
    }

    // Spawn the map
    commands.spawn((
		PxMapBundle::<Layer> {
			size: map_size,
			storage,
			tileset: tilesets.load("/public/tileset/tileset.png", UVec2::splat(8)),
			..default()
		},
		PxAnimationBundle {
            // Use millis_per_animation to have each tile loop at the same time
            duration: PxAnimationDuration::millis_per_frame(1000),
            on_finish: PxAnimationFinishBehavior::Loop,
			frame_transition: PxAnimationFrameTransition::None,
            ..default()
        }),
	);

    next_state.set(AppState::MapLoaded);

}

pub fn set_map_loaded(mut next_state: ResMut<NextState<AppState>>) {
    
}


pub fn click(
	cursor_pos: Res<PxCursorPosition>,
    buttons: Res<Input<MouseButton>>,
	mut player_q: Query<&mut PxPosition, With<Player>>,
) {
	if buttons.just_released(MouseButton::Left) {
		
		warn!("click");

		if let Some(cur_pos) = **cursor_pos {


			warn!("click : {0} {1}", cur_pos.x, cur_pos.y);

			let mut player_pos = player_q.single_mut();
			**player_pos = IVec2::new(cur_pos.x as i32, cur_pos.y as i32);

			//warn!("player_pos : {0} {1}", player_pos.x, player_pos.y);
			

		}
	}
}