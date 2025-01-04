use ::rand::{seq::SliceRandom, thread_rng, Rng};
use macroquad::prelude::*;
use std::collections::HashMap;

mod wfc_functions;

use wfc_functions::*;

const ROW: usize = 10;
const COLUMN: usize = 15;
const TEXTURE_SIZE: f32 = 64.;
const GRID_SIZE: usize = ROW * COLUMN;
const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;
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

#[derive(Clone, PartialEq)]
enum Tile {
    Options(Vec<i32>),
    Collapsed(Cell),
}

#[derive(Clone, PartialEq)]
struct Cell {
    tile: i32,
    edges: Vec<i32>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "WFC-Rails".into(),
        icon: None,
        window_width: COLUMN as i32 * TEXTURE_SIZE as i32,
        window_height: ROW as i32 * TEXTURE_SIZE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    set_default_filter_mode(FilterMode::Nearest);
    let mut rng = thread_rng();

    // load rail texture
    let rail_all_texture = load_texture("rail_all.png").await.unwrap();
    let rail_h_texture = load_texture("rail_h.png").await.unwrap();
    let rail_v_texture = load_texture("rail_v.png").await.unwrap();
    let rail_ld_texture = load_texture("rail_ld.png").await.unwrap();
    let rail_lu_texture = load_texture("rail_lu.png").await.unwrap();
    let rail_rd_texture = load_texture("rail_rd.png").await.unwrap();
    let rail_ru_texture = load_texture("rail_ru.png").await.unwrap();
    //
    let rail_lrd1_texture = load_texture("rail_lrd1.png").await.unwrap();
    let rail_lrd2_texture = load_texture("rail_lrd2.png").await.unwrap();
    let rail_lru1_texture = load_texture("rail_lru1.png").await.unwrap();
    let rail_lru2_texture = load_texture("rail_lru2.png").await.unwrap();

    // create tiles and edges
    let mut cells: HashMap<usize, Vec<i32>> = HashMap::new();
    cells.insert(0, vec![0, 0, 0, 0]);
    cells.insert(1, vec![1, 1, 1, 1]);
    cells.insert(2, vec![0, 1, 0, 1]);
    cells.insert(3, vec![1, 0, 1, 0]);
    cells.insert(4, vec![0, 0, 1, 1]);
    cells.insert(5, vec![1, 0, 0, 1]);
    cells.insert(6, vec![0, 1, 1, 0]);
    cells.insert(7, vec![1, 1, 0, 0]);
    //
    cells.insert(8, vec![0, 1, 1, 1]);
    cells.insert(9, vec![0, 1, 1, 1]);
    cells.insert(10, vec![1, 1, 0, 1]);
    cells.insert(11, vec![1, 1, 0, 1]);

    // create options
    let tile_options = vec![
        0,  //empty
        1,  //all
        2,  //vertical
        3,  //horizontal
        4,  //ld
        5,  //lu
        6,  //rd
        7,  //ru
        8,  //lrd1
        9,  //lrd2
        10, //lru1
        11, //lru2
    ];

    // create grid
    let mut grid = vec![Tile::Options(tile_options.clone()); GRID_SIZE];

    let mut choosen_cell = rng.gen_range(COLUMN..=GRID_SIZE - COLUMN);
    let mut choosen_cell_tile = *tile_options.choose(&mut rng).unwrap() as usize;

    grid[choosen_cell] = Tile::Collapsed(Cell {
        tile: choosen_cell_tile as i32,
        edges: cells[&choosen_cell_tile].clone(),
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

        // ! MARK: Enterance
        if is_key_pressed(KeyCode::A) {
            // reset grid
            grid = vec![Tile::Options(tile_options.clone()); GRID_SIZE];

            choosen_cell = rng.gen_range(COLUMN..=GRID_SIZE - COLUMN);
            choosen_cell_tile = *tile_options.choose(&mut rng).unwrap() as usize;

            grid[choosen_cell] = Tile::Collapsed(Cell {
                tile: choosen_cell_tile as i32,
                edges: cells[&choosen_cell_tile].clone(),
            });
        }

        // ! MARK: WFC Part 1
        wave_funtion(&mut grid, &cells);

        // ! MARK: Check for least option one
        let mut least_one = 0;
        let mut least_num = 100;

        for (num, tile) in grid.iter().enumerate() {
            match tile {
                Tile::Options(options) => {
                    if options.len() < least_num {
                        least_num = options.len();
                        least_one = num;
                    }
                }
                Tile::Collapsed(_) => continue,
            }
        }

        // ! MARK: WFC Part 2
        if let Tile::Options(options) = &grid[least_one] {
            if let Some(damn) = options.choose(&mut rng) {
                let choosen = *damn as usize;

                grid[least_one] = Tile::Collapsed(Cell {
                    tile: choosen as i32,
                    edges: cells[&choosen].clone(),
                });
            } else {
                grid[least_one] = Tile::Options(tile_options.clone())
            }
        }

        // ! MARK: Draw world
        for (index, cell) in grid.iter().enumerate() {
            let x = (index % COLUMN) as f32 * TEXTURE_SIZE;
            let y = (index / COLUMN) as f32 * TEXTURE_SIZE;

            match cell {
                Tile::Options(_) => draw_rectangle(
                    x,
                    y,
                    TEXTURE_SIZE,
                    TEXTURE_SIZE,
                    Color {
                        r: 0.33,
                        g: 0.32,
                        b: 0.49,
                        a: 1.,
                    },
                ),
                Tile::Collapsed(cell) => match cell.tile {
                    0 => draw_rectangle(x, y, TEXTURE_SIZE, TEXTURE_SIZE, BLACK),
                    1 => draw_texture_ex(&rail_all_texture, x, y, WHITE, TEXTURE_PARAM),
                    2 => draw_texture_ex(&rail_h_texture, x, y, WHITE, TEXTURE_PARAM),
                    3 => draw_texture_ex(&rail_v_texture, x, y, WHITE, TEXTURE_PARAM),
                    4 => draw_texture_ex(&rail_ld_texture, x, y, WHITE, TEXTURE_PARAM),
                    5 => draw_texture_ex(&rail_lu_texture, x, y, WHITE, TEXTURE_PARAM),
                    6 => draw_texture_ex(&rail_rd_texture, x, y, WHITE, TEXTURE_PARAM),
                    7 => draw_texture_ex(&rail_ru_texture, x, y, WHITE, TEXTURE_PARAM),
                    //
                    8 => draw_texture_ex(&rail_lrd1_texture, x, y, WHITE, TEXTURE_PARAM),
                    9 => draw_texture_ex(&rail_lrd2_texture, x, y, WHITE, TEXTURE_PARAM),
                    10 => draw_texture_ex(&rail_lru1_texture, x, y, WHITE, TEXTURE_PARAM),
                    11 => draw_texture_ex(&rail_lru2_texture, x, y, WHITE, TEXTURE_PARAM),
                    _ => draw_rectangle(x, y, TEXTURE_SIZE, TEXTURE_SIZE, MAGENTA),
                },
            }
        }
        next_frame().await;
    }
}
