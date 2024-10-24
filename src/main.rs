use ::rand::{seq::SliceRandom, thread_rng, Rng};
use macroquad::prelude::*;

const ROW: f32 = 9.;
const COLUMN: f32 = 16.;
const TEXTURE_SIZE: f32 = 64.;

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
    tile: i16,
    tile_options: Vec<u8>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "WFC-Rails".into(),
        icon: None,
        window_width: (COLUMN * TEXTURE_SIZE) as i32,
        window_height: (ROW * TEXTURE_SIZE) as i32,
        window_resizable: false,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    set_default_filter_mode(FilterMode::Nearest);
    let mut rng = thread_rng();

    //load rails
    let rail_h = load_texture("rail_h.png").await.unwrap();
    let rail_v = load_texture("rail_v.png").await.unwrap();
    let rail_ld = load_texture("rail_ld.png").await.unwrap();
    let rail_lu = load_texture("rail_lu.png").await.unwrap();
    let rail_rd = load_texture("rail_rd.png").await.unwrap();
    let rail_ru = load_texture("rail_ru.png").await.unwrap();

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

    let mut grid = vec![
        Cell {
            is_collapsed: false,
            tile: -1,
            tile_options: vec![0, 1, 2, 3, 4, 5, 6]
        };
        (ROW * COLUMN) as usize
    ];

    //rand::gen_range(low, high);
    let mut choosen_cell =
        rng.gen_range(COLUMN as usize..=(ROW * COLUMN) as usize - COLUMN as usize);
    grid[choosen_cell].is_collapsed = true;
    grid[choosen_cell].tile = rng.gen_range(1..=6);
    grid[choosen_cell].tile_options = vec![grid[choosen_cell].tile as u8];

    loop {
        //update world
        if is_key_pressed(KeyCode::A) {
            grid = vec![
                Cell {
                    is_collapsed: false,
                    tile: -1,
                    tile_options: vec![0, 1, 2, 3, 4, 5, 6]
                };
                (ROW * COLUMN) as usize
            ];

            choosen_cell = rand::gen_range(0, (ROW * COLUMN) as usize);
            info!("choosen cell = {}", choosen_cell);

            grid[choosen_cell].is_collapsed = true;
            grid[choosen_cell].tile = rng.gen_range(1..=6);
            grid[choosen_cell].tile_options = vec![grid[choosen_cell].tile as u8];
        }

        //grid.iter().any(|el| !el.is_collapsed)
        // ! MARK: WFC
        for _ in 0..100 {
            let mut shortest_option = 7;
            let mut index = 50;

            for (grid_index, grid_tile) in grid.iter().enumerate() {
                if grid_tile.tile_options.len() < shortest_option {
                    shortest_option = grid_tile.tile_options.len();
                    index = grid_index;
                }
            }
            info!("shortest index {}", index);

            let north = index - COLUMN as usize;
            let south = index + COLUMN as usize;
            let east = index + 1_usize;
            let west = index - 1_usize;

            if grid[north].tile == 2
                || grid[north].tile == 3
                || grid[north].tile == 5
                || grid[south].tile == 2
                || grid[south].tile == 3
                || grid[south].tile == 5
                || grid[west].tile == 2
                || grid[west].tile == 3
                || grid[west].tile == 5
                || grid[east].tile == 2
                || grid[east].tile == 3
                || grid[east].tile == 5
            {
                grid[index].tile_options.retain(|&a| a != 0);
                grid[index].tile_options.retain(|&a| a != 4);
                grid[index].tile_options.retain(|&a| a != 1);
                grid[index].tile_options.retain(|&a| a != 6);
            } else if grid[north].tile == 1
                || grid[north].tile == 4
                || grid[north].tile == 6
                || grid[south].tile == 1
                || grid[south].tile == 4
                || grid[south].tile == 6
                || grid[west].tile == 1
                || grid[west].tile == 4
                || grid[west].tile == 6
                || grid[east].tile == 1
                || grid[east].tile == 4
                || grid[east].tile == 6
            {
                grid[index].tile_options.retain(|&a| a != 0);
                grid[index].tile_options.retain(|&a| a != 2);
                grid[index].tile_options.retain(|&a| a != 3);
                grid[index].tile_options.retain(|&a| a != 5);
            }

            //info!("index vec = {:?}", grid[index].tile_options);
            grid[index].tile_options.choose(&mut rng);
            grid[index].is_collapsed = true;
            grid[index].tile = grid[index].tile_options[0] as i16;
        }

        // ! MARK: draw world
        for (index, cell) in grid.iter().enumerate() {
            let x = (index as f32 % COLUMN) * TEXTURE_SIZE;
            let y = (index / COLUMN as usize) as f32 * TEXTURE_SIZE;

            match cell.tile {
                -1 => draw_rectangle(x, y, TEXTURE_SIZE, TEXTURE_SIZE, MAGENTA),
                0 => draw_rectangle(
                    x,
                    y,
                    TEXTURE_SIZE,
                    TEXTURE_SIZE,
                    Color::from_rgba(81, 162, 0, 255),
                ),
                1 => draw_texture_ex(&rail_h, x, y, WHITE, texture_param.clone()),
                2 => draw_texture_ex(&rail_v, x, y, WHITE, texture_param.clone()),
                3 => draw_texture_ex(&rail_ld, x, y, WHITE, texture_param.clone()),
                4 => draw_texture_ex(&rail_lu, x, y, WHITE, texture_param.clone()),
                5 => draw_texture_ex(&rail_rd, x, y, WHITE, texture_param.clone()),
                6 => draw_texture_ex(&rail_ru, x, y, WHITE, texture_param.clone()),
                _ => (),
            }
        }
        next_frame().await;
    }
}
