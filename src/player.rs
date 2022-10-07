use crate::actions::{Actions, GameControl};
use crate::loading::TextureAssets;
use crate::logic::PlayerControl;
use crate::GameState;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use iyes_loopless::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player {
    pub pos: Vec2,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, spawn_player)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(animate_sprite)
                    .with_system(move_player)
                    .into(),
            );
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("textures/duck_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 1, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(8., 8., 10.),
            // transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(Player::default())
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

// impl Player {
//     fn set_position(&mut self, new_pos: Vec2) {
//         self.pos = new_pos
//     }
// }

pub fn move_player(
    mut player_query: Query<(&mut Transform, &Player), (With<Player>,)>,
    player_control: Res<PlayerControl>,
) {
    let pl_move = player_control.player_pos;
    for (mut transform, player) in player_query.iter_mut() {
        transform.translation = Vec3::new(
            player.pos[0] + f32::from(pl_move[0]) * 16.,
            player.pos[1] + f32::from(pl_move[1]) * 16.,
            0.,
        );
    }
}
