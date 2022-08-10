//Crates
use rltk::{GameState, Rltk, RGB};
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

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let postions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&postions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("My Rustlike")
        .build()?;

    let mut gs = State { ecs: World::new() };
    //Initialize components with ecs
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    //Resource Initialization
    let (rooms, map) = new_map();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

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
        .build();

    // for i in 0..10 {
    //     gs.ecs
    //     .create_entity()
    //     .with(Position { x: i * 7, y: 20 })
    //     .with(Renderable {
    //         glyph: rltk::to_cp437('☺'),
    //         fg: RGB::named(rltk::RED),
    //         bg: RGB::named(rltk::BLACK),
    //     })
    //     .with(LeftMover{})
    //     .build();
    // }

    //Game loop
    rltk::main_loop(context, gs)
}
