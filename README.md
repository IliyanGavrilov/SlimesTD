# Slimes TD

A tower defense game where waves of colored slimes march along a path toward your base. Place wizard towers, earn gold, upgrade your defenses, and survive all 10 waves to win.

Built with [Bevy 0.10.1](https://bevyengine.org/) and Rust.

**Itch.io (play in browser or download):** https://iliyangavrilov.itch.io/slimestd

---

## Platforms

| Platform | How to get it |
|----------|--------------|
| Windows  | `installation/windows.zip` → run `Slimes TD.exe` |
| Linux    | `installation/linux.zip` → run `Slimes TD` (mark executable if needed) |
| Browser  | Play directly on [itch.io](https://iliyangavrilov.itch.io/slimestd) (WebAssembly) |

### Linux = first run

```bash
unzip "installation/linux.zip"
chmod +x "Slimes TD - Linux/Slimes TD"
"Slimes TD - Linux/Slimes TD"
```

Bevy on Linux requires a few system libraries. Install them if the binary fails to launch:

**Ubuntu / Debian:**
```bash
sudo apt install libasound2-dev libudev-dev libx11-dev libxcb1-dev pkg-config
```

**Fedora / RHEL:**
```bash
sudo dnf install alsa-lib-devel libudev-devel libX11-devel
```

### Windows

Unzip `installation/windows.zip` and double-click `Slimes TD.exe`. No install needed. Windows Defender may show a SmartScreen warning on first launch = click **More info → Run anyway**.

---

## Building from source

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain, 2021 edition)
- On Linux: the system libraries listed above

```bash
git clone <repo-url>
cd SlimesTD
cargo run
```

The first build takes a few minutes (Bevy is large). Subsequent builds are fast thanks to `opt-level = 1` for the dev profile.

### WebAssembly (WASM)

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```

The `.cargo/config.toml` already sets `wasm-server-runner` as the runner for the WASM target, so `cargo run` opens a local browser tab automatically.

---

## How to play

### Goal

Prevent slimes from reaching your base. You start with a set amount of gold and base health. Each slime that reaches the end costs you health. Survive all 10 waves to win.

### Controls

| Input                      | Action |
|----------------------------|--------|
| Left click on tower button | Pick up tower to place |
| Left click on map          | Place tower |
| Right click                | Cancel placement |
| `1` - `0`                  | Keyboard shortcut for each tower (same order as the button bar) |
| Left click on placed tower | Open upgrade / sell panel |
| `G` | Toggle snap mode (aligns to tower axes and tile edges) |

### Maps

Choose from 2 maps on the map selection screen. Maps differ in path layout and the amount of buildable water tiles.

### Tower placement rules

- Towers **cannot** be placed on the path, spawn, or end tiles.
- Each tower type has terrain requirements:

| Tower | Allowed terrain |
|-------|----------------|
| Nature Wizard | Grass |
| Fire Wizard | Grass |
| Ice Wizard | Grass or Water |
| Dark Wizard | Grass |
| Mage Wizard | Water only |
| Archmage | Grass |
| Farm (Passive) | Grass |
| Farm (Kill) | Grass |
| Farm (Wave) | Grass |
| Farm (Hunter) | Grass |

If you try to place somewhere invalid, a red error message appears at the top of the screen explaining why.

---

## Towers

| # | Tower | Price | Role |
|---|-------|-------|------|
| 1 | Nature Wizard | $100 | Balanced starter |
| 2 | Fire Wizard | $125 | High damage, fast projectiles |
| 3 | Ice Wizard | $100 | Rapid-fire, low per-shot damage |
| 4 | Dark Wizard | $150 | Pierces through multiple enemies |
| 5 | Mage Wizard | $175 | Slow but hits extremely hard (water only) |
| 6 | Archmage | $250 | Best stats across the board |
| 7 | Farm (Passive) | $125 | Earns $50 every 15 seconds |
| 8 | Farm (Kill) | $150 | Earns $5 per enemy killed anywhere on map |
| 9 | Farm (Wave) | $200 | Earns $75 bonus when a wave is cleared |
| 0 | Farm (Hunter) | $175 | Shoots enemies; earns gold per personal kill |

Each combat tower has **3 upgrade paths** with **5 levels each** (damage, attack speed, range/pierce). Farms upgrade their income rate.

### Targeting priorities

Click a placed tower to change its targeting: **First**, **Last**, **Strongest**, **Weakest**, **Closest**. Sell a tower for 1/3 of total gold spent on it.

---

## Enemies

8 slime types, each a different color with increasing health. Waves escalate in difficulty, mixing types and tightening spawn intervals.

| Slime | Health |
|-------|--------|
| Green | 1 |
| Yellow | 2 |
| Pink | 3 |
| White | 4 |
| Blue | 5 |
| Orange | 6 |
| Purple | 7 |
| Red | 8 |

---

## Project structure

```
src/
  main.rs          = app setup, plugins, asset loading
  map/             = tilemap, pathfinding, camera
  tower/           = tower logic, placement, upgrades, UI
  enemy/           = enemy spawning, movement, waves
  gameplay_ui/     = HUD (gold, health, wave counter)
  main_menu/       = main menu, map selection, game state
assets/data/       = RON balance files (edit to tune the game)
installation/      = pre-built Windows and Linux binaries
```

Balance lives in `assets/data/` RON files = tweak stats, waves, and upgrade paths without recompiling.

---

## Tech

- **Engine:** Bevy 0.10.1
- **Language:** Rust 2021
- **Serialization:** RON (Rusty Object Notation) via `bevy_common_assets`
- **Asset loading:** `bevy_asset_loader`
- **WASM runner:** `wasm-server-runner`
- **Inspector:** `bevy-inspector-egui` (always on, press nothing = it's the floating panel)
