use ratatui::{
    buffer::Buffer, crossterm::terminal::Clear, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Widget}
};
use std::time::SystemTime;

use crate::{model, settings::{UIResultsSettings, UISearchSettings}};
#[derive(Clone, Default)]
pub struct ResultsBox {
	results: Vec<(u16, usize)>,
	data: crate::model::Data,
	index: usize,
    settings: UIResultsSettings,
}

impl ResultsBox {
    pub fn new(
        results: Vec<(u16, usize)>,
		data: &crate::model::Data,
		index: usize,
        settings: UIResultsSettings,
    ) -> Self {
        Self {
            results,
			data: data.clone(),
            index,
            settings,
        }
    }
}

impl Widget for ResultsBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            // .title("Search")
            .padding(Padding::new(1,1,1,1));
        // block.render(area, buf);
        let inner_area = block.inner(area);

		let items = self.results.iter().map(|(score, idx)| {
			let app = &self.data.applications[*idx];
			ListItem::new(format!("{} - {} ({})", app.name, app.exec, score))
		}).collect::<Vec<ListItem>>();

		let list = List::new(items)
			.style(Style::default().fg(Color::White));
		
		
        list.render(inner_area, buf);
        block.render(area, buf);


    }
}
