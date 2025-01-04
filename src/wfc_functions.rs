use std::collections::HashMap;

use crate::{Tile, BOTTOM, COLUMN, LEFT, RIGHT, ROW, TOP};

pub fn wave_funtion(grid: &mut [Tile], cells: &HashMap<usize, Vec<i32>>) {
    for y in 0..ROW {
        'row: for x in 0..COLUMN {
            let current_tile = (y * COLUMN) + x;

            if let Tile::Collapsed(cell) = grid[current_tile].clone() {
                // ! MARK: check right
                if x != COLUMN - 1 {
                    if let Tile::Options(options) = &grid[current_tile + 1] {
                        if cell.edges[RIGHT] == 1 {
                            let collection: Vec<_> = cells
                                .clone()
                                .into_iter()
                                .filter(|(_, value)| value[LEFT] == 1)
                                .map(|(key, _)| key)
                                .collect();

                            let matching: Vec<_> = collection
                                .into_iter()
                                .filter(|item| options.contains(&(*item as i32)))
                                .map(|key| key as i32)
                                .collect();

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

                // ! MARK: check left
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

                // ! MARK: check top
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

                // ! MARK: check bottom
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
}
