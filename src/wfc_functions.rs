use std::collections::HashMap;

use crate::{Tile, TileOptions, BOTTOM, COLUMN, EDGE_COUNT, LEFT, RIGHT, ROW, TOP};

pub fn wave_funtion(grid: &mut [Tile], cells: &HashMap<TileOptions, Vec<i32>>) {
    for y in 0..ROW {
        'row: for x in 0..COLUMN {
            let current_tile = (y * COLUMN) + x;

            if let Tile::Collapsed(cell) = grid[current_tile].clone() {
                // ! MARK: check right
                if x != COLUMN - 1 {
                    if let Tile::Options(options) = grid[current_tile + 1].clone() {
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

                                grid[current_tile + 1] = Tile::Options(matching);
                            }
                        }
                    }
                }

                // ! MARK: check left
                if x != 0 {
                    if let Tile::Options(options) = grid[current_tile - 1].clone() {
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

                                grid[current_tile - 1] = Tile::Options(matching);
                            }
                        }
                    }
                }

                // ! MARK: check top
                if y != 0 {
                    if let Tile::Options(options) = grid[current_tile - COLUMN].clone() {
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

                                grid[current_tile - COLUMN] = Tile::Options(matching);
                            }
                        }
                    }
                }

                // ! MARK: check bottom
                if y != ROW - 1 {
                    if let Tile::Options(options) = grid[current_tile + COLUMN].clone() {
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

                                grid[current_tile + COLUMN] = Tile::Options(matching);
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
