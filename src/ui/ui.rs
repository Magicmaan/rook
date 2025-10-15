use ratatui::{
    layout::{Constraint, Layout}, widgets::{Block, Clear, Paragraph}, Frame
};

use crate::{model, ui::search_box::SearchBox};

pub struct UI {}

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, model: &mut model::Model, frame: &mut Frame) {
        let ui_settings = &model.settings.ui;
        // println!("Drawing UI at tick: {}, delta_time: {}ms", model.tick, model.delta_time);
        // Draw the UI components
        let area = frame.area();
        frame.render_widget(Clear, area);

        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let chunks = layout.split(area);

        let search_box = SearchBox::new(
            model.search.query.clone(),
            Some(model.ui.caret_position),
            ui_settings.search.clone(),
        );
        let mut i = 0;
     
        let result_box = crate::ui::results_box::ResultsBox::new(
            model.search.results.clone(),
            &model.data,
            0,
            ui_settings.results.clone(),
        );

        frame.render_widget(search_box, chunks[0]);
        frame.render_widget(result_box, chunks[1]);
    }
}
