//Crates
use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

//Modules
mod map;
pub use map::*;
mod rect;
pub use rect::Rect;
mod player;
use player::*;
mod components;
pub use components::*;
mod visibility_system;
use visibility_system::*;
mod monster_ai_system;
use monster_ai_system::*;

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        // let mut lw = LeftWalker {};
        // lw.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let postions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&postions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    Paused,
    Running,
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("My Rustlike")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    //Initialize components with ecs
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    //Resource Initialization
    let map: Map = Map::new_map();
    let (player_x, player_y) = map.rooms[0].center();

    //Making some entities
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: 2,
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Venturer".to_string(),
        })
        .build();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let fg: rltk::RGB;
        let name: String;

        let roll = rng.range(0, 4);
        match roll {
            0 => {
                glyph = rltk::to_cp437('B');
                fg = RGB::named(rltk::CORNFLOWER_BLUE);
                name = "Bard".to_string();
            }
            1 => {
                glyph = rltk::to_cp437('g');
                fg = RGB::named(rltk::LIGHTGREEN);
                name = "goblin".to_string();
            }
            2 => {
                glyph = rltk::to_cp437('o');
                fg = RGB::named(rltk::DARKGREEN);
                name = "orc".to_string();
            }
            3 => {
                glyph = rltk::to_cp437('s');
                fg = RGB::named(rltk::WEB_GREEN);
                name = "snake".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('?');
                fg = RGB::named(rltk::WEB_PURPLE);
                name = "Invalid".to_string();
            }
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg,
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .build();
    }

    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(map);

    //Game loop
    rltk::main_loop(context, gs)
}
