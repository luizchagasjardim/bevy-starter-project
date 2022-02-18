#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Ground,
}

impl Tile {
    pub fn connects_to(self, other: Tile) -> bool {
        match (self, other) {
            (Tile::Ground, Tile::Ground) => true,
            (_, _) => false,
        }
    }
}

type Line = [Tile; Map::HEIGHT];

pub struct Map {
    values: [Line; Map::WIDTH],
}

impl Map {
    pub const WIDTH: usize = 100;
    pub const HEIGHT: usize = 20;
    pub fn left(&self, i: usize, j: usize) -> Tile {
        if i > 0 { self.values[i-1][j] } else { Tile::Empty }
    }
    pub fn right(&self, i: usize, j: usize) -> Tile {
        if i+1 < Self::WIDTH { self.values[i+1][j] } else { Tile::Empty }
    }
    pub fn below(&self, i: usize, j: usize) -> Tile {
        if j > 0 { self.values[i][j-1] } else { Tile::Empty }
    }
    pub fn above(&self, i: usize, j: usize) -> Tile {
        if j+1 < Self::HEIGHT { self.values[i][j+1] } else { Tile::Empty }
    }
    pub fn below_left(&self, i: usize, j: usize) -> Tile {
        if i > 0 && j > 0 { self.values[i-1][j-1] } else { Tile::Empty }
    }
    pub fn below_right(&self, i: usize, j: usize) -> Tile {
        if i+1 < Self::WIDTH && j > 0 { self.values[i+1][j-1] } else { Tile::Empty }
    }
    pub fn above_left(&self, i: usize, j: usize) -> Tile {
        if i > 0 && j+1 < Self::HEIGHT { self.values[i-1][j+1] } else { Tile::Empty }
    }
    pub fn above_right(&self, i: usize, j: usize) -> Tile {
        if i+1 < Self::WIDTH && j+1 < Self::HEIGHT { self.values[i+1][j+1] } else { Tile::Empty }
    }
    pub fn get_image(&self, i: usize, j: usize) -> Option<(&str, bool)> {
        use crate::sprite::SPRITES;

        let ground_type = "ground";
        let tile = self[i][j];
        match tile {
            Tile::Empty => None,
            Tile::Ground => {
                let left = tile.connects_to(self.left(i, j));
                let right = tile.connects_to(self.right(i,j));
                let below = tile.connects_to(self.below(i, j));
                let above = tile.connects_to(self.above(i, j));
                let below_left = tile.connects_to(self.below_left(i, j));
                let below_right = tile.connects_to(self.below_right(i, j));
                let above_left = tile.connects_to(self.above_left(i, j));
                let above_right = tile.connects_to(self.above_right(i, j));
                let image = match (above, left, right, below) {
                    (false, false, false, false) => "grass alone",
                    (false, true, false, false) => "grass left",
                    (false, false, true, false) => "grass right",
                    (false, true, true, false) => "grass left right",
                    (false, false, false, true) => "grass down",
                    (false, true, false, true) => "grass down left",
                    (false, false, true, true) => "grass down right",
                    (false, true, true, true) => "grass down left right",
                    (true, false, false, false) => "above",
                    (true, true, false, false) => "above left",
                    (true, false, true, false) => "above right",
                    (true, true, true, false) => "below empty",
                    (true, false, false, true) => "above below",
                    (true, true, false, true) => "right empty",
                    (true, false, true, true) => "left empty",
                    (true, true, true, true) => match (below_left, below_right, above_left, above_right) {
                        //TODO: missing some cases due to not having the images
                        (false, _, _, _) => "below left empty",
                        (_, false, _, _) => "below right empty",
                        (_, _, false, _) => "above left empty",
                        (_, _, _, false) => "above right empty",
                        (_, _, _, _) => "full",
                    }
                };
                let collision = image != "full";
                Some((SPRITES[ground_type][image], collision))
            }
        }
    }
}

impl std::ops::Deref for Map {
    type Target = [Line; 100];
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl std::ops::Index<usize> for Map {
    type Output = Line;
    fn index(&self, i: usize) -> &Self::Output {
        &self.values[i]
    }
}

pub fn read_map() -> Map {
    //TODO: read map from a file
    LEVEL_0
}

const LEVEL_0: Map = Map { values: [
    //turn your head to the right to read this
    [Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty],
    [Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
    [Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
]};