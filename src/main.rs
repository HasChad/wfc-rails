use ::rand::{seq::SliceRandom, thread_rng};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::iter::Iterator;

const ROW: usize = 5;
const COLUMN: usize = 5;
const TEXTURE_SIZE: f32 = 64.;
const GRID_SIZE: usize = ROW * COLUMN;
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
const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

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
    let rail_h_texture = load_texture("rail_h.png").await.unwrap();
    let rail_v_texture = load_texture("rail_v.png").await.unwrap();
    let rail_ld_texture = load_texture("rail_ld.png").await.unwrap();
    let rail_lu_texture = load_texture("rail_lu.png").await.unwrap();
    let rail_rd_texture = load_texture("rail_rd.png").await.unwrap();
    let rail_ru_texture = load_texture("rail_ru.png").await.unwrap();

    // create tiles and edges
    let mut cells: HashMap<usize, Vec<i32>> = HashMap::new();
    cells.insert(0, vec![0, 0, 0, 0]);
    cells.insert(1, vec![0, 1, 0, 1]);
    cells.insert(2, vec![1, 0, 1, 0]);
    cells.insert(3, vec![0, 0, 1, 1]);
    cells.insert(4, vec![1, 0, 0, 1]);
    cells.insert(5, vec![0, 1, 1, 0]);
    cells.insert(6, vec![1, 1, 0, 0]);

    // create grid
    let mut grid = vec![Tile::Options(vec![0, 1, 2, 3, 4, 5, 6]); GRID_SIZE];

    // ! prototype
    grid[0] = Tile::Collapsed(Cell {
        tile: 1,
        edges: vec![0, 1, 0, 1],
    });

    /*
    match &grid[0] {
        Tile::Options(options) => {
            let choosen: usize = *options.choose(&mut rng).unwrap() as usize;

            grid[0] = Tile::Collapsed(Cell {
                tile: choosen as i32,
                edges: cells[&choosen].clone(),
            });
        }
        Tile::Collapsed(cell) => {
            if cell.edges[RIGHT] == 1 {
                let collection: Vec<_> = cells
                    .iter()
                    .filter(|(_, value)| value[LEFT] == 1)
                    .map(|(key, _)| key)
                    .collect();

                let choosen = **collection.choose(&mut rng).unwrap() as usize;

                grid[1] = Tile::Collapsed(Cell {
                    tile: choosen as i32,
                    edges: cells[&choosen].clone(),
                });
            }
        }
    }
    */

    /*
        let mut choosen_cell = rng.gen_range(COLUMN..=GRID_SIZE - COLUMN);

        grid[choosen_cell].is_collapsed = true;
        grid[choosen_cell].tile = rng.gen_range(1..=6);
        grid[choosen_cell].tile_options = vec![grid[choosen_cell].tile as u8];
    */

    loop {
        /*
        let minimum_frame_time = 1. / 1.; // 60 FPS
        let frame_time = get_frame_time();
        println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));

        }
        */
        /*
        if is_key_pressed(KeyCode::A) {
            grid = vec![
                Cell {
                    is_collapsed: false,
                    tile: -1,
                    tile_options: vec![0, 1, 2, 3, 4, 5, 6]
                };
                GRID_SIZE
            ];

            choosen_cell = rand::gen_range(0, GRID_SIZE);
            info!("choosen cell = {}", choosen_cell);

            grid[choosen_cell].is_collapsed = true;
            grid[choosen_cell].tile = rng.gen_range(1..=6);
            grid[choosen_cell].tile_options = vec![grid[choosen_cell].tile as u8];
        }
        */
        // ! MARK: First WFC
        for y in 0..ROW {
            'row: for x in 0..COLUMN {
                let current_tile = (y * COLUMN) + x;

                if let Tile::Collapsed(cell) = grid[current_tile].clone() {
                    // check right
                    if x != COLUMN - 1 {
                        if let Tile::Options(options) = &grid[current_tile + 1] {
                            if cell.edges[RIGHT] == 1 {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[LEFT] == 1)
                                    .map(|(key, _)| key)
                                    .collect();

                                //info!("{:?}", options);
                                // info!("{:?}", collection);

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(&(*item as i32)))
                                    .map(|key| key as i32)
                                    .collect();

                                // info!("{:?}", matching);

                                grid[current_tile + 1] = Tile::Options(matching);
                            } else {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[LEFT] == 0)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(&(*item as i32)))
                                    .map(|key| key as i32)
                                    .collect();

                                grid[current_tile + 1] = Tile::Options(matching);
                            }
                        }
                    }

                    //check left
                    if x != 0 {
                        if let Tile::Options(options) = &grid[current_tile - 1] {
                            if cell.edges[LEFT] == 1 {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[RIGHT] == 1)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(&(*item as i32)))
                                    .map(|key| key as i32)
                                    .collect();

                                grid[current_tile - 1] = Tile::Options(matching);
                            } else {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[RIGHT] == 0)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(&(*item as i32)))
                                    .map(|key| key as i32)
                                    .collect();

                                grid[current_tile - 1] = Tile::Options(matching);
                            }
                        }
                    }

                    //check top
                    if y != 0 {
                        if let Tile::Options(options) = &grid[current_tile - COLUMN] {
                            if cell.edges[TOP] == 1 {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[BOTTOM] == 1)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(&(*item as i32)))
                                    .map(|key| key as i32)
                                    .collect();

                                grid[current_tile - COLUMN] = Tile::Options(matching);
                            } else {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[BOTTOM] == 0)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(&(*item as i32)))
                                    .map(|key| key as i32)
                                    .collect();

                                grid[current_tile - COLUMN] = Tile::Options(matching);
                            }
                        }
                    }

                    //check bottom
                    if y != ROW - 1 {
                        if let Tile::Options(options) = &grid[current_tile + COLUMN] {
                            if cell.edges[BOTTOM] == 1 {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[TOP] == 1)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(&(*item as i32)))
                                    .map(|key| key as i32)
                                    .collect();

                                grid[current_tile + COLUMN] = Tile::Options(matching);
                            } else {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[TOP] == 0)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(&(*item as i32)))
                                    .map(|key| key as i32)
                                    .collect();

                                grid[current_tile + COLUMN] = Tile::Options(matching);
                            }
                        }
                    }
                } else {
                    continue 'row;
                }
            }
        }

        // ! MARK: Check for least option ones
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

        // ! MARK: Second WFC
        if let Tile::Options(options) = &grid[least_one] {
            let choosen = *options.choose(&mut rng).unwrap() as usize;

            grid[least_one] = Tile::Collapsed(Cell {
                tile: choosen as i32,
                edges: cells[&choosen].clone(),
            });
        }

        /*
        for y in 0..ROW {
            'row: for x in 0..COLUMN {
                let current_tile = (y * COLUMN) + x;

                if let Tile::Collapsed(_) = &grid[current_tile] {
                    continue 'row;
                }

                if x != 0 {
                    if let Tile::Collapsed(cell) = &grid[current_tile - 1] {
                        if cell.edges[RIGHT] == 1 {
                            let collection: Vec<_> = cells
                                .iter()
                                .filter(|(_, value)| value[LEFT] == 1)
                                .map(|(key, _)| key)
                                .collect();

                            let choosen = **collection.choose(&mut rng).unwrap() as usize;

                            grid[current_tile] = Tile::Collapsed(Cell {
                                tile: choosen as i32,
                                edges: cells[&choosen].clone(),
                            });
                        } else {
                            let collection: Vec<_> = cells
                                .iter()
                                .filter(|(_, value)| value[LEFT] == 0)
                                .map(|(key, _)| key)
                                .collect();

                            let choosen = **collection.choose(&mut rng).unwrap() as usize;

                            grid[current_tile] = Tile::Collapsed(Cell {
                                tile: choosen as i32,
                                edges: cells[&choosen].clone(),
                            });
                        }
                    }
                } else if y != 0 {
                    if let Tile::Collapsed(cell) = &grid[current_tile - COLUMN] {
                        if cell.edges[BOTTOM] == 1 {
                            let collection: Vec<_> = cells
                                .iter()
                                .filter(|(_, value)| value[TOP] == 1)
                                .map(|(key, _)| key)
                                .collect();

                            let choosen = **collection.choose(&mut rng).unwrap() as usize;

                            grid[current_tile] = Tile::Collapsed(Cell {
                                tile: choosen as i32,
                                edges: cells[&choosen].clone(),
                            });
                        } else {
                            let collection: Vec<_> = cells
                                .iter()
                                .filter(|(_, value)| value[TOP] == 0)
                                .map(|(key, _)| key)
                                .collect();

                            let choosen = **collection.choose(&mut rng).unwrap() as usize;

                            grid[current_tile] = Tile::Collapsed(Cell {
                                tile: choosen as i32,
                                edges: cells[&choosen].clone(),
                            });
                        }
                    }
                }
            }
        }
        */

        // ! MARK: draw world
        for (index, cell) in grid.iter().enumerate() {
            let x = (index % COLUMN) as f32 * TEXTURE_SIZE;
            let y = (index / COLUMN) as f32 * TEXTURE_SIZE;

            match cell {
                Tile::Options(_) => draw_rectangle(x, y, TEXTURE_SIZE, TEXTURE_SIZE, MAGENTA),
                Tile::Collapsed(cell) => match cell.tile {
                    0 => draw_rectangle(x, y, TEXTURE_SIZE, TEXTURE_SIZE, BLACK),
                    1 => draw_texture_ex(&rail_h_texture, x, y, WHITE, TEXTURE_PARAM),
                    2 => draw_texture_ex(&rail_v_texture, x, y, WHITE, TEXTURE_PARAM),
                    3 => draw_texture_ex(&rail_ld_texture, x, y, WHITE, TEXTURE_PARAM),
                    4 => draw_texture_ex(&rail_lu_texture, x, y, WHITE, TEXTURE_PARAM),
                    5 => draw_texture_ex(&rail_rd_texture, x, y, WHITE, TEXTURE_PARAM),
                    6 => draw_texture_ex(&rail_ru_texture, x, y, WHITE, TEXTURE_PARAM),
                    _ => draw_rectangle(x, y, TEXTURE_SIZE, TEXTURE_SIZE, MAGENTA),
                },
            }
        }
        next_frame().await;
    }
}
