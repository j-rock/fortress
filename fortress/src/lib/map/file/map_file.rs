use app::StatusOr;
use file::{
    Config,
    self,
};
use liquidfun;
use map::file::MapFileCharacter;
use std::{
    collections::HashSet,
    path::PathBuf
};

pub struct GridLocation(usize, usize);

impl GridLocation {
    pub fn to_2d(&self, grid_cell_len: f32) -> liquidfun::box2d::common::math::Vec2 {
        liquidfun::box2d::common::math::Vec2 {
            x: grid_cell_len * (self.1 as f32),
            y: -grid_cell_len * (self.0 as f32),
        }
    }
}

pub struct GridWall {
    pub top_left_location: GridLocation,
    pub size: (usize, usize)
}

pub struct MapFile {
    pub buff_boxes: Vec<GridLocation>,
    pub spawns: Vec<GridLocation>,
    pub walls: Vec<GridWall>,
}

impl Config for MapFile {
    fn from_path(path: &PathBuf) -> StatusOr<MapFile> {
        let raw_map_data = Self::raw_data(path)?;
        let mut wall_claimed: HashSet<(usize, usize)> = HashSet::with_capacity(raw_map_data.len() * raw_map_data[0].len());

        let mut buff_boxes = Vec::new();
        let mut spawns = Vec::new();
        let mut walls = Vec::new();

        for (row_idx, row) in raw_map_data.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if let Some(cell) = cell {
                    match cell {
                        MapFileCharacter::Wall => {
                            if !wall_claimed.contains(&(row_idx, col_idx)) {
                                let wall_right_edge = Self::compute_wall_end(&wall_claimed, row, row_idx, col_idx);
                                let wall_bottom_edge = Self::compute_wall_bottom(&raw_map_data, row_idx, col_idx, wall_right_edge);

                                for row_i in row_idx..(wall_bottom_edge + 1) {
                                    for col_i in col_idx..(wall_right_edge + 1) {
                                        wall_claimed.insert((row_i, col_i));
                                    }
                                }

                                walls.push(Self::compute_wall((row_idx, wall_bottom_edge), (col_idx, wall_right_edge)));
                            }
                        },
                        MapFileCharacter::Buff => {
                            buff_boxes.push(GridLocation(row_idx, col_idx));
                        }
                        MapFileCharacter::Spawn => {
                            spawns.push(GridLocation(row_idx, col_idx));
                        },
                    }
                }
            }
        }

        if spawns.len() < 4 {
            return Err(format!("Mapfile must contain at least 4 spawns: {:?}", path));
        }

        Ok(MapFile {
            buff_boxes,
            spawns,
            walls,
        })
    }
}

impl MapFile {
    fn raw_data(path: &PathBuf) -> StatusOr<Vec<Vec<Option<MapFileCharacter>>>> {
        let mut data = vec!();

        let mut max_line_length = None;
        for (line_idx, line) in file::util::slurp_file(path)?.lines().enumerate() {
            let line_length = line.len();
            max_line_length = match max_line_length {
                Some(len) => {
                    if line_length != len {
                        return Err(format!("Uneven map data for {:?}, line {}", path, line_idx));
                    }
                    Some(len)
                },
                None => Some(line_length)
            };
            let mut row_data = Vec::with_capacity(line.len());
            for (column_idx, byte) in line.as_bytes().iter().enumerate() {
                row_data.push(MapFileCharacter::parse_byte(path, line_idx, column_idx, *byte)?);
            }

            data.push(row_data);
        }

        if data.len() < 1 {
            return Err(format!("MapFile must contain at least 1 row: {:?}", path));
        }

        Ok(data)
    }

    fn compute_wall_end(wall_claimed: &HashSet<(usize, usize)>, row: &Vec<Option<MapFileCharacter>>, row_idx: usize, start_col_idx: usize) -> usize {
        let mut out_idx = start_col_idx;
        for col in (start_col_idx + 1)..row.len() {
            if let Some(MapFileCharacter::Wall) = row[col] {
                if !wall_claimed.contains(&(row_idx, col)) {
                    out_idx = col;
                } else {
                    return out_idx;
                }
            } else {
                return out_idx;
            }
        }
        return out_idx;
    }

    fn compute_wall_bottom(raw_map_data: &Vec<Vec<Option<MapFileCharacter>>>, row_idx: usize, col_idx: usize, wall_right_edge: usize) -> usize {
        let mut out_row = row_idx;
        for row_i in (row_idx + 1)..raw_map_data.len() {
            if col_idx == wall_right_edge {
                if let Some(MapFileCharacter::Wall) = raw_map_data[row_i][col_idx] {
                    out_row = row_i;
                } else {
                    return out_row;
                }
            }
            if raw_map_data[row_i][col_idx .. wall_right_edge + 1]
                .iter()
                .all(|cell| {
                    if let Some(MapFileCharacter::Wall) = *cell {
                        true
                    } else {
                        false
                    }
                }) {
                out_row = row_i;
            } else {
                return out_row;
            }
        }
        return out_row;
    }

    fn compute_wall(rows: (usize, usize), cols: (usize, usize)) -> GridWall {
        let width = cols.1 - cols.0 + 1;
        let height = rows.1 - rows.0 + 1;

        GridWall {
            top_left_location: GridLocation(rows.0, cols.0),
            size: (width, height),
        }
    }
}

