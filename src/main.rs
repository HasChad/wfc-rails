use ::rand::{seq::SliceRandom, thread_rng, Rng};
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets},
};
use std::collections::HashMap;

mod app_settings;
mod wfc_functions;

use app_settings::*;
use wfc_functions::*;

const ROW: usize = 10;
const COLUMN: usize = 15;
const TEXTURE_SIZE: f32 = 64.;
const GRID_SIZE: usize = ROW * COLUMN;
const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;
const EDGE_COUNT: i32 = 2;
const TEXTURE_PARAM: DrawTextureParams = DrawTextureParams {
    dest_size: Some(Vec2 {
        x: TEXTURE_SIZE,
        y: TEXTURE_SIZE,
    }),
    source: None,
    rotation: 0.,
    flip_x: false,
    flip_y: false,
    pivot: None,
};

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
enum Tile {
    UnderCons,
    Empty,
    All,
    Horizontal,
    Vertical,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    LeftRightDown1,
    LeftRightDown2,
    LeftRightUp1,
    LeftRightUp2,
}

#[derive(Clone, PartialEq)]
enum Cell {
    Options(Vec<Tile>),
    Collapsed(TileProp),
}

#[derive(Clone, PartialEq)]
struct TileProp {
    tile: Tile,
    edges: Vec<i32>,
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    set_default_filter_mode(FilterMode::Nearest);
    let mut rng = thread_rng();

    // load rail textures
    let empty_texture = load_texture("empty.png").await.unwrap();
    let uc_sign_texture = load_texture("under_cons.png").await.unwrap();
    let rail_all_texture = load_texture("rail_all.png").await.unwrap();
    let rail_h_texture = load_texture("rail_h.png").await.unwrap();
    let rail_v_texture = load_texture("rail_v.png").await.unwrap();
    let rail_ld_texture = load_texture("rail_ld.png").await.unwrap();
    let rail_lu_texture = load_texture("rail_lu.png").await.unwrap();
    let rail_rd_texture = load_texture("rail_rd.png").await.unwrap();
    let rail_ru_texture = load_texture("rail_ru.png").await.unwrap();
    let rail_lrd1_texture = load_texture("rail_lrd1.png").await.unwrap();
    let rail_lrd2_texture = load_texture("rail_lrd2.png").await.unwrap();
    let rail_lru1_texture = load_texture("rail_lru1.png").await.unwrap();
    let rail_lru2_texture = load_texture("rail_lru2.png").await.unwrap();

    // create tiles and edges
    let cells = HashMap::from([
        (Tile::UnderCons, vec![0, 0, 0, 0]),
        (Tile::Empty, vec![0, 0, 0, 0]),
        (Tile::All, vec![1, 1, 1, 1]),
        (Tile::Horizontal, vec![0, 1, 0, 1]),
        (Tile::Vertical, vec![1, 0, 1, 0]),
        (Tile::LeftDown, vec![0, 0, 1, 1]),
        (Tile::LeftUp, vec![1, 0, 0, 1]),
        (Tile::RightDown, vec![0, 1, 1, 0]),
        (Tile::RightUp, vec![1, 1, 0, 0]),
        (Tile::LeftRightDown1, vec![0, 1, 1, 1]),
        (Tile::LeftRightDown2, vec![0, 1, 1, 1]),
        (Tile::LeftRightUp1, vec![1, 1, 0, 1]),
        (Tile::LeftRightUp2, vec![1, 1, 0, 1]),
    ]);

    // create options
    let tile_options = vec![
        Tile::UnderCons,
        Tile::Empty,
        Tile::All,
        Tile::Horizontal,
        Tile::Vertical,
        Tile::LeftDown,
        Tile::LeftUp,
        Tile::RightDown,
        Tile::RightUp,
        Tile::LeftRightDown1,
        Tile::LeftRightDown2,
        Tile::LeftRightUp1,
        Tile::LeftRightUp2,
    ];

    // create grid
    let mut grid = vec![Cell::Options(tile_options.clone()); GRID_SIZE];

    // choose random tile for start
    let mut choosen_cell = rng.gen_range(0..GRID_SIZE);
    let mut choosen_cell_tile = tile_options.choose(&mut rng).unwrap();

    grid[choosen_cell] = Cell::Collapsed(TileProp {
        tile: choosen_cell_tile.clone(),
        edges: cells[choosen_cell_tile].clone(),
    });

    loop {
        // ! MARK: FPS limiter
        /*
        let minimum_frame_time = 1. / 5.; // 60 FPS
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
        */

        // ! MARK: UI
        /*
        let ui_windows_size = Vec2::new(150., 100.);
        let ui_windows_pos = Vec2::new(25., 25.);

        widgets::Window::new(hash!(), ui_windows_pos, ui_windows_size)
            //.movable(false)
            .label("World Config")
            .ui(&mut root_ui(), |_ui| {});
        */

        // ! MARK: Enterance
        if is_key_pressed(KeyCode::A) {
            grid = vec![Cell::Options(tile_options.clone()); GRID_SIZE];

            choosen_cell = rng.gen_range(0..GRID_SIZE);
            choosen_cell_tile = tile_options.choose(&mut rng).unwrap();

            grid[choosen_cell] = Cell::Collapsed(TileProp {
                tile: choosen_cell_tile.clone(),
                edges: cells[choosen_cell_tile].clone(),
            });
        }

        // ! MARK: WFC Part 1: Wave
        wave_funtion(&mut grid, &cells);

        // ! MARK: Check for least option one
        let mut least_one = 0;
        let mut least_num = 100;

        for (num, tile) in grid.iter().enumerate() {
            match tile {
                Cell::Options(options) => {
                    if options.len() < least_num {
                        least_num = options.len();
                        least_one = num;
                    }
                }
                Cell::Collapsed(_) => continue,
            }
        }

        // ! MARK: WFC Part 2: Collapse
        if let Cell::Options(options) = &grid[least_one] {
            if let Some(damn) = options.choose(&mut rng) {
                let choosen = damn;

                grid[least_one] = Cell::Collapsed(TileProp {
                    tile: choosen.clone(),
                    edges: cells[choosen].clone(),
                });
            } else {
                grid[least_one] = Cell::Collapsed(TileProp {
                    tile: Tile::UnderCons,
                    edges: vec![0, 0, 0, 0],
                })
            }
        }

        // ! MARK: Draw world
        for (index, cell) in grid.iter().enumerate() {
            let x = (index % COLUMN) as f32 * TEXTURE_SIZE;
            let y = (index / COLUMN) as f32 * TEXTURE_SIZE;

            match cell {
                Cell::Options(_) => draw_texture_ex(&uc_sign_texture, x, y, WHITE, TEXTURE_PARAM),
                Cell::Collapsed(cell) => match cell.tile {
                    Tile::UnderCons => {
                        draw_texture_ex(&uc_sign_texture, x, y, WHITE, TEXTURE_PARAM)
                    }
                    Tile::Empty => draw_texture_ex(&empty_texture, x, y, WHITE, TEXTURE_PARAM),
                    Tile::All => draw_texture_ex(&rail_all_texture, x, y, WHITE, TEXTURE_PARAM),
                    Tile::Horizontal => {
                        draw_texture_ex(&rail_h_texture, x, y, WHITE, TEXTURE_PARAM)
                    }
                    Tile::Vertical => draw_texture_ex(&rail_v_texture, x, y, WHITE, TEXTURE_PARAM),
                    Tile::LeftDown => draw_texture_ex(&rail_ld_texture, x, y, WHITE, TEXTURE_PARAM),
                    Tile::LeftUp => draw_texture_ex(&rail_lu_texture, x, y, WHITE, TEXTURE_PARAM),
                    Tile::RightDown => {
                        draw_texture_ex(&rail_rd_texture, x, y, WHITE, TEXTURE_PARAM)
                    }
                    Tile::RightUp => draw_texture_ex(&rail_ru_texture, x, y, WHITE, TEXTURE_PARAM),
                    Tile::LeftRightDown1 => {
                        draw_texture_ex(&rail_lrd1_texture, x, y, WHITE, TEXTURE_PARAM)
                    }
                    Tile::LeftRightDown2 => {
                        draw_texture_ex(&rail_lrd2_texture, x, y, WHITE, TEXTURE_PARAM)
                    }
                    Tile::LeftRightUp1 => {
                        draw_texture_ex(&rail_lru1_texture, x, y, WHITE, TEXTURE_PARAM)
                    }
                    Tile::LeftRightUp2 => {
                        draw_texture_ex(&rail_lru2_texture, x, y, WHITE, TEXTURE_PARAM)
                    }
                },
            }
        }

        next_frame().await;
    }
}
