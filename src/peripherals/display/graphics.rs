use crate::peripherals::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Display, DisplayError};
use log::debug;

impl Display {
    pub async fn set_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<(), DisplayError> {
        debug!("Setting pixel at ({}, {}) to color 0x{:04X}", x, y, color);

        self.set_frame(x, y, x, y).await?;

        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;

        self.write_data(&[hi, lo]).await?;

        Ok(())
    }

    pub async fn set_background(&mut self, color: u16) -> Result<(), DisplayError> {
        debug!("Setting background color: 0x{:04X}", color);

        self.sleep().await?;

        self.set_frame(0, 0, DISPLAY_WIDTH - 1, DISPLAY_HEIGHT - 1)
            .await?;

        let hi = (color >> 8) as u8;
        let lo = (color & 0xFF) as u8;

        let mut buffer = [0u8; super::BUFFER_SIZE];
        for i in (0..super::BUFFER_SIZE).step_by(2) {
            buffer[i] = hi;
            buffer[i + 1] = lo;
        }

        for _row in 0..DISPLAY_HEIGHT {
            self.write_data(&buffer).await?;
        }

        self.wake().await?;

        Ok(())
    }

    // TODO: Use buffered writes for performance
    // TODO: Optimize vertical and horizontal lines
    pub async fn draw_line(
        &mut self,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        color: u16,
    ) -> Result<(), DisplayError> {
        debug!(
            "Drawing line from ({}, {}) to ({}, {}) with color 0x{:04X}",
            x1, y1, x2, y2, color
        );

        self.set_frame(x1, y1, x2, y2).await?;

        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            self.set_pixel(x, y, color).await?;

            if x == x2 && y == y2 {
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

    // FIXME: The created circle is not centered
    // TODO: Check bounds to avoid overflow
    pub async fn draw_circle(
        &mut self,
        x: u16,
        y: u16,
        radius: u16,
        color: u16,
    ) -> Result<(), DisplayError> {
        debug!(
            "Drawing circle at ({}, {}) with radius {} and color 0x{:04X}",
            x, y, radius, color
        );

        let mut f = 1 - radius as i32;
        let mut dd_f_x = 1;
        let mut dd_f_y = -2 * radius as i32;
        let mut x1 = 0;
        let mut y1 = radius as i32;

        while x1 < y1 {
            if f >= 0 {
                y1 -= 1;
                dd_f_y += 2;
                f += dd_f_y;
            }

            x1 += 1;
            dd_f_x += 2;
            f += dd_f_x;

            self.set_pixel(x + x1 as u16, y + y1 as u16, color).await?;
            self.set_pixel(x - x1 as u16, y + y1 as u16, color).await?;
            self.set_pixel(x + x1 as u16, y - y1 as u16, color).await?;
            self.set_pixel(x - x1 as u16, y - y1 as u16, color).await?;
            self.set_pixel(x + y1 as u16, y + x1 as u16, color).await?;
            self.set_pixel(x - y1 as u16, y + x1 as u16, color).await?;
            self.set_pixel(x + y1 as u16, y - x1 as u16, color).await?;
            self.set_pixel(x - y1 as u16, y - x1 as u16, color).await?;
        }

        self.set_pixel(x, y + radius, color).await?;
        self.set_pixel(x, y - radius, color).await?;
        self.set_pixel(x + radius, y, color).await?;
        self.set_pixel(x - radius, y, color).await?;

        Ok(())
    }
}
