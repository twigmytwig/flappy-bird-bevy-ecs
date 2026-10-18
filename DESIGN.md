# Flappy Bird: Design

How the game is put together and why. Written for Bevy 0.20.

This file describes **purpose, rules and structure**. For exact details (which
systems exist, which set a system is in), the code is the source of truth:
search for the name.

## ECS refresher

- **Entity**: a thing that exists. Just an ID.
- **Component**: data attached to an entity.
- **Resource**: global data, only one copy.
- **Message**: a one-off "this happened" note one system sends and others read.
- **System**: a function Bevy runs (every frame, or on a state change) that reads/writes the above.
- **Plugin**: groups related systems and setup in one place (organization only).

## Game states

```
            ┌──────────┐   Space    ┌──────────┐
  start ───▶│ PreGame  │───────────▶│ Playing  │
            └──────────┘            └────┬─────┘
                 ▲                       │ hit pipe / screen edge
                 │         R        ┌────▼─────┐
                 └──────────────────│ GameOver │
                                    └──────────┘
```

| State | What happens |
|---|---|
| `PreGame` | Bird and score text spawn. Bird floats (no gravity). "Press Space to start". |
| `Playing` | Gravity, movement, pipes, collision and scoring all run. |
| `GameOver` | High score updated and saved, game over screen shown. Waits for R. |

The Space press that starts a round also flaps, because `flap` runs in both
`PreGame` and `Playing`.

## Plugins

| Plugin (file) | Owns |
|---|---|
| `StatePlugin` (`state.rs`) | `GameState`, `GameOverSet` ordering, PreGame/GameOver text, start/restart input |
| `BirdPlugin` (`bird.rs`) | Spawning the bird, flap, gravity, collision, detecting passed pipes |
| `PipePlugin` (`pipe.rs`) | Gap layout math, spawning pipe pairs on a timer, despawning offscreen pipes |
| `MovementPlugin` (`movement.rs`) | `Velocity` and `apply_velocity` (moves bird and pipes) |
| `ScoringPlugin` (`scoring.rs`) | Current score, high score, save file, score text |

## Entities

```
Bird ─────────── Bird · Velocity · Transform · Sprite · DespawnOnExit(Playing)
Bottom pipe ──── Pipe · Velocity · Transform · Sprite · DespawnOnExit(Playing)
Top pipe ─────── Pipe · TopPipe · Velocity · Transform · Sprite · DespawnOnExit(Playing)
Score text ───── ScoreText · Text · TextFont · Node · DespawnOnExit(Playing)
PreGame text ─── Text · TextFont · Node · DespawnOnExit(PreGame)
GameOver text ── Text · TextFont · Node · DespawnOnExit(GameOver)
Camera ───────── Camera2d
```

- A bottom pipe gets `ScoredAlready` once the bird passes it.
- `Transform.translation` is the **center** of a sprite.
- World coordinates: `(0, 0)` is the screen center, y points up. UI (`Node`)
  is measured from the window's top-left corner instead.

## Resources

| Resource | Purpose |
|---|---|
| `GameScore` | Points this round. Reset on entering `PreGame`. |
| `HighScore` | Best score ever. Loaded from `save.ron` at startup. |
| `PipeSpawnTimer` | Repeating timer; a pipe pair spawns each time it finishes. |

## Messages

| Message | Sent by | Read by |
|---|---|---|
| `PointScored` | `check_pipe_passed` (bird.rs) | `increase_score` (scoring.rs) |

Anything that should award a point (a future coin, for example) sends
`PointScored`. Only `increase_score` changes `GameScore`.

## System ordering

**`GameOverSet`** (defined and ordered in `state.rs`) splits entering `GameOver`
into phases that run in this order:

1. `UpdateScores`: the final score is known. Update high score, save data, stats.
2. `ShowUi`: build the game over screen, which can rely on the numbers being final.

To add a system to a phase, register it on `OnEnter(GameState::GameOver)` with
`.in_set(GameOverSet::...)`. Don't list members here; search for
`GameOverSet::` to see them.

Other systems run in no guaranteed order within a frame. That's fine so far
(at worst one frame of delay), but use sets or `.before()`/`.after()` if a new
system depends on another's result in the same frame.

## Save data

`save.ron` holds a `SaveData` struct, converted with serde + RON:

```
(
    high_score: 42,
)
```

- Loaded once at startup. Missing or invalid file → starts at 0 (warning if invalid).
- Written only when a round ends with a new high score.
- The resource is updated **before** writing, so the screen is right even if saving fails.
- Listed in `.gitignore`. Saved in the folder the game runs from.

## Design decisions

- **Group plugins by thing, not action.** No "spawning" plugin; each plugin spawns its own entities.
- **Shared velocity.** One `apply_velocity` system moves the bird and every pipe.
- **Bird x never changes.** Pipes move toward it. "Passed" = bird center x ≥ pipe center x.
- **One rectangle-overlap check (AABB).** Works for any pipe, top or bottom; no special cases.
- **Score once per obstacle.** Only bottom pipes count (`Without<TopPipe>`), and
  `ScoredAlready` + `Without<ScoredAlready>` stops a pipe from scoring twice.
- **Scoring via a message**, so new ways to score don't touch score code.
- **Despawn by reason, not by entity.** Offscreen pipes have a system; everything
  else is tagged `DespawnOnExit(state)` and cleaned up automatically.
- **Order plugins with sets, not function names**, so files depend on a shared
  agreement (`GameOverSet`) instead of each other's internals.
- **Restart on R, not Space**, so a panicked flap while dying can't skip the game over screen.

## Roadmap

**Done**
- [x] Bird, gravity, flap
- [x] Random pipes, scrolling, offscreen despawn
- [x] Collision with pipes and screen edges
- [x] Scoring with on-screen text
- [x] PreGame / Playing / GameOver states, restart
- [x] High score saved to disk

**Code cleanup**
- [ ] Move `rects_overlap` out of `bird.rs` (TODO in code)
- [x] Remove leftover `println!`s and old comments

**Features and polish**
- [ ] Pause
- [ ] Camera scaling (fixed game size regardless of window size)
- [ ] Properly centered UI text
- [ ] Images instead of colored rectangles
- [ ] Tilt the bird based on its velocity
- [ ] Sound effects (flap, score, hit)

## Future ideas

**Not started until the core game is solid, polished and fun.** These are
direction, not commitments.

- **Coins.** Collectible in the level. Picking one up adds to the score (send
  `PointScored`) *and* to a persistent coin balance (a new `SaveData` field).
- **Spending coins.** A shop between runs for perks, power-ups or cosmetics.
  Probably its own `GameState`.
- **Moving obstacles.** Pipes or hazards that move vertically, or new obstacle
  types. A `Velocity` with a y component may be most of it.
- **Boss fights.** A scripted encounter after some number of pipes.
- **Roguelike structure.** Runs made of stages, choosing upgrades between them,
  coins and unlocks carrying over between runs.
