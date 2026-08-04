mod tictactoe_test;
mod digit_test;

pub trait Tile {
    const SIZE: usize;
    fn atlas_pos(&self) -> [usize; 2];
}

pub trait CharTile {
    fn from_char(c: char) -> Self;
}

pub trait MultiTile {
    type SubTile: Tile;
    fn dimensions(&self) -> [ usize; 2 ];
    fn sub(&self, x: usize, y: usize) -> Self::SubTile;
}

pub struct TileGrid<T, const W: usize, const H: usize> {
    pub grid: [[T; W]; H]
}
impl<T, const W: usize, const H: usize> TileGrid<T, W, H> {

    pub fn view<'a>(&'a mut self, x: usize, y: usize) -> TileGridView<'a, T, W, H> {
        TileGridView { tg: self, x, y }
    }

}
impl<T: Clone + Copy, const W: usize, const H: usize> TileGrid<T, W, H> {

    pub fn fill(fill: T) -> Self {
        Self { grid: [[fill; W]; H] }
    }

}
impl<T: Tile, const W: usize, const H: usize> TileGrid<T, W, H> {

    pub fn compose_image(&self, atlas: &[u8], atlas_w_tiles: usize) -> Vec<u8> {

        let tile_w_bytes = T::SIZE * 4;
        let tile_h_bytes = T::SIZE;

        let res_w_bytes = W * tile_w_bytes;
        let res_h_bytes = H * tile_h_bytes;

        let atlas_w_bytes = atlas_w_tiles * tile_w_bytes;

        let mut res = vec![0; res_w_bytes * res_h_bytes ];

        for grid_x_tiles in 0..W {
            let grid_x_bytes = grid_x_tiles * tile_w_bytes;
            for grid_y_tiles in 0..H {
                let grid_y_bytes = grid_y_tiles * tile_h_bytes;

                let [ atlas_x_tiles, atlas_y_tiles ] = self.grid[grid_y_tiles][grid_x_tiles].atlas_pos();
                let atlas_x_bytes = atlas_x_tiles * tile_w_bytes;
                let atlas_y_bytes = atlas_y_tiles * tile_h_bytes;

                for offset_y in 0..tile_h_bytes {

                    let res_start   = (grid_y_bytes  + offset_y) * res_w_bytes   + grid_x_bytes;
                    let atlas_start = (atlas_y_bytes + offset_y) * atlas_w_bytes + atlas_x_bytes;
                    
                    let res_slice = &mut res[res_start  ..(res_start   + tile_w_bytes)];
                    let atlas_slice = &atlas[atlas_start..(atlas_start + tile_w_bytes)];

                    res_slice.copy_from_slice(atlas_slice);

                }

            }
        }

        res

    }

}

pub struct TileGridView<'a, T, const W: usize, const H: usize> {
    tg: &'a mut TileGrid<T, W, H>,
    x: usize,
    y: usize,
}
impl<'a, T, const W: usize, const H: usize> TileGridView<'a, T, W, H> {

    pub fn set_x(self, x: usize) -> Self {
        Self { tg: self.tg, x, y: self.y }
    }

    pub fn set_y(self, y: usize) -> Self {
        Self { tg: self.tg, x: self.x, y }
    }

    pub fn move_x(self, dx: i32) -> Self {
        let new_x = (self.x as i32 + dx) as usize;
        self.set_x(new_x)
    }

    pub fn move_y(self, dy: i32) -> Self {
        let new_y = (self.y as i32 + dy) as usize;
        self.set_y(new_y)
    }

    pub fn put(self, v: T) -> Self {
        self.tg.grid[self.y][self.x] = v;
        self.move_x(1)
    }

    pub fn put_multi<M: MultiTile<SubTile = T>>(self, v: M) -> Self {
        let [ tw, th ] = v.dimensions();
        for dx in 0..tw {
            for dy in 0..th {
                self.tg.grid[self.y + dy][self.x + dx] = v.sub(dx, dy);
            }
        }
        self.move_x(tw as i32)
    }

    pub fn write_multi<M: MultiTile<SubTile = T> + CharTile>(self, s: &str) -> Self {
        let initial_x = self.x;
        s.chars().fold(self, |view, character| match character {
            '\r' => view.set_x(initial_x),
            '\n' => view.move_y(1),
            c => view.put_multi(M::from_char(c)),
        })
    }

}
impl<'a, T: CharTile, const W: usize, const H: usize> TileGridView<'a, T, W, H> {

    pub fn write(self, s: &str) -> Self {
        let initial_x = self.x;
        s.chars().fold(self, |view, character| match character {
            '\r' => view.set_x(initial_x),
            '\n' => view.move_y(1),
            c => view.put(T::from_char(c)),
        })
    }

}