use ratatui::widgets::StatefulWidget;

use crate::{
    events::Event,
    model::model::Model,
    modules::module::{UI, Update},
    ui::{results_box::ResultsBox, search_box::SearchBox},
};

struct ApplicationModule {
    model: Model,

    search_box: SearchBox,
    results_box: ResultsBox,
}

impl Update for ApplicationModule {
    fn update(&mut self, events: &Vec<Event>) {
        // Update logic for the application module
        // this will handle events and update the model accordingly
    }
}
impl UI for ApplicationModule {
    fn search_box(&self) -> &dyn StatefulWidget<State = crate::model::ui::UIState> {
        &self.search_box
    }

    fn results_box(&self) -> &dyn StatefulWidget<State = crate::model::ui::UIState> {
        &self.results_box
    }
}
