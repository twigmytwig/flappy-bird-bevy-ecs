# Flappy Bird (Bevy ECS)

A Flappy Bird clone written in Rust with the [Bevy](https://bevy.org) game engine (0.20).
Built as a first project for learning Rust and Bevy's Entity Component System.

## Run it

```
cargo run
```

The first build compiles all of Bevy and takes a while. Later builds are fast.

## Controls

| Key | Action |
|---|---|
| Space | Start a round / flap |
| R | Restart after game over |

## How to play

Fly through the gaps between pipes. Each pipe you pass is one point.
Hitting a pipe or the top/bottom of the screen ends the round.
Your best score is saved to `save.ron` and kept between runs.

## Project layout

```
src/
├── main.rs       App setup: adds every plugin, spawns the camera
├── state.rs      GameState (PreGame / Playing / GameOver), menus, GameOverSet ordering
├── bird.rs       Bird: spawning, flap, gravity, collision, passing pipes
├── pipe.rs       Pipes: random gap layout, timed spawning, offscreen despawn
├── movement.rs   Velocity component and the system that moves anything with one
└── scoring.rs    Score, high score, save file, score text
```

See [DESIGN.md](DESIGN.md) for how the pieces fit together and why.
