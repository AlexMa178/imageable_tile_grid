#![cfg(feature = "ggez")]

use ggez::context::HasMut;
use ggez::error::GameResult;
use ggez::graphics::{ GraphicsContext, Canvas, Color, DrawParam, Image, Rect, Sampler };

use ggez::mint::Point2;

use crate::{ Tile, TileGrid };

impl<T: Tile, const W: usize, const H: usize> TileGrid<T, W, H> {

    fn compose_image_ggez(&self, gfx: &mut impl HasMut<GraphicsContext>, atlas: Image) -> GameResult<Image> {

        let gfx = gfx.retrieve_mut();

        let canvas_image = Image::new_canvas_image(gfx, (W * T::SIZE) as u32, (H * T::SIZE) as u32, 1);
        let mut canvas = Canvas::from_image(gfx, canvas_image.clone(), Some(Color::from_rgba(0, 0, 0, 0)));
        canvas.set_sampler(Sampler::nearest_clamp());
        for grid_x in 0..W {
            for grid_y in 0..H {
                let [ atlas_x, atlas_y ] = self.grid[grid_y][grid_x].atlas_pos();
                canvas.draw(&atlas, DrawParam::default()
                    .src(Rect {
                        x: (T::SIZE * atlas_x) as f32 / atlas.width() as f32,
                        y: (T::SIZE * atlas_y) as f32 / atlas.width() as f32,
                        w: T::SIZE as f32 / atlas.width() as f32,
                        h: T::SIZE as f32 / atlas.height() as f32,
                    }).dest(Point2 {
                        x: (T::SIZE * grid_x) as f32,
                        y: (T::SIZE * grid_y) as f32,
                    }));
            }
        }
        canvas.finish(gfx)?;
        Ok(canvas_image)

    }

}