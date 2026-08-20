use std::iter;

use ggez::context::HasMut;
use ggez::error::GameResult;
use ggez::graphics::{ GraphicsContext, Image };

use glamour::Unit;

use num_traits::{ AsPrimitive, ConstOne, ConstZero, NumCast };

use ggez_pixel_canvas::{ AsPixel, PixelCanvas, PixelDrawParams };

pub trait Tile: Copy {
    type TileUnit: Unit<Scalar: AsPrimitive<usize>> + AsPixel<PixelType: Unit<Scalar: AsPrimitive<u32> + AsPrimitive<f32>>>;
    fn atlas_pos(&self) -> [ <Self::TileUnit as Unit>::Scalar; 2 ];
}

pub trait CharTile {
    fn from_char(c: char) -> Self;
}

pub trait MultiTile {
    type SubTile: Tile;
    fn dimensions(&self) -> [ <<Self::SubTile as Tile>::TileUnit as Unit>::Scalar; 2 ];
    fn sub(&self, x: <<Self::SubTile as Tile>::TileUnit as Unit>::Scalar, y: <<Self::SubTile as Tile>::TileUnit as Unit>::Scalar) -> Self::SubTile;
}

pub struct TileGrid<T: Tile, const W: usize, const H: usize> {
    pub grid: [[T; W]; H]
}
impl<T: Tile, const W: usize, const H: usize> TileGrid<T, W, H> {

    pub const fn fill(fill: T) -> Self {
        Self { grid: [[fill; W]; H] }
    }

    pub fn at(&self, x: <T::TileUnit as Unit>::Scalar, y: <T::TileUnit as Unit>::Scalar) -> T {
        let r: usize = y.as_();
        let c: usize = x.as_();
        self.grid[r][c]
    }

    pub fn at_mut(&mut self, x: <T::TileUnit as Unit>::Scalar, y: <T::TileUnit as Unit>::Scalar) -> &mut T {
        let r: usize = y.as_();
        let c: usize = x.as_();
        &mut self.grid[r][c]
    }

    pub const fn builder(self, x: <T::TileUnit as Unit>::Scalar, y: <T::TileUnit as Unit>::Scalar) -> TileGridBuilder<T, W, H> {
        TileGridBuilder { tg: self, x, y }
    }

    pub fn compose_image(&self, gfx: &mut impl HasMut<GraphicsContext>, atlas: &Image) -> GameResult<Image> {

        let gfx = gfx.retrieve_mut();
        let w = NumCast::from(W).unwrap();
        let h = NumCast::from(H).unwrap();
        let mut canvas = PixelCanvas::new::<T::TileUnit>(gfx, [ w, h ]);
        for x in range_from_zero::<T::TileUnit>(w) {
            for y in range_from_zero::<T::TileUnit>(h) {
                let [ atlas_x, atlas_y ] = self.at(x, y).atlas_pos();
                canvas.draw(atlas, PixelDrawParams::<<T::TileUnit as AsPixel>::PixelType>::default()
                    .dest::<T::TileUnit>([ x, y ])
                    .atlas_rect::<T::TileUnit>(([ atlas_x, atlas_y ], [ ConstOne::ONE, ConstOne::ONE ]))
                );
            }
        }
        canvas.finish(gfx)

    }

}

pub struct TileGridBuilder<T: Tile, const W: usize, const H: usize> {
    tg: TileGrid<T, W, H>,
    x: <T::TileUnit as Unit>::Scalar,
    y: <T::TileUnit as Unit>::Scalar,
}
impl<T: Tile, const W: usize, const H: usize> TileGridBuilder<T, W, H> {

    pub fn build(self) -> TileGrid<T, W, H> {
        self.tg
    }

    pub fn set_x(self, x: <T::TileUnit as Unit>::Scalar) -> Self {
        Self { tg: self.tg, x, y: self.y }
    }

    pub fn set_y(self, y: <T::TileUnit as Unit>::Scalar) -> Self {
        Self { tg: self.tg, x: self.x, y }
    }

    pub fn move_x(self, dx: <T::TileUnit as Unit>::Scalar) -> Self {
        let x = self.x;
        self.set_x(x + dx)
    }

    pub fn move_y(self, dy: <T::TileUnit as Unit>::Scalar) -> Self {
        let y = self.y;
        self.set_y(y + dy)
    }

    pub fn put(mut self, v: T) -> Self {
        *self.tg.at_mut(self.x, self.y) = v;
        self.move_x(ConstOne::ONE)
    }

    pub fn put_multi<M: MultiTile<SubTile = T>>(mut self, v: M) -> Self {
        let [ w, h ] = v.dimensions();
        for dx in range_from_zero::<T::TileUnit>(w) {
            for dy in range_from_zero::<T::TileUnit>(h) {
                *self.tg.at_mut(self.x + dx, self.y + dy) = v.sub(dx, dy);
            }
        }
        self.move_x(w)
    }

    pub fn write_multi<M: MultiTile<SubTile = T> + CharTile>(self, s: &str) -> Self {
        let initial_x = self.x;
        s.chars().fold(self, |view, character| match character {
            '\r' => view.set_x(initial_x),
            '\n' => view.move_y(ConstOne::ONE),
            c => view.put_multi(M::from_char(c)),
        })
    }

}
impl<T: CharTile + Tile, const W: usize, const H: usize> TileGridBuilder<T, W, H> {

    pub fn write(self, s: &str) -> Self {
        let initial_x = self.x;
        s.chars().fold(self, |view, character| match character {
            '\r' => view.set_x(initial_x),
            '\n' => view.move_y(ConstOne::ONE),
            c => view.put(T::from_char(c)),
        })
    }

}

fn range_from_zero<T: Unit>(max_exclusive: T::Scalar) -> Vec<T::Scalar> {
    let mut curr = ConstZero::ZERO;
    iter::from_fn(move || {
        if curr < max_exclusive { let res = Some(curr); curr += ConstOne::ONE; res } else { None }
    }).collect()
}