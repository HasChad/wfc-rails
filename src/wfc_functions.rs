use macroquad::prelude::*;
use std::collections::HashMap;

use crate::{Cell, Tile, BOTTOM, EDGE_COUNT, LEFT, RIGHT, TOP};

pub fn wave_funtion(
    grid: &mut [Cell],
    cells: &HashMap<Tile, Vec<i32>>,
    grid_row: &usize,
    grid_column: &usize,
) {
    for y in 0..*grid_row {
        'row: for x in 0..*grid_column {
            let current_tile = (y * *grid_column) + x;

            if let Cell::Collapsed(cell) = grid[current_tile].clone() {
                // ! MARK: check right
                if x != *grid_column - 1 {
                    if let Cell::Options(options) = grid[current_tile + 1].clone() {
                        for edge_count in 0..EDGE_COUNT {
                            if cell.edges[RIGHT] == edge_count {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[LEFT] == edge_count)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(item))
                                    .collect();

                                grid[current_tile + 1] = Cell::Options(matching);
                            }
                        }
                    }
                }

                // ! MARK: check left
                if x != 0 {
                    if let Cell::Options(options) = grid[current_tile - 1].clone() {
                        for edge_count in 0..EDGE_COUNT {
                            if cell.edges[LEFT] == edge_count {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[RIGHT] == edge_count)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(item))
                                    .collect();

                                grid[current_tile - 1] = Cell::Options(matching);
                            }
                        }
                    }
                }

                // ! MARK: check top
                if y != 0 {
                    if let Cell::Options(options) = grid[current_tile - *grid_column].clone() {
                        for edge_count in 0..EDGE_COUNT {
                            if cell.edges[TOP] == edge_count {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[BOTTOM] == edge_count)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(item))
                                    .collect();

                                grid[current_tile - *grid_column] = Cell::Options(matching);
                            }
                        }
                    }
                }

                // ! MARK: check bottom
                if y != *grid_row - 1 {
                    if let Cell::Options(options) = grid[current_tile + *grid_column].clone() {
                        for edge_count in 0..EDGE_COUNT {
                            if cell.edges[BOTTOM] == edge_count {
                                let collection: Vec<_> = cells
                                    .clone()
                                    .into_iter()
                                    .filter(|(_, value)| value[TOP] == edge_count)
                                    .map(|(key, _)| key)
                                    .collect();

                                let matching: Vec<_> = collection
                                    .into_iter()
                                    .filter(|item| options.contains(item))
                                    .collect();

                                grid[current_tile + *grid_column] = Cell::Options(matching);
                            }
                        }
                    }
                }
            } else {
                continue 'row;
            }
        }
    }
}

pub fn camera_fixer(camera: &mut Camera2D, zoomer: &mut Vec2) {
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
    } else if mouse_wheel().1 < 0. && zoomer.x > -2. {
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
