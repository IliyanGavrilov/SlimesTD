# Audio assets

The game looks for the files below. **All must be `.ogg`** (Ogg Vorbis). The game
runs fine if any are missing - Bevy just logs a warning and stays silent for that
sound - so you can add them incrementally.

Drop the files at exactly these paths:

```
assets/audio/
  music/
    menu_music.ogg       # loops on the main menu
    gameplay_music.ogg   # loops during gameplay
  sfx/
    shoot.ogg            # a tower fires a bullet
    enemy_death.ogg      # a slime dies
    enemy_jump.ogg       # a slime completes a hop (synced to the jump animation)
    tower_place.ogg      # a tower is placed
    wave_start.ogg       # a new wave begins
    button_click.ogg     # any UI button is clicked
    game_over.ogg        # base destroyed
    victory.ogg          # all waves cleared
```

## Where to get them for free (no attribution headaches)

All of these let you use sounds for free; prefer CC0 / public-domain so you don't
owe attribution in a university project.

1. **Kenney.nl** - https://kenney.nl/assets?q=audio
   - Best first stop. Everything is CC0 (public domain, no attribution required).
   - Packs to grab: "Interface Sounds" (button_click), "Impact Sounds" /
     "Digital Audio" (shoot, enemy_death), "UI Audio" (wave_start, game_over,
     victory).

2. **OpenGameArt.org** - https://opengameart.org/
   - Filter by license **CC0**. Search "shoot", "magic", "slime", "win jingle".
   - Loopable background music: search "loop" + "fantasy" / "8-bit", filter CC0.

3. **freesound.org** - https://freesound.org/
   - Huge library. Filter license to **Creative Commons 0**. Great for one-shots
     like impacts, pops, magic zaps.

4. **Pixabay** - https://pixabay.com/sound-effects/ and https://pixabay.com/music/
   - Pixabay license, free for commercial use, no attribution. Good for music.

5. **Incompetech (Kevin MacLeod)** - https://incompetech.com/music/royalty-free/
   - Free music if you credit (CC-BY). Good loopable tracks for the two music files.

## Converting to .ogg

If a source gives you `.wav` / `.mp3`, convert with ffmpeg:

```
ffmpeg -i input.wav music/menu_music.ogg
```

(Online converters like cloudconvert.com also work.)

## Tips

- Keep SFX short (< 1s) and not too loud - `shoot.ogg` plays a lot.
- Make the two music tracks seamless loops (the game loops them automatically).
- Default volumes live in `AudioSettings` (`src/audio/audio.rs`): master 1.0,
  music 0.4, sfx 0.6. Tune in-game once the volume sliders are added.
