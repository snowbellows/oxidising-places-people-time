use nannou::{
    image::{self, GenericImageView},
    prelude::*,
};

use crate::utils::pixel_to_hsla;

#[derive(Debug)]
pub struct Cell {
    pub position: Vec2,
    pub colour: Hsla,
    finished: bool,
    scale: f32,
}

impl Cell {
    pub fn finished(&mut self) {
        self.finished = true;
    }
}

pub struct ImageGrid {
    cell_size: u32,
    cells: Vec<Vec<Cell>>,
    size_rect: Rect,
}

impl ImageGrid {
    pub fn new(size_rect: &Rect, cell_size: u32, base_image: &image::DynamicImage) -> Self {
        let grid_width = size_rect.w() as u32 / cell_size;
        let grid_height = size_rect.h() as u32 / cell_size;
        let cells: Vec<Vec<Cell>> = (0..grid_width)
            .map(|grid_x| {
                (0..grid_height)
                    .map(move |grid_y| {
                        let pos_x = size_rect.left()
                            + (cell_size * grid_x) as f32
                            + (cell_size as f32 / 2.0);
                        let pos_y = size_rect.top()
                            - (cell_size * grid_y) as f32
                            - (cell_size as f32 / 2.0);

                        let pixel = base_image.get_pixel(
                            base_image.width() / grid_width * grid_x,
                            base_image.height() / grid_height * grid_y,
                        );

                        let colour = pixel_to_hsla(pixel);

                        Cell {
                            position: vec2(pos_x, pos_y),
                            colour,
                            finished: false,
                            scale: 1.0 - colour.lightness,
                        }
                    })
                    .collect()
            })
            .collect();

        ImageGrid {
            cell_size,
            cells,
            size_rect: *size_rect,
        }
    }

    pub fn draw(&self, draw: &Draw, scale_factor: f32) {
        for cell in self.cells.iter().flatten() {
            let w = map_range(
                cell.scale,
                0.0,
                1.0,
                0.0,
                self.cell_size as f32 * scale_factor,
            );
            draw.xy(cell.position)
                .ellipse()
                .w_h(w, w)
                .color(cell.colour);
        }
    }

    pub fn add_image_centre(&mut self, image: &image::DynamicImage, size: Vec2) {
        let image_rect = Rect::from_wh(size).middle_of(self.size_rect).shift(vec2(
            random_range(-100.0, 100.0),
            random_range(-100.0, 100.0),
        ));

        let mut centre: Vec<Vec<&mut Cell>> = self
            .cells
            .iter_mut()
            .map(|vc| {
                vc.iter_mut()
                    .filter(|c| image_rect.contains(c.position))
                    .collect()
            })
            .filter(|vc: &Vec<&mut Cell>| !vc.is_empty())
            .collect();

        let width = centre.len() - 1;
        let height = centre[0].len() - 1;

        for (x, column) in centre.iter_mut().enumerate() {
            for (y, cell) in column.iter_mut().enumerate() {
                let pixel = image.get_pixel(
                    map_range(x, 0, width, 0, image.width() - 1),
                    map_range(y, 0, height, 0, image.height() - 1),
                );

                let colour = pixel_to_hsla(pixel);

                cell.scale = 1.0 - colour.lightness;
            }
        }
    }

    pub fn iter_mut_cells(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut().flatten().filter(|c| !c.finished)
    }
}
