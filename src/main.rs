#![windows_subsystem = "windows"]

use ::rand::{random_range, rng, seq::IndexedRandom};
use egui_macroquad::egui;
use macroquad::prelude::*;
use std::collections::HashMap;

mod app_settings;
mod resources;
mod wfc_functions;

use app_settings::*;
use resources::*;
use wfc_functions::*;

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;
//
const EDGE_COUNT: i32 = 2;
const TEXTURE_SIZE: f32 = 64.0;

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
    let mut rng = rng();
    let mut camera = Camera2D {
        zoom: vec2(2. / screen_width(), 2. / screen_height()),
        ..Default::default()
    };
    let mut zoomer = ZOOM_DEFAULT;

    let mut next_row: usize = 10;
    let mut next_column: usize = 10;
    //
    let mut row: usize = 10;
    let mut column: usize = 10;
    //
    let mut grid_size: usize = row * column;

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
    let mut grid = vec![Cell::Options(tile_options.clone()); grid_size];

    // choose random tile for start
    let mut choosen_cell = random_range(0..grid_size);
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

        // ! MARK: UI
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings")
                .resizable(false)
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(2)
                        .spacing([10.0, 5.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Row: ");
                            ui.add(
                                egui::Slider::new(&mut next_row, 2..=30)
                                    .trailing_fill(true)
                                    .step_by(0.1),
                            );
                            ui.end_row();

                            ui.label("Column: ");
                            ui.add(
                                egui::Slider::new(&mut next_column, 2..=30)
                                    .trailing_fill(true)
                                    .step_by(0.1),
                            );
                            ui.end_row();

                            if ui.button("Generate").clicked() {
                                row = next_row;
                                column = next_column;
                                grid_size = row * column;
                                grid = vec![Cell::Options(tile_options.clone()); grid_size];

                                choosen_cell = random_range(0..grid_size);
                                choosen_cell_tile = tile_options.choose(&mut rng).unwrap();

                                grid[choosen_cell] = Cell::Collapsed(TileProp {
                                    tile: choosen_cell_tile.clone(),
                                    edges: cells[choosen_cell_tile].clone(),
                                });
                            }
                        })
                });
        });

        // ! MARK: WFC Part 1: Wave
        wave_funtion(&mut grid, &cells, column, row);

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

        let pos_x = (column as f32 * TEXTURE_SIZE) / 2.;
        let pos_y = (row as f32 * TEXTURE_SIZE) / 2.;

        let texture_param = DrawTextureParams {
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

        for (index, cell) in grid.iter().enumerate() {
            let x = (index % column) as f32 * TEXTURE_SIZE - pos_x;
            let y = (index / column) as f32 * TEXTURE_SIZE - pos_y;

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

        egui_macroquad::draw();
        next_frame().await;
    }
}
