use std::vec;

use crate::get_lines;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum D22Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid line: {0}")]
    InvalidLine(String),
    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

pub fn parse_coords_str(coords: &str, line: String) -> Result<[u32; 3], D22Error> {
    coords
        .split(',')
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?
        .try_into()
        .or(Err(D22Error::InvalidLine(line.to_string())))
}

pub fn set_block_fall(b: &([u32; 3], [u32; 3]), fall: &mut Vec<Vec<Vec<u32>>>, fill_value: u32) {
    for x in b.0[0].min(b.1[0])..=b.0[0].max(b.1[0]) {
        for y in b.0[1].min(b.1[1])..=b.1[1].max(b.1[1]) {
            for z in b.0[2].min(b.1[2])..=b.1[2].max(b.1[2]) {
                fall[x as usize][y as usize][z as usize] = fill_value;
            }
        }
    }
}

pub fn get_block_supports(fp: &str) -> Result<Vec<Vec<usize>>, D22Error> {
    let mut blocks: Vec<([u32; 3], [u32; 3])> = vec![];
    let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);

    for line in get_lines(fp)? {
        let line = line?;
        let Some((coords1, coords2)) = line.split_once('~') else {
            return Err(D22Error::InvalidLine(line.to_string()));
        };
        let block = (
            parse_coords_str(coords1, line.to_string())?,
            parse_coords_str(coords2, line.to_string())?,
        );
        blocks.push(block);

        max_x = max_x.max(block.0[0]).max(block.1[0]);
        max_y = max_y.max(block.0[1]).max(block.1[1]);
        max_z = max_z.max(block.0[2]).max(block.1[2]);
    }

    let (max_x, max_y, max_z) = (max_x as usize + 1, max_y as usize + 1, max_z as usize + 1);
    let mut full = vec![vec![vec![false; max_z]; max_y]; max_x];
    let mut fall = vec![vec![vec![u32::MAX; max_z]; max_y]; max_x];
    let mut support = vec![vec![vec![0; max_z]; max_y]; max_x];
    for x in 0..max_x {
        for y in 0..max_y {
            full[x][y][0] = true;
            fall[x][y][0] = 0;
        }
    }

    for (i, b) in blocks.iter().enumerate() {
        for x in b.0[0].min(b.1[0])..=b.0[0].max(b.1[0]) {
            for y in b.0[1].min(b.1[1])..=b.1[1].max(b.1[1]) {
                for z in b.0[2].min(b.1[2])..=b.1[2].max(b.1[2]) {
                    full[x as usize][y as usize][z as usize] = true;
                    support[x as usize][y as usize][z as usize] = i + 1;
                }
            }
        }
    }

    let mut block_supports = vec![vec![]; blocks.len()];
    for z in 1..max_z {
        for x in 0..max_x {
            for y in 0..max_y {
                let fall_dist = fall[x][y][z - 1] + !full[x][y][z] as u32;
                let supported_by = support[x][y][z - 1];

                if !full[x][y][z] {
                    fall[x][y][z] = fall_dist;
                    support[x][y][z] = supported_by;
                } else if supported_by != support[x][y][z] {
                    let block_ind = support[x][y][z] - 1;
                    if fall_dist < fall[x][y][z] {
                        block_supports[block_ind] = vec![supported_by];
                        set_block_fall(&blocks[block_ind], &mut fall, fall_dist);
                    } else if fall_dist == fall[x][y][z]
                        && !block_supports[block_ind].contains(&supported_by)
                    {
                        block_supports[block_ind].push(supported_by);
                    }
                }
            }
        }
    }

    Ok(block_supports)
}
