use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

// Simple 3x5 font for digits
const DIGITS: [&[&str]; 10] = [
    &["###", "# #", "# #", "# #", "###"], // 0
    &["  #", "  #", "  #", "  #", "  #"], // 1
    &["###", "  #", "###", "#  ", "###"], // 2
    &["###", "  #", "###", "  #", "###"], // 3
    &["# #", "# #", "###", "  #", "  #"], // 4
    &["###", "#  ", "###", "  #", "###"], // 5
    &["###", "#  ", "###", "# #", "###"], // 6
    &["###", "  #", "  #", "  #", "  #"], // 7
    &["###", "# #", "###", "# #", "###"], // 8
    &["###", "# #", "###", "  #", "###"], // 9
];

pub struct BigText<'a> {
    text: &'a str,
    style: Style,
}

impl<'a> BigText<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for BigText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 5 {
            return;
        }

        let mut current_x = area.x;

        for c in self.text.chars() {
            if c >= '0' && c <= '9' {
                let digit_idx = (c as u8 - b'0') as usize;
                let glyph = DIGITS[digit_idx];

                if current_x + 3 > area.x + area.width {
                    break;
                }

                for (y, row) in glyph.iter().enumerate() {
                    for (x, ch) in row.chars().enumerate() {
                        if ch == '#' {
                            if let Some(cell) = buf.cell_mut((current_x + x as u16, area.y + y as u16)) {
                                cell.set_char('█')
                                    .set_style(self.style);
                            }
                        }
                    }
                }

                current_x += 4; // 3 width + 1 spacing
            } else if c == '.' || c == ',' {
                if current_x + 1 > area.x + area.width {
                    break;
                }
                if let Some(cell) = buf.cell_mut((current_x, area.y + 4)) {
                    cell.set_char('█')
                        .set_style(self.style);
                }
                current_x += 2; // 1 width + 1 spacing
            } else if c == ' ' {
                current_x += 3;
            }
        }
    }
}
