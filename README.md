# Slimes TD

[![Release](https://github.com/IliyanGavrilov/SlimesTD/actions/workflows/release.yml/badge.svg)](https://github.com/IliyanGavrilov/SlimesTD/actions/workflows/release.yml)
[![Web](https://github.com/IliyanGavrilov/SlimesTD/actions/workflows/web.yml/badge.svg)](https://github.com/IliyanGavrilov/SlimesTD/actions/workflows/web.yml)
![Rust 2024](https://img.shields.io/badge/Rust-2024-orange)
![Bevy 0.10](https://img.shields.io/badge/Bevy-0.10-232326)

**Play: [itch.io](https://iliyangavrilov.itch.io/slimestd)** or **[GitHub Pages](https://iliyangavrilov.github.io/SlimesTD/)** - runs in the browser, nothing to install.

A tower defense game made in Rust with the Bevy engine. Waves of slimes walk along a path toward your base. Place wizard towers, earn gold, upgrade your defenses and **survive all 10 waves** to win.

My diploma project at TUES (2023), reworked in 2026.

## Desktop builds

Windows, Linux and macOS builds are on the **[Releases](../../releases)** page. Unzip and run, no installation needed.

On Windows, SmartScreen may warn on first launch (More info > Run anyway). On macOS the build is unsigned, so right-click > Open the first time.

## How to play

Pick a tower from the tower bar (or press `0`-`9`), left click on the map to place it and right click to cancel. Click a placed tower to upgrade it, sell it or change its targeting. Each slime that reaches the end costs you health.

There are three maps. On the third one a second lane opens from wave 2, so you have to defend two routes at once.

### Towers

| Tower | Price | Effect |
|---|---|---|
| Nature Wizard | $100 | Poison damage over time |
| Fire Wizard | $125 | Splash damage |
| Ice Wizard | $100 | Slows enemies, can be placed on water |
| Dark Wizard | $150 | Knockback, can see invisible slimes |
| Mage Wizard | $175 | Stuns, water only |
| Archmage | $250 | Chain lightning |
| Farms (4 types) | $125 - $200 | Earn gold over time, per kill or per wave |

Combat towers have **3 upgrade paths with 5 levels each**. Targeting can be set to first, last, closest, farthest, strongest, weakest or random.

### Enemies

8 slime colors with 1 to 8 health. Some waves have **invisible slimes** that only the Dark Wizard can hit.

## Building from source

- **[Rust](https://rustup.rs/) 1.85+**
- On Linux, a few system libraries for Bevy (below)

Then:

```bash
git clone https://github.com/IliyanGavrilov/SlimesTD.git
cd SlimesTD
cargo run
```

Linux libraries:

```bash
sudo apt install libasound2-dev libudev-dev libx11-dev libxcb1-dev pkg-config
```

Web build:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```

## Project structure

```
src/
  map/          tilemap, pathfinding
  tower/        placement, upgrades, projectiles, targeting
  enemy/        enemies and waves
  effects/      poison, splash, chain lightning, knockback
  audio/        music and sound effects
  main_menu/    menus, map selection, settings
  tutorial/     first launch tutorial
  persistence/  saving settings and progress
assets/data/    tower stats, enemy stats, waves and upgrades (RON files)
```

Tower stats, waves and upgrades are loaded from the RON files in `assets/data/`, so **balancing the game doesn't require recompiling**.

GitHub Actions builds the desktop releases and deploys the web version to GitHub Pages and itch.io on every push to `main`.

## Tech

Rust, Bevy 0.10, bevy_ecs_tilemap, bevy_kira_audio, bevy_asset_loader, RON, WebAssembly
