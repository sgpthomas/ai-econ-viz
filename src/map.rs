use log::info;
use std::fs;

use crate::tiles::Tile;

pub struct Map {
    pub tiles: Vec<Vec<Option<Tile>>>,
}

impl Map {
    pub fn read_from_file(path: &str) -> Self {
        let raw_map = fs::read_to_string(path).expect("Couldn't read file");
        Self::read_from_str(&raw_map)
    }

    pub fn read_from_str(raw: &str) -> Self {
        let rows: Vec<&str> = raw.split(";").collect();
        let width = rows[0].len();
        let height = rows.len();

        let mut tiles: Vec<Vec<Option<Tile>>> = vec![vec![None; width]; height];

        for x in 0..width {
            for y in 0..height {
                if let Some(tile) = rows[y].chars().nth(x) {
                    tiles[x][y] = match tile {
                        'W' => Some(Tile::Tree),
                        'S' => Some(Tile::Stone),
                        '@' => Some(Tile::Wall),
                        ' ' => None,
                        x => panic!("Unexpected token {}", x),
                    }
                }
            }
        }
        info!("{:#?}", tiles);

        Map { tiles }
    }
}
