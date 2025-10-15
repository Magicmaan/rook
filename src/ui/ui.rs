use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Padding, Paragraph},
};

use crate::{model, ui::search_box::SearchBox};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UISection {
    Search,
    Results,
    Tooltip,
    ResultItem(usize), // index in results
}

pub struct UI {}

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, model: &mut model::Model, frame: &mut Frame) {
        let ui_settings = &model.settings.ui;
        // println!("Drawing UI at tick: {}, delta_time: {}ms", model.tick, model.delta_time);
        // Draw the UI components

        let root = Block::new()
            .style(ui_settings.theme.get_default_style(None))
            .padding(Padding::new(2, 2, 1, 1));
        let area = root.inner(frame.area());

        let layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let chunks: Rc<[Rect]> = layout.split(area);

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

        frame.render_widget(root, frame.area());

        frame.render_widget(search_box, chunks[0]);
        // frame.render_widget(result_box, chunks[1]);
        // model.ui.result_list_state.select(Some(2));
        frame.render_stateful_widget(result_box, chunks[1], &mut model.ui.result_list_state);

        // let mut rects = Vec::new();
        // rects.push((UISection::Search, chunks[0]));
        // rects.push((UISection::Results, chunks[1]));
        // let results_layout = Layout::vertical(
        //     model
        //         .search
        //         .results
        //         .iter()
        //         .map(|_| Constraint::Length(1))
        //         .collect::<Vec<Constraint>>(),
        // );
        // model.ui.sections = rects.clone();
    }
}
