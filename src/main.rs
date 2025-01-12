#![windows_subsystem = "windows"]

use ::rand::{seq::SliceRandom, thread_rng, Rng};
use macroquad::prelude::*;
use std::collections::HashMap;

mod app_settings;
mod resources;
mod wfc_functions;

use app_settings::*;
use resources::*;
use wfc_functions::*;

const ROW: usize = 10;
const COLUMN: usize = 15;
const GRID_SIZE: usize = ROW * COLUMN;
const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;
const EDGE_COUNT: i32 = 2;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
enum Tile {
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
    let mut texture_size: f32;
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = Vec2::ZERO;

    let textures = Resources::load_textures();

    // create tiles and edges
    let cells = HashMap::from([
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
        let minimum_frame_time = 1. / 120.;
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

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
            }
        }

        camera_controller(&mut camera, &mut zoomer);

        // ! MARK: Draw world
        set_camera(&camera);

        texture_size = screen_height() / ROW as f32;

        let pos_x = (COLUMN as f32 * texture_size) / 2.;
        let pos_y = (ROW as f32 * texture_size) / 2.;

        let texture_param = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: texture_size,
                y: texture_size,
            }),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        for (index, cell) in grid.iter().enumerate() {
            let x = (index % COLUMN) as f32 * texture_size - pos_x;
            let y = (index / COLUMN) as f32 * texture_size - pos_y;

            match cell {
                Cell::Options(_) => {
                    draw_texture_ex(&textures.uc_sign, x, y, WHITE, texture_param.clone())
                }
                Cell::Collapsed(cell) => match cell.tile {
                    Tile::Empty => {
                        draw_texture_ex(&textures.rail_empty, x, y, WHITE, texture_param.clone())
                    }
                    Tile::All => {
                        draw_texture_ex(&textures.rail_all, x, y, WHITE, texture_param.clone())
                    }
                    Tile::Horizontal => {
                        draw_texture_ex(&textures.rail_h, x, y, WHITE, texture_param.clone())
                    }
                    Tile::Vertical => {
                        draw_texture_ex(&textures.rail_v, x, y, WHITE, texture_param.clone())
                    }
                    Tile::LeftDown => {
                        draw_texture_ex(&textures.rail_ld, x, y, WHITE, texture_param.clone())
                    }
                    Tile::LeftUp => {
                        draw_texture_ex(&textures.rail_lu, x, y, WHITE, texture_param.clone())
                    }
                    Tile::RightDown => {
                        draw_texture_ex(&textures.rail_rd, x, y, WHITE, texture_param.clone())
                    }
                    Tile::RightUp => {
                        draw_texture_ex(&textures.rail_ru, x, y, WHITE, texture_param.clone())
                    }
                    Tile::LeftRightDown1 => {
                        draw_texture_ex(&textures.rail_lrd1, x, y, WHITE, texture_param.clone())
                    }
                    Tile::LeftRightDown2 => {
                        draw_texture_ex(&textures.rail_lrd2, x, y, WHITE, texture_param.clone())
                    }
                    Tile::LeftRightUp1 => {
                        draw_texture_ex(&textures.rail_lru1, x, y, WHITE, texture_param.clone())
                    }
                    Tile::LeftRightUp2 => {
                        draw_texture_ex(&textures.rail_lru2, x, y, WHITE, texture_param.clone())
                    }
                },
            }
        }

        next_frame().await;
    }
}

pub fn camera_controller(camera: &mut Camera2D, zoomer: &mut Vec2) {
    // ! window res
    camera.zoom = vec2(
        2. / screen_width() + zoomer.x / screen_width(),
        2. / screen_height() + zoomer.y / screen_height(),
    );
    camera.target = Vec2::ZERO;

    if screen_width() < 320. {
        request_new_screen_size(320., screen_height());
    }

    if screen_height() < 240. {
        request_new_screen_size(screen_width(), 240.);
    }

    // ! controller
    if mouse_wheel().1 > 0. {
        *zoomer += 0.2
    } else if mouse_wheel().1 < 0. && zoomer.x > -1. {
        *zoomer -= 0.2;
    }

    if camera.zoom.x < 0. {
        camera.zoom += Vec2::new(0.1 / screen_width(), 0.1 / screen_height())
    }

    if is_mouse_button_down(MouseButton::Left) {
        let mouse_pos = mouse_delta_position();

        camera.offset.x -= mouse_pos.x;
        camera.offset.y += mouse_pos.y;
    }

    if is_key_pressed(KeyCode::Space) {
        camera.offset = Vec2::ZERO;
        *zoomer = Vec2::ZERO;
    }
}
