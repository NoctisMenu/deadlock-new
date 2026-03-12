//! name: Mina Cone 1
//! description: Triggers key 1 when playing Mina and an enemy is inside a frontal cone

// ── persistent state ───────────────────────────────────────────────────────
// is_def_var guard ensures this runs exactly once; the binding survives ticks.
if !is_def_var("last_tap_ms") { let last_tap_ms = 0; }

// ── helpers ────────────────────────────────────────────────────────────────

// Returns true if the Signature 1 ability exists and is off cooldown.
fn is_key1_ready(lp) {
    for ab in lp.abilities {
        if ab.slot == "ESlot_Signature_1" || ab.slot_id == 0 {
            return !ab.cooling_down && !ab.channeling;
        }
    }
    false
}

// Returns true when `enemy` is within RANGE and inside the CONE_DEG frontal arc.
// Constants are local to avoid module-scope capture issues.
fn in_front_cone(lp, enemy) {
    let RANGE       = 393.7007874;  // 10 m × 39.37 units/m
    let CONE_COS    = 0.99144486;   // cos(7.5°) — half of 15° full cone

    let dx = enemy.pos.x - lp.pos.x;
    let dy = enemy.pos.y - lp.pos.y;
    let dz = enemy.pos.z - lp.pos.z;

    let dist_3d = sqrt(dx*dx + dy*dy + dz*dz);
    if dist_3d < 0.001 || dist_3d > RANGE { return false; }

    let dist_2d = sqrt(dx*dx + dy*dy);
    if dist_2d < 0.001 { return false; }

    // view_yaw is in degrees — convert to radians inline.
    let yaw_rad = lp.view_yaw * 0.017453292519943295;
    let dot = cos(yaw_rad) * dx / dist_2d
            + sin(yaw_rad) * dy / dist_2d;
    dot >= CONE_COS
}

// ── lifecycle ──────────────────────────────────────────────────────────────
fn on_load()   { console::info("mina_cone_1 loaded");   }
fn on_unload() { console::info("mina_cone_1 unloaded"); }

// ── per-tick logic ─────────────────────────────────────────────────────────
// Scope variables injected by the host each tick:
//   local_player : Player  (default/dead sentinel when no local player exists)
//   players      : Array   (all players this tick)
//   timestamp_ms : i64
//   window_w/h   : i64
//
// No top-level `let` here — avoids accumulating scope entries across ticks.

if !local_player.alive || local_player.health <= 0 { return; }
if local_player.hero != "Mina" { return; }
if !is_key1_ready(local_player) { return; }
if timestamp_ms - last_tap_ms < 180 { return; }  // TAP_COOLDOWN_MS = 180

for p in players {
    if !p.is_local && p.alive && p.health > 0 && p.team_id != local_player.team_id {
        if in_front_cone(local_player, p) {
            if input::tap_key(0x31, 10) {   // KEY_1_VK = 0x31, hold 10 ms
                last_tap_ms = timestamp_ms;
            }
            return;
        }
    }
}
