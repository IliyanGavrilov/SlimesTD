use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{Bullet, EnemyDeathEvent, GameData, GameState, Tower, WaveClearedEvent, Waves};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
  fn build(&self, app: &mut App) {
    app
      // bevy_kira_audio backend
      .add_plugin(bevy_kira_audio::prelude::AudioPlugin)
      .add_audio_channel::<MusicChannel>()
      .add_audio_channel::<SfxChannel>()
      .add_event::<EnemyJumpEvent>()
      .insert_resource(AudioSettings::default())
      .insert_resource(CurrentMusic::None)
      // Load audio + apply initial volumes before startup
      .add_startup_system(load_audio.in_base_set(StartupSet::PreStartup))
      .add_system(apply_volume_settings)
      // Music per game state
      .add_system(play_menu_music.in_schedule(OnEnter(GameState::MainMenu)))
      .add_system(play_gameplay_music.in_schedule(OnEnter(GameState::Gameplay)))
      // SFX
      .add_system(sfx_button_click)
      .add_systems(
        (sfx_shoot, sfx_enemy_death, sfx_enemy_jump, sfx_tower_placed, sfx_wave_start)
          .in_set(OnUpdate(GameState::Gameplay)),
      )
      .add_systems(
        (stop_music, sfx_game_over).in_schedule(OnEnter(GameState::GameOver)),
      )
      .add_systems(
        (stop_music, sfx_victory).in_schedule(OnEnter(GameState::Victory)),
      );
  }
}

/// Channel for looping background music.
#[derive(Resource)]
pub struct MusicChannel;

/// Channel for one-shot sound effects.
#[derive(Resource)]
pub struct SfxChannel;

/// Sent by the enemy sprite animation each time a slime completes a jump cycle,
/// so the jump SFX stays in sync with the visible hop.
pub struct EnemyJumpEvent;

/// Which music track is currently playing, so re-entering a state (e.g. coming
/// back from the Settings screen) doesn't restart the same track.
#[derive(Resource, PartialEq, Clone, Copy)]
pub enum CurrentMusic {
  None,
  Menu,
  Gameplay,
}

/// User-adjustable volumes, all in `0.0..=1.0`. `master` scales the others.
#[derive(Resource)]
pub struct AudioSettings {
  pub master: f32,
  pub music: f32,
  pub sfx: f32,
}

impl Default for AudioSettings {
  fn default() -> Self {
    Self {
      master: 1.0,
      music: 0.4,
      sfx: 0.6,
    }
  }
}

#[derive(Resource)]
pub struct GameAudio {
  pub menu_music: Handle<AudioSource>,
  pub gameplay_music: Handle<AudioSource>,
  pub shoot: Handle<AudioSource>,
  pub enemy_death: Handle<AudioSource>,
  pub enemy_jump: Handle<AudioSource>,
  pub tower_place: Handle<AudioSource>,
  pub wave_start: Handle<AudioSource>,
  pub button_click: Handle<AudioSource>,
  pub game_over: Handle<AudioSource>,
  pub victory: Handle<AudioSource>,
}

fn load_audio(mut commands: Commands, assets: Res<AssetServer>) {
  commands.insert_resource(GameAudio {
    menu_music: assets.load("audio/music/menu_music.ogg"),
    gameplay_music: assets.load("audio/music/gameplay_music.ogg"),
    shoot: assets.load("audio/sfx/shoot.ogg"),
    enemy_death: assets.load("audio/sfx/enemy_death.ogg"),
    enemy_jump: assets.load("audio/sfx/enemy_jump.ogg"),
    tower_place: assets.load("audio/sfx/tower_place.ogg"),
    wave_start: assets.load("audio/sfx/wave_start.ogg"),
    button_click: assets.load("audio/sfx/button_click.ogg"),
    game_over: assets.load("audio/sfx/game_over.ogg"),
    victory: assets.load("audio/sfx/victory.ogg"),
  });
}

fn apply_volume_settings(
  settings: Res<AudioSettings>,
  music: Res<AudioChannel<MusicChannel>>,
  sfx: Res<AudioChannel<SfxChannel>>,
) {
  if settings.is_changed() {
    music.set_volume((settings.master * settings.music) as f64);
    sfx.set_volume((settings.master * settings.sfx) as f64);
  }
}

fn play_menu_music(
  audio: Res<GameAudio>,
  music: Res<AudioChannel<MusicChannel>>,
  mut current: ResMut<CurrentMusic>,
) {
  if *current == CurrentMusic::Menu {
    return;
  }
  *current = CurrentMusic::Menu;
  music.stop();
  music.play(audio.menu_music.clone()).looped();
}

fn play_gameplay_music(
  audio: Res<GameAudio>,
  music: Res<AudioChannel<MusicChannel>>,
  mut current: ResMut<CurrentMusic>,
) {
  if *current == CurrentMusic::Gameplay {
    return;
  }
  *current = CurrentMusic::Gameplay;
  music.stop();
  music.play(audio.gameplay_music.clone()).looped();
}

fn sfx_shoot(
  audio: Res<GameAudio>,
  sfx: Res<AudioChannel<SfxChannel>>,
  new_bullets: Query<(), Added<Bullet>>,
) {
  // Play once per frame regardless of how many towers fired, to avoid stacking.
  // Quieter than other SFX since it fires very frequently.
  if !new_bullets.is_empty() {
    sfx.play(audio.shoot.clone()).with_volume(0.35);
  }
}

fn sfx_enemy_death(
  audio: Res<GameAudio>,
  sfx: Res<AudioChannel<SfxChannel>>,
  mut deaths: EventReader<EnemyDeathEvent>,
) {
  if deaths.iter().next().is_some() {
    deaths.clear();
    sfx.play(audio.enemy_death.clone());
  }
}

fn sfx_enemy_jump(
  time: Res<Time>,
  audio: Res<GameAudio>,
  sfx: Res<AudioChannel<SfxChannel>>,
  mut jumps: EventReader<EnemyJumpEvent>,
  mut cooldown: Local<f32>,
) {
  *cooldown -= time.delta_seconds();
  let any_jumped = jumps.iter().next().is_some();
  jumps.clear();
  // Many slimes hop out of phase; throttle so it stays lively without stacking
  // into a buzz. Quiet, since it plays often.
  if any_jumped && *cooldown <= 0. {
    sfx.play(audio.enemy_jump.clone()).with_volume(0.4);
    *cooldown = 0.06;
  }
}

fn sfx_tower_placed(
  audio: Res<GameAudio>,
  sfx: Res<AudioChannel<SfxChannel>>,
  new_towers: Query<(), Added<Tower>>,
) {
  if !new_towers.is_empty() {
    sfx.play(audio.tower_place.clone());
  }
}

fn sfx_wave_start(
  audio: Res<GameAudio>,
  sfx: Res<AudioChannel<SfxChannel>>,
  game_data: Res<GameData>,
  wave_assets: Res<Assets<Waves>>,
  mut wave_cleared: EventReader<WaveClearedEvent>,
) {
  let total = wave_assets
    .get(&game_data.enemy_waves)
    .map_or(0, |w| w.waves.len());
  // `WaveClearedEvent` also fires for the final wave (which leads to Victory, not
  // a new wave) — only play the "new wave" cue when another wave actually follows.
  let starts_new_wave = wave_cleared.iter().any(|e| e.index + 1 < total);
  wave_cleared.clear();
  if starts_new_wave {
    sfx.play(audio.wave_start.clone());
  }
}

fn sfx_button_click(
  audio: Res<GameAudio>,
  sfx: Res<AudioChannel<SfxChannel>>,
  buttons: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
  if buttons.iter().any(|i| matches!(i, Interaction::Clicked)) {
    sfx.play(audio.button_click.clone());
  }
}

fn stop_music(music: Res<AudioChannel<MusicChannel>>, mut current: ResMut<CurrentMusic>) {
  music.stop();
  *current = CurrentMusic::None;
}

fn sfx_game_over(audio: Res<GameAudio>, sfx: Res<AudioChannel<SfxChannel>>) {
  sfx.play(audio.game_over.clone());
}

fn sfx_victory(audio: Res<GameAudio>, sfx: Res<AudioChannel<SfxChannel>>) {
  sfx.play(audio.victory.clone());
}
