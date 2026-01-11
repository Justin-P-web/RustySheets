use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect, Spacing}, widgets::{Block, Paragraph, Widget}};

pub struct Grid {
    pub(crate) cols: usize,
    pub(crate) rows: usize,
}

impl Widget for Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(9));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_constraints).spacing(Spacing::Overlap(1));
        let vertical = Layout::vertical(row_constraints).spacing(Spacing::Overlap(1));

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            Paragraph::new(format!("Area {:02}", i + 1))
                .block(Block::bordered())
                .render(cell, buf);
        }
    }
}