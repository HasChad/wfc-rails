use ::rand::{seq::SliceRandom, thread_rng, Rng};
use macroquad::prelude::*;
use std::collections::HashMap;

const ROW: usize = 2;
const COLUMN: usize = 2;
const TEXTURE_SIZE: f32 = 64.;
const GRID_SIZE: usize = ROW * COLUMN;
const TEXTURE_PARAM: macroquad::texture::DrawTextureParams = DrawTextureParams {
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

/*

- Tüm celleri -1 olan vector yap
`grid = cell {tile, is_collapsed, tile_options}`
- Rastgele choosen_cell ve tile seç
- choosen_cell collapse at ve komşu cell lere operasyon yap
>Loop: if grid contains -1
- grid içinde en az tile_options a sahip olan cell i bul
  - if grid
- cell içinde rastgele tile seç ve collapse at
- Komşu cell lere operasyon yap

*/

#[derive(Clone, PartialEq)]
struct Cell {
    is_collapsed: bool,
    tile: i32,
    edges: Vec<i32>,
    tile_options: Vec<u8>,
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

    //load rail texture
    let rail_h_texture = load_texture("rail_h.png").await.unwrap();
    let rail_v_texture = load_texture("rail_v.png").await.unwrap();
    let rail_ld_texture = load_texture("rail_ld.png").await.unwrap();
    let rail_lu_texture = load_texture("rail_lu.png").await.unwrap();
    let rail_rd_texture = load_texture("rail_rd.png").await.unwrap();
    let rail_ru_texture = load_texture("rail_ru.png").await.unwrap();

    let mut cells: HashMap<usize, Vec<i32>> = HashMap::new();

    cells.insert(1, vec![0, 1, 0, 1]);
    cells.insert(2, vec![1, 0, 1, 0]);
    cells.insert(3, vec![0, 0, 1, 1]);
    cells.insert(4, vec![1, 0, 0, 1]);
    cells.insert(5, vec![0, 1, 1, 0]);
    cells.insert(6, vec![1, 1, 0, 0]);

    let mut grid = vec![
        Cell {
            is_collapsed: false,
            tile: -1,
            edges: vec![0, 0, 0, 0],
            tile_options: vec![0, 1, 2, 3, 4, 5, 6]
        };
        GRID_SIZE
    ];

    grid[0] = Cell {
        is_collapsed: true,
        tile: 1,
        edges: vec![0, 1, 0, 1],
        tile_options: vec![],
    };

    if grid[0].edges[1] == 1 {
        let collection = {
            let mut damn: Vec<&usize> = vec![];
            for (num, item) in cells.iter() {
                if item[3] == 1 {
                    damn.push(num);
                }
            }
            damn
        };

        grid[1].tile = **collection.choose(&mut rng).unwrap() as i32;
    }

    /*
        let mut choosen_cell = rng.gen_range(COLUMN..=GRID_SIZE - COLUMN);

        grid[choosen_cell].is_collapsed = true;
        grid[choosen_cell].tile = rng.gen_range(1..=6);
        grid[choosen_cell].tile_options = vec![grid[choosen_cell].tile as u8];
    */

    loop {
        //update world
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

        // ! MARK: WFC

        // ! MARK: draw world
        for (index, cell) in grid.iter().enumerate() {
            let x = (index % COLUMN) as f32 * TEXTURE_SIZE;
            let y = (index / COLUMN) as f32 * TEXTURE_SIZE;

            match cell.tile {
                0 => draw_rectangle(x, y, TEXTURE_SIZE, TEXTURE_SIZE, BLACK),
                1 => draw_texture_ex(&rail_h_texture, x, y, WHITE, TEXTURE_PARAM),
                2 => draw_texture_ex(&rail_v_texture, x, y, WHITE, TEXTURE_PARAM),
                3 => draw_texture_ex(&rail_ld_texture, x, y, WHITE, TEXTURE_PARAM),
                4 => draw_texture_ex(&rail_lu_texture, x, y, WHITE, TEXTURE_PARAM),
                5 => draw_texture_ex(&rail_rd_texture, x, y, WHITE, TEXTURE_PARAM),
                6 => draw_texture_ex(&rail_ru_texture, x, y, WHITE, TEXTURE_PARAM),
                _ => draw_rectangle(x, y, TEXTURE_SIZE, TEXTURE_SIZE, MAGENTA),
            }
        }
        next_frame().await;
    }
}
