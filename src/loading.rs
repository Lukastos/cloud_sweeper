#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use crate::{audio::SelectedSong, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    pub selected_song: SelectedSong,
    #[asset(path = "audio/song_1/song_full.wav.ron")]
    pub song_1: Handle<AudioSource>,
    #[asset(path = "audio/song_1/sample_1.wav.ron")]
    pub song_1_sample_1: Handle<AudioSource>,
    #[asset(path = "audio/song_1/sample_2.wav.ron")]
    pub song_1_sample_2: Handle<AudioSource>,
    #[asset(path = "audio/song_2/song_full.wav.ron")]
    pub song_2: Handle<AudioSource>,
    #[asset(path = "audio/debug_beep_200ms.wav.ron")]
    pub debug_beep: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}
