use macroquad::prelude::*;
use std::collections::HashMap;

use crate::{Cell, Tile, BOTTOM, EDGE_COUNT, LEFT, RIGHT, TOP};

pub fn wave_funtion(grid: &mut [Cell], cells: &HashMap<Tile, Vec<i32>>, column: usize, row: usize) {
    for y in 0..row {
        'row: for x in 0..column {
            let current_tile = (y * column) + x;

            if let Cell::Collapsed(cell) = grid[current_tile].clone() {
                // ! MARK: check right
                if x != column - 1 {
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
                    if let Cell::Options(options) = grid[current_tile - column].clone() {
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

                                grid[current_tile - column] = Cell::Options(matching);
                            }
                        }
                    }
                }

                // ! MARK: check bottom
                if y != row - 1 {
                    if let Cell::Options(options) = grid[current_tile + column].clone() {
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

                                grid[current_tile + column] = Cell::Options(matching);
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
