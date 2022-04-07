use nannou::{
    image::{self, GenericImageView},
    prelude::*,
};

#[derive(Debug)]
pub struct Cell {
    position: Vec2,
    colour: Hsla,
    scale: f32,
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

        let cells = (0..grid_width)
            .map(|grid_x| {
                (0..grid_height)
                    .map(move |grid_y| {
                        let pos_x = size_rect.left()
                            + (cell_size * grid_x) as f32
                            + (cell_size as f32 / 2.0);
                        let pos_y = size_rect.top()
                            - (cell_size * grid_y) as f32
                            - (cell_size as f32 / 2.0);

                        let base_colour = base_image.get_pixel(
                            base_image.width() / grid_width * grid_x,
                            base_image.height() / grid_height * grid_y,
                        );

                        let red = base_colour[0] as f32 / 255.0;
                        let green = base_colour[1] as f32 / 255.0;
                        let blue = base_colour[2] as f32 / 255.0;

                        let scale = red * 0.222 + green * 0.707 + blue * 0.071;

                        let colour = srgba(red, green, blue, 1.0).into();

                        Cell {
                            position: vec2(pos_x, pos_y),
                            colour,
                            scale,
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
        for column in &self.cells {
            for cell in column {
                let w = map_range(
                    cell.scale,
                    0.0,
                    1.0,
                    self.cell_size as f32 * scale_factor,
                    0.0,
                );
                draw.ellipse()
                    .x_y(cell.position.x, cell.position.y)
                    .w_h(w, w)
                    .color(cell.colour);
            }
        }
    }

    pub fn add_image_centre(&mut self, image: &image::DynamicImage, size: Vec2) {
        let image_rect = Rect::from_wh(size).middle_of(self.size_rect);

        let mut centre: Vec<Vec<&mut Cell>> = self
            .cells
            .iter_mut()
            .map(|vc| {
                vc.iter_mut()
                    .filter(|c| image_rect.contains(c.position))
                    .collect()
            })
            .filter(|vc: &Vec<&mut Cell>| vc.len() > 0)
            .collect();

        let width = centre.len() - 1;
        let height = centre[0].len() - 1;

        for (x, column) in centre.iter_mut().enumerate() {
            for (y, cell) in column.iter_mut().enumerate() {
                let base_colour = image.get_pixel(
                    map_range(x, 0, width, 0, image.width() - 1),
                    map_range(y, 0, height, 0, image.height() - 1),
                );

                let red = base_colour[0] as f32 / 255.0;
                let green = base_colour[1] as f32 / 255.0;
                let blue = base_colour[2] as f32 / 255.0;

                let scale = red * 0.222 + green * 0.707 + blue * 0.071;

                let colour = srgba(red, green, blue, 1.0).into();

                cell.colour = colour; //Gradient::new([cell.colour, colour]).get(0.5);
                cell.scale = (cell.scale + scale) / 2.0;
            }
        }
    }
}
