
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use notify::Watcher;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VIRTUAL_KEY,
};

use newbase::{App, LogicSystem};
use rhai::{AST, Dynamic, Engine, Module, Scope};
use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    sync::Mutex,
};

use newoverlay::imgui::*;

use crate::models::math::Vector3;
use crate::player::{Ability, Player};

// ── low-level key injection ───────────────────────────────────────────────

fn send_virtual_key(vk_code: u16, key_up: bool) -> bool {
    if vk_code == 0 {
        return false;
    }

    let flags = if key_up {
        KEYEVENTF_KEYUP
    } else {
        Default::default()
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(vk_code),
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) == 1 }
}

fn tap_virtual_key(vk_code: u16, hold_ms: u64) -> bool {
    if !send_virtual_key(vk_code, false) {
        return false;
    }

    if hold_ms > 0 {
        thread::sleep(Duration::from_millis(hold_ms));
    }

    send_virtual_key(vk_code, true)
}

// ── engine factory ────────────────────────────────────────────────────────

fn build_engine() -> Engine {
    let mut engine = Engine::new();

    engine.set_max_operations(50_000);
    engine.set_max_call_levels(20);
    engine.set_max_array_size(10_000);
    engine.set_max_string_size(65_536);
    engine.set_max_map_size(10_000);

    // Vector3 ----------------------------------------------------------------
    engine.register_type_with_name::<Vector3>("Vector3");
    engine.register_get("x", |v: &mut Vector3| v.x as f64);
    engine.register_get("y", |v: &mut Vector3| v.y as f64);
    engine.register_get("z", |v: &mut Vector3| v.z as f64);

    // Ability ----------------------------------------------------------------
    engine.register_type_with_name::<Ability>("Ability");
    engine.register_get("cooling_down", |a: &mut Ability| a.cooling_down);
    engine.register_get("channeling",   |a: &mut Ability| a.channeling);
    // slot name as a debug string, e.g. "ESlot_Signature_1"
    engine.register_get("slot",         |a: &mut Ability| format!("{:?}", a.slot));
    // slot numeric id: ESlot_Signature_1 == 0, etc.
    engine.register_get("slot_id",      |a: &mut Ability| a.slot as i64);

    // Player -----------------------------------------------------------------
    engine.register_type_with_name::<Player>("Player");
    engine.register_get("pos",        |p: &mut Player| p.pos);
    engine.register_get("view_yaw",   |p: &mut Player| p.view_yaw as f64);
    engine.register_get("alive",      |p: &mut Player| p.alive);
    engine.register_get("health",     |p: &mut Player| p.health as i64);
    engine.register_get("max_health", |p: &mut Player| p.max_health as i64);
    engine.register_get("is_local",   |p: &mut Player| p.is_local);
    engine.register_get("team_id",    |p: &mut Player| p.team_id as i64);
    // hero name as string, e.g. "Mina", "Infernus"
    engine.register_get("hero",       |p: &mut Player| p.hero.to_string());
    engine.register_get("abilities",  |p: &mut Player| -> rhai::Array {
        p.abilities.iter().map(|a| Dynamic::from(*a)).collect()
    });

    // input module -----------------------------------------------------------
    // scripts: input::tap_key(vk_code, hold_ms) -> bool
    //          input::send_key(vk_code, key_up)  -> bool
    // set_native_fn requires F=true (fallible), so closures must return Result.
    let mut input_mod = Module::new();
    input_mod.set_native_fn("tap_key", |vk: i64, hold_ms: i64| -> Result<bool, Box<rhai::EvalAltResult>> {
        Ok(tap_virtual_key(vk as u16, hold_ms as u64))
    });
    input_mod.set_native_fn("send_key", |vk: i64, key_up: bool| -> Result<bool, Box<rhai::EvalAltResult>> {
        Ok(send_virtual_key(vk as u16, key_up))
    });
    engine.register_static_module("input", Arc::new(input_mod));

    // console module ---------------------------------------------------------
    // scripts: console::info(msg) / warn(msg) / error(msg)
    let mut console_mod = Module::new();
    console_mod.set_native_fn("info",  |msg: &str| -> Result<(), Box<rhai::EvalAltResult>> { println!("[script] {}",  msg); Ok(()) });
    console_mod.set_native_fn("warn",  |msg: &str| -> Result<(), Box<rhai::EvalAltResult>> { println!("[script] {}",  msg); Ok(()) });
    console_mod.set_native_fn("error", |msg: &str| -> Result<(), Box<rhai::EvalAltResult>> { println!("[script] {}", msg); Ok(()) });
    engine.register_static_module("console", Arc::new(console_mod));

    engine
}

// ── lifecycle hook helpers ────────────────────────────────────────────────

/// Rhai's call_fn executes module-level statements before invoking the
/// target function.  Inject zero/sentinel values for every host-provided
/// variable so the tick code (`if !local_player.alive { return; }` etc.)
/// can run safely before the first real tick populates the scope.
fn ensure_scope_sentinels(scope: &mut Scope) {
    scope.set_or_push("players",      rhai::Array::new());
    scope.set_or_push("local_player", Player::default());
    scope.set_or_push("timestamp_ms", 0i64);
    scope.set_or_push("window_w",     0i64);
    scope.set_or_push("window_h",     0i64);
}

fn call_hook(engine: &Engine, scope: &mut Scope, ast: &AST, name: &str) {
    ensure_scope_sentinels(scope);
    if let Err(e) = engine.call_fn::<()>(scope, ast, name, ()) {
        if !matches!(*e, rhai::EvalAltResult::ErrorFunctionNotFound(_, _)) {
            log::warn!("[script hook '{}'] {}", name, e);
        }
    }
}

// ── metadata / stats / state ─────────────────────────────────────────────

#[derive(Default)]
pub struct ScriptMeta {
    pub display_name: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
}

fn parse_meta(source: &str) -> ScriptMeta {
    let mut meta = ScriptMeta::default();
    for line in source.lines().take_while(|l| l.starts_with("//!")) {
        let content = line.trim_start_matches("//!").trim();
        if let Some((key, val)) = content.split_once(':') {
            match key.trim() {
                "name"        => meta.display_name = Some(val.trim().to_string()),
                "author"      => meta.author       = Some(val.trim().to_string()),
                "version"     => meta.version      = Some(val.trim().to_string()),
                "description" => meta.description  = Some(val.trim().to_string()),
                _ => {}
            }
        }
    }
    meta
}

#[derive(Default)]
pub struct ScriptStats {
    pub last_tick_us: u64,
    pub avg_tick_us: f64,
    pub total_ticks: u64,
    pub peak_tick_us: u64,
}

pub enum ScriptState {
    Active,
    Disabled,
    Faulted {
        error: String,
        since: std::time::Instant,
    },
}

struct Script {
    pub name: String,
    pub metadata: ScriptMeta,
    pub stats: ScriptStats,
    pub path: PathBuf,
    pub ast: Arc<AST>,
    pub scope: Scope<'static>,
    pub state: ScriptState,
}

// ── watcher helper ────────────────────────────────────────────────────────

fn script_name_from_event(event: &notify::Result<notify::Event>) -> Option<String> {
    let event = event.as_ref().ok()?;

    match event.kind {
        notify::EventKind::Modify(_)
        | notify::EventKind::Create(_)
        | notify::EventKind::Remove(_)
        | notify::EventKind::Any => {}
        _ => return None,
    }

    event
        .paths
        .iter()
        .find(|p| p.extension().and_then(|e| e.to_str()) == Some("rhai"))
        .and_then(|p| p.file_stem())
        .map(|s| s.to_string_lossy().into_owned())
}

// ── scripting system ──────────────────────────────────────────────────────

struct ScriptingSystem {
    engine: Engine,
    scripts: HashMap<String, Script>,
    reload_queue: Arc<Mutex<Vec<String>>>,
    _watcher: notify::RecommendedWatcher,
    start_time: std::time::Instant,
}

impl ScriptingSystem {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let engine = build_engine();

        let mut scripts = Self::load_all(&engine, "scripts/".into())?;

        // Fire on_load for every script that defines it.
        for script in scripts.values_mut() {
            call_hook(&engine, &mut script.scope, &script.ast, "on_load");
        }

        let queue = Arc::new(Mutex::new(Vec::<String>::new()));
        let watcher_queue = Arc::clone(&queue);

        let mut watcher = notify::recommended_watcher(move |event| {
            if let Some(name) = script_name_from_event(&event) {
                watcher_queue.lock().unwrap().push(name);
            }
        })?;
        watcher.watch("scripts/".as_ref(), notify::RecursiveMode::NonRecursive)?;

        Ok(Self {
            engine,
            scripts,
            reload_queue: queue,
            _watcher: watcher,
            start_time: std::time::Instant::now(),
        })
    }

    fn compile_script(
        engine: &Engine,
        path: &PathBuf,
    ) -> Result<(Arc<AST>, ScriptMeta), Box<dyn Error>> {
        let source = std::fs::read_to_string(path)?;
        let ast = Arc::new(engine.compile(&source)?);
        let metadata = parse_meta(&source);
        Ok((ast, metadata))
    }

    fn load_all(engine: &Engine, dir: PathBuf) -> Result<HashMap<String, Script>, Box<dyn Error>> {
        // Create the directory if it doesn't exist yet.
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        let mut scripts = HashMap::new();
        for entry in std::fs::read_dir(&dir)?.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("rhai") {
                continue;
            }
            let name = path.file_stem().unwrap().to_string_lossy().into_owned();
            let (ast, metadata) = Self::compile_script(engine, &path)?;
            scripts.insert(
                name.clone(),
                Script {
                    name,
                    metadata,
                    stats: ScriptStats::default(),
                    path,
                    ast,
                    scope: Scope::new(),
                    state: ScriptState::Active,
                },
            );
        }
        Ok(scripts)
    }

    pub fn load(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        let (ast, metadata) = Self::compile_script(&self.engine, &path)?;
        let mut script = Script {
            name: name.clone(),
            metadata,
            stats: ScriptStats::default(),
            path,
            ast,
            scope: Scope::new(),
            state: ScriptState::Active,
        };
        call_hook(&self.engine, &mut script.scope, &script.ast, "on_load");
        self.scripts.insert(name, script);
        Ok(())
    }

    pub fn unload(&mut self, name: &str) {
        if let Some(mut script) = self.scripts.remove(name) {
            call_hook(&self.engine, &mut script.scope, &script.ast, "on_unload");
        }
    }

    pub fn enable(&mut self, name: &str) {
        if let Some(script) = self.scripts.get_mut(name) {
            script.state = ScriptState::Active;
        }
    }

    pub fn disable(&mut self, name: &str) {
        if let Some(script) = self.scripts.get_mut(name) {
            script.state = ScriptState::Disabled;
        }
    }

    pub fn reload(&mut self, name: &str, preserve_state: bool) -> Result<(), Box<dyn Error>> {
        if let Some(script) = self.scripts.get_mut(name) {
            call_hook(&self.engine, &mut script.scope, &script.ast, "on_unload");

            let (ast, metadata) = Self::compile_script(&self.engine, &script.path)?;
            script.ast = ast;
            script.metadata = metadata;
            script.state = ScriptState::Active;
            if !preserve_state {
                script.scope = Scope::new();
            }

            call_hook(&self.engine, &mut script.scope, &script.ast, "on_load");
        }
        Ok(())
    }
}

// ── LogicSystem impl ──────────────────────────────────────────────────────

impl LogicSystem<crate::AppData> for ScriptingSystem {
    fn name(&self) -> &'static str {
        "Scripting"
    }

    fn tick(&mut self, app: &mut App<crate::AppData>, _ui: &Ui, _draw_list: &DrawListMut) {
        // Drain the watcher queue; recompile changed scripts before any run this tick.
        let pending: Vec<String> = self.reload_queue.lock().unwrap().drain(..).collect();
        for name in pending {
            if let Err(e) = self.reload(&name, true) {
                log::warn!("[{}] compile error on reload: {}", name, e);
            }
        }

        let window_w = app.window_info.size.0.max(0) as i64;
        let window_h = app.window_info.size.1.max(0) as i64;
        let timestamp_ms = self.start_time.elapsed().as_millis() as i64;

        let players = app.state.player_buf.read().to_vec();

        // Use a default (dead) Player as the "no local player" sentinel.
        // Scripts guard with `if !local_player.alive { return; }`.
        let local_player: Player = players
            .iter()
            .find(|p| p.is_local)
            .cloned()
            .unwrap_or_default();

        // Convert players to a Rhai Array so scripts can iterate with `for`.
        let players_array: rhai::Array = players.into_iter().map(Dynamic::from).collect();

        for script in self.scripts.values_mut() {
            if matches!(
                script.state,
                ScriptState::Disabled | ScriptState::Faulted { .. }
            ) {
                continue;
            }

            // Inject fresh per-tick globals.  set_or_push updates existing
            // bindings in place so persistent script variables are not lost.
            script.scope.set_or_push("players",      players_array.clone());
            script.scope.set_or_push("local_player", local_player.clone());
            script.scope.set_or_push("timestamp_ms", timestamp_ms);
            script.scope.set_or_push("window_w",     window_w);
            script.scope.set_or_push("window_h",     window_h);

            let start = std::time::Instant::now();

            match self
                .engine
                .run_ast_with_scope(&mut script.scope, &script.ast)
            {
                Ok(_) => {
                    let elapsed = start.elapsed().as_micros() as u64;
                    let stats = &mut script.stats;
                    stats.last_tick_us = elapsed;
                    stats.peak_tick_us = stats.peak_tick_us.max(elapsed);
                    stats.total_ticks += 1;
                    stats.avg_tick_us +=
                        (elapsed as f64 - stats.avg_tick_us) / stats.total_ticks as f64;
                }
                Err(e) => {
                    script.state = ScriptState::Faulted {
                        error: e.to_string(),
                        since: std::time::Instant::now(),
                    };
                    log::warn!("[{}] faulted: {}", script.name, e);
                }
            }
        }
    }
}

pub fn system() -> Result<impl LogicSystem<crate::AppData>, Box<dyn Error>> {
    Ok(ScriptingSystem::new()?)
}
