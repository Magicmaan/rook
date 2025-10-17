use ratatui::widgets::{StatefulWidget, Widget};

use crate::{
    events::Event,
    ui::{results_box::ResultsBox, search_box::SearchBox},
};

pub trait Update {
    fn update(&mut self, events: &Vec<Event>);
}

pub trait UI {
    fn search_box(&self) -> &dyn StatefulWidget<State = crate::model::ui::UIState> {}
    fn results_box(&self) -> &dyn StatefulWidget<State = crate::model::ui::UIState> {}
}
