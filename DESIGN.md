# Flappy Bird: Design

Bevy 0.20. Build the `Playing` state first, then menus/pause.

## Game states

```
┌───────────┐  press start  ┌───────────┐   press P    ┌───────────┐
│ MainMenu  │──────────────▶│  Playing  │◀────────────▶│  Paused   │
└───────────┘               └─────┬─────┘              └───────────┘
      ▲                           │ bird hits pipe/ground
      │                           ▼
      │    press restart    ┌───────────┐
      └─────────────────────│ GameOver  │
                            └───────────┘
   (leaving Playing despawns all game entities automatically)
```

## ECS refresher

- **Entity**: a thing that exists. Just an ID.
- **Component**: data attached to an entity.
- **Resource**: global data, only one copy.
- **System**: a function that runs every frame and queries components/resources.
- **Plugin**: a way to group related systems in one place (organization only).

## Entities and components

```
Bird ─────────── Bird · Transform · Sprite · Velocity
Bottom pipe ──── Pipe · Transform · Sprite · Velocity · ScoreZone
Top pipe ─────── Pipe · Transform · Sprite · Velocity
Score text ───── ScoreText · Text
Camera ───────── Camera2d
```

After the bird passes a bottom pipe, that pipe also gets `Scored`.

- Each obstacle = **two** pipe entities (top + bottom) with a gap between.
- `Transform.translation` is the **center** of the sprite.

## Resources

```
Score ────────── current score
PipeSpawnTimer ─ when to spawn the next pair
```

## Types

- **Ours:** `Bird`, `Pipe`, `ScoreZone`, `Scored`, `ScoreText` (markers), `Velocity`, `Score`, `PipeSpawnTimer`, `GameState` (enum)
- **Built into Bevy:** `Transform`, `Sprite`, `Text`, `Camera2d`

## Plugins and systems

```
MovementPlugin
 └─ apply_velocity ........ Transform += Velocity × time  (bird AND pipes)

BirdPlugin
 ├─ spawn_bird ............ on entering Playing
 ├─ flap .................. key pressed → set Velocity upward
 ├─ apply_gravity ......... pull bird Velocity down every frame
 └─ check_collisions ...... bird overlaps pipe or ground → GameOver

PipePlugin
 ├─ spawn_pipes ........... timer fires → spawn top + bottom with a gap
 └─ despawn_offscreen ..... pipe x past left edge → despawn

ScorePlugin
 ├─ check_score ........... ScoreZone without Scored, left of bird
 │                          → Score += 1, insert Scored
 └─ update_score_text ..... write Score into the ScoreText's Text

StatePlugin / UI
 ├─ menus / game over screens
 └─ reset Score on new game
```

## One frame of Playing

```
input ──▶ flap ──▶ apply_gravity ──▶ apply_velocity ──▶ check_collisions
                                                     ├──▶ check_score ──▶ update_score_text
                                                     └──▶ despawn_offscreen
spawn_pipes (whenever the timer fires)
```

Bevy renders everything after our systems run.

## Design decisions

- **Group plugins by thing, not action.** No "spawning" plugin; each plugin spawns its own entities.
- **Velocity is shared.** One `apply_velocity` system moves bird and pipes.
- **Bird x never changes.** Pipes move toward it. "Passed a pipe" = pipe x is left of bird x (adjusted for widths).
- **Score once per obstacle.** *(As built)* Top pipes are excluded with `Without<TopPipe>`. After scoring, insert `ScoredAlready`; the query uses `Without<ScoredAlready>`.
- **Scoring uses a message.** Anything that earns a point sends `PointScored`; only `increase_score` touches `GameScore`. Coins etc. just send the message.
- **Despawn by reason, not by entity.** Off-screen pipes are despawned by a system; everything else is tagged `DespawnOnExit(state)` and cleaned up on state change.

## Build order

- [x] 1. Window, camera, bird square
- [x] 2. Gravity and velocity, so the bird falls
- [x] 3. Flap on spacebar
- [x] 4. Pipes spawning and scrolling
- [x] 5. Despawn off-screen pipes
- [x] 6. Collision → game over
- [x] 7. Scoring and score text
- [ ] 8. States: ~~restart~~ ✓, main menu, pause

## Next: fix and polish

**Gameplay fixes**
- [x] Die when flying off the top/bottom of the screen
- [x] ~~Short input delay on game over~~ Restart uses R instead of Space, so a dying flap can't restart
- [x] PreGame "Press Space to start" state before each round
- [x] Despawn pipes only when fully offscreen (`left_edge - PIPE_WIDTH / 2`)

**Code cleanup**
- [ ] Move `rects_overlap` out of `bird.rs` (TODO in code)
- [ ] Remove leftover `println!`s and old comments

**Feel and looks**
- [ ] Camera scaling (fixed game size regardless of window size)
- [ ] Properly centered UI text
- [ ] Images instead of colored rectangles
- [ ] Tilt the bird based on its velocity
- [ ] Sound effects (flap, score, hit)
- [ ] High score (resource that survives restarts; later, saved to a file)
