use super::Figure;
use crate::peripherals::display::{
    BUFFER_SIZE, DISPLAY_HEIGHT, DISPLAY_WIDTH, Display, DisplayError,
};
use log::debug;

pub struct Line {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
    pub color: u16,
}

impl Figure for Line {
    async fn draw(&self, display: &mut Display) -> Result<(), DisplayError> {
        debug!(
            "Drawing line from ({}, {}) to ({}, {}) with color 0x{:04X}",
            self.x1, self.y1, self.x2, self.y2, self.color
        );

        display
            .set_frame(self.x1, self.y1, self.x2, self.y2)
            .await?;

        let dx = (self.x2 as i32 - self.x1 as i32).abs();
        let dy = (self.y2 as i32 - self.y1 as i32).abs();
        let sx = if self.x1 < self.x2 { 1 } else { -1 };
        let sy = if self.y1 < self.y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = self.x1;
        let mut y = self.y1;

        loop {
            display.set_pixel(x, y, self.color).await?;

            if x == self.x2 && y == self.y2 {
                break;
            }

            let err2 = err * 2;

            if err2 > -dy {
                err -= dy;
                x += sx as u16;
            }

            if err2 < dx {
                err += dx;
                y += sy as u16;
            }
        }

        Ok(())
    }
}

pub struct Circle {
    pub x: u16,
    pub y: u16,
    pub radius: u16,
    pub color: u16,
}

impl Figure for Circle {
    async fn draw(&self, display: &mut Display) -> Result<(), DisplayError> {
        debug!(
            "Drawing circle at ({}, {}) with radius {} and color 0x{:04X}",
            self.x, self.y, self.radius, self.color
        );

        let mut f = 1 - self.radius as i32;
        let mut dd_f_x = 1;
        let mut dd_f_y = -2 * self.radius as i32;
        let mut x1 = 0;
        let mut y1 = self.radius as i32;

        while x1 < y1 {
            if f >= 0 {
                y1 -= 1;
                dd_f_y += 2;
                f += dd_f_y;
            }

            x1 += 1;
            dd_f_x += 2;
            f += dd_f_x;

            display
                .set_pixel(self.x + x1 as u16, self.y + y1 as u16, self.color)
                .await?;
            display
                .set_pixel(self.x - x1 as u16, self.y + y1 as u16, self.color)
                .await?;
            display
                .set_pixel(self.x + x1 as u16, self.y - y1 as u16, self.color)
                .await?;
            display
                .set_pixel(self.x - x1 as u16, self.y - y1 as u16, self.color)
                .await?;
            display
                .set_pixel(self.x + y1 as u16, self.y + x1 as u16, self.color)
                .await?;
            display
                .set_pixel(self.x - y1 as u16, self.y + x1 as u16, self.color)
                .await?;
            display
                .set_pixel(self.x + y1 as u16, self.y - x1 as u16, self.color)
                .await?;
            display
                .set_pixel(self.x - y1 as u16, self.y - x1 as u16, self.color)
                .await?;
        }

        display
            .set_pixel(self.x, self.y + self.radius, self.color)
            .await?;
        display
            .set_pixel(self.x, self.y - self.radius, self.color)
            .await?;
        display
            .set_pixel(self.x + self.radius, self.y, self.color)
            .await?;
        display
            .set_pixel(self.x - self.radius, self.y, self.color)
            .await?;

        Ok(())
    }
}

impl Display {
    pub async fn draw<T>(&mut self, shape: &T) -> Result<(), DisplayError>
    where
        T: Figure,
    {
        shape.draw(self).await
    }

    pub async fn set_background(&mut self, color: u16) -> Result<(), DisplayError> {
        debug!("Setting background color: 0x{:04X}", color);

        self.sleep().await?;

        self.set_frame(0, 0, DISPLAY_WIDTH - 1, DISPLAY_HEIGHT - 1)
            .await?;

        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;

        let mut buffer = [0u8; BUFFER_SIZE];
        for i in (0..BUFFER_SIZE).step_by(2) {
            buffer[i] = hi;
            buffer[i + 1] = lo;
        }

        for _row in 0..DISPLAY_HEIGHT {
            self.write_data(&buffer).await?;
        }

        self.wake().await?;

        Ok(())
    }
}
