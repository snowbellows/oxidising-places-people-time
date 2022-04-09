use nannou::{
    color::blend::Blend,
    image::{self, GenericImageView},
    prelude::*,
};

#[derive(Debug)]
pub struct Cell {
    pub position: Vec2,
    pub colour: Hsla,
    pub original_colour: Hsla,
    finished: bool,
    scale: f32,
}

impl Cell {
    pub fn x(&self) -> f32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
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

                        let base_colour = base_image.get_pixel(
                            base_image.width() / grid_width * grid_x,
                            base_image.height() / grid_height * grid_y,
                        );

                        let red = base_colour[0] as f32 / 255.0;
                        let green = base_colour[1] as f32 / 255.0;
                        let blue = base_colour[2] as f32 / 255.0;

                        let colour: Hsla = srgba(red, green, blue, 1.0).into();

                        Cell {
                            position: vec2(pos_x, pos_y),
                            colour,
                            original_colour: colour,
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
            draw.x_y(cell.x(), cell.y())
                .ellipse()
                .w_h(w, w)
                .color(cell.colour);
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

                let colour: Hsla = srgba(red, green, blue, 1.0).into();

                cell.colour = colour; //Gradient::new([cell.colour, colour]).get(0.5);
                cell.original_colour = colour;
                cell.scale = (cell.scale + 1.0 - colour.lightness) / 2.0;
            }
        }
    }

    pub fn add_cell_colours<F>(&mut self, func: F)
    where
        F: Fn(&Cell) -> Option<Hsla>,
    {
        for cell in self.cells.iter_mut().flatten().filter(|c| !c.finished) {
            if let Some(new_colour) = func(&cell) {
                // cell.colour.hue = new_colour.hue;
                // cell.colour.saturation += new_colour.saturation / 10.0;
                // cell.colour.lightness += new_colour.lightness / 10.0;
                // new_colour.hue += random_range(-10.0, 10.0);

                cell.colour = new_colour;
                cell.colour.hue += random_range(-10.0, 10.0);
                cell.colour.saturation += random_range(-0.1, 0.1);
                cell.colour.lightness += random_range(-0.1, 0.1);

                cell.finished = true;

                // cell.colour =  LinSrgba::from(cell.original_colour).overlay(new_colour.into()).into();
                // cell.colour =  LinSrgba::from(new_colour).overlay(cell.original_colour.into()).into();

                // cell.colour =  LinSrgba::from(new_colour).multiply(cell.original_colour.into()).into(); // Option 1
                // cell.colour =  LinSrgba::from(cell.original_colour).overlay(new_colour.into()).into(); // Option 2
                // cell.colour = new_colour;
                // cell.scale = 1.0 - cell.colour.lightness;
            }
        }
    }
}
