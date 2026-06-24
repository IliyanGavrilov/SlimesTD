use super::*;

fn settings(master: f32, sfx: f32) -> AudioSettings {
  AudioSettings {
    master,
    music: 1.0,
    sfx,
  }
}

#[test]
fn full_volume_passes_base_through() {
  assert_eq!(scaled_sfx_volume(1.0, &settings(1.0, 1.0)), 1.0);
}

#[test]
fn base_level_scales_the_result() {
  assert_eq!(scaled_sfx_volume(0.5, &settings(1.0, 1.0)), 0.5);
}

#[test]
fn master_and_sfx_multiply_together() {
  assert_eq!(scaled_sfx_volume(1.0, &settings(0.5, 0.5)), 0.25);
}

#[test]
fn muted_master_silences_sfx() {
  assert_eq!(scaled_sfx_volume(1.0, &settings(0.0, 1.0)), 0.0);
}

#[test]
fn muted_sfx_silences_sfx() {
  assert_eq!(scaled_sfx_volume(1.0, &settings(1.0, 0.0)), 0.0);
}

#[test]
fn zero_base_is_silent() {
  assert_eq!(scaled_sfx_volume(0.0, &settings(1.0, 1.0)), 0.0);
}
