# Slimes TD

A tower defense game where waves of colored slimes march along a path toward your base. Place wizard towers, earn gold, upgrade your defenses, and survive all 10 waves to win.

Built with [Bevy 0.10.1](https://bevyengine.org/) and Rust.

**Itch.io (play in browser or download):** https://iliyangavrilov.itch.io/slimestd

---

## Download & play

Pre-built bundles for **Windows, Linux and macOS** are published on the
[**Releases** page](../../releases). Each release has a `SlimesTD-<platform>.zip` —
download the one for your OS, unzip it, and run the game inside. No installation needed.

| Platform | After unzipping |
|----------|-----------------|
| Windows  | Run `Slimes TD.exe`. Windows SmartScreen may warn on first launch = click **More info → Run anyway**. |
| Linux    | Run `./"Slimes TD"` (see system libraries below). |
| macOS    | Run `Slimes TD`. It's unsigned, so right-click → **Open** the first time to bypass Gatekeeper. |
| Browser  | Play directly on [itch.io](https://iliyangavrilov.itch.io/slimestd) (WebAssembly) — no download. |

The release bundles are produced automatically by GitHub Actions
([`.github/workflows/release.yml`](.github/workflows/release.yml)) on native runners
for each OS, so they always match the source.

### Linux = first run

```bash
unzip SlimesTD-Linux.zip
cd "Slimes TD - Linux"
chmod +x "Slimes TD"   # if the zip didn't preserve it
./"Slimes TD"
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

Choose from 3 maps on the map selection screen. Maps differ in path layout and the amount of buildable water tiles.

- **Level 1 / Level 2** = single-path maps of increasing complexity.
- **Level 3** = an **adaptive two-lane map**. Slimes enter down one shared lane, and once the assault escalates (from wave 2) a **second lane opens** and the slimes split between both routes, forcing you to defend two fronts at once. A central water pool sits between the lanes for water-only towers.

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

| # | Tower | Price | On-hit effect / role |
|---|-------|-------|------|
| 1 | Nature Wizard | $100 | **Poison** = damage-over-time, balanced starter |
| 2 | Fire Wizard | $125 | **Splash** = area damage around the impact |
| 3 | Ice Wizard | $100 | **Slow** = rapid-fire, chills enemies (grass or water) |
| 4 | Dark Wizard | $150 | **Knockback** + **sees invisible enemies**, pierces 2 |
| 5 | Mage Wizard | $175 | **Stun** = slow but hits hard (water only) |
| 6 | Archmage | $250 | **Chain lightning** = arcs between enemies, best all-rounder |
| 7 | Farm (Passive) | $125 | Earns gold on a timer |
| 8 | Farm (Kill) | $150 | Earns gold per enemy killed anywhere on map |
| 9 | Farm (Wave) | $200 | Earns a bonus when a wave is cleared |
| 0 | Farm (Hunter) | $175 | Shoots enemies; earns gold per personal kill |

On-hit effects are **data-driven** = each tower's `effect` (Poison, Splash, Slow, Stun, Chain, Knockback) is declared in `assets/data/stats.tower_stats.ron`, so retuning or reassigning them is a config edit, not a code change.

Each combat tower has **3 upgrade paths** with **5 levels each** (damage, attack speed, range/pierce). Farms upgrade their income rate.

### Targeting priorities

Click a placed tower to cycle its targeting: **First**, **Last**, **Closest**, **Farthest**, **Strongest**, **Weakest**, **Random**. Sell a tower for 1/3 of total gold spent on it.

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

### Enemy traits = invisibility

Slimes can carry **traits** that are independent of their color. The first is **Invisibility**: an invisible slime renders semi-transparent and is **untargetable by every tower except the Dark Wizard** = direct shots, splash, and chain lightning from other towers all pass right through it. Traits are assigned **per spawn in the wave data**, not baked into the enemy type, so the same color can appear visible in one wave and invisible in another (e.g. White slimes are invisible in early waves but visible later). Build a Dark Wizard to counter them.

---

## Project structure

```
src/
  main.rs          = app setup, plugins, asset loading
  map/             = tilemap, multi-lane pathfinding, camera
  tower/           = tower logic, placement, upgrades, projectiles, targeting, UI
  enemy/           = enemy spawning, movement, waves, traits (invisibility)
  effects/         = on-hit effects (poison, splash, chain, knockback) + visual feedback
  audio/           = music + SFX channels, volume settings
  gameplay_ui/     = HUD (gold, health, wave counter)
  main_menu/       = main menu, map selection, settings, game state
  tutorial/        = first-launch interactive tutorial
  persistence/     = save/load of settings and progress (RON)
assets/data/       = RON balance files (edit to tune the game)
.github/workflows/ = CI that builds the Windows/Linux/macOS release bundles
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
