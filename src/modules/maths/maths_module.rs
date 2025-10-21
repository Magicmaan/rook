use shunting::ShuntingParser;

use crate::{
    model::module::{ModuleState, Result, UIState},
    modules::module::Module,
};

pub struct Equation {
    pub expression: String,
    pub result: String,
    pub valid: bool,
}

pub struct Data {
    pub equations: Vec<Equation>,
}

pub struct MathsModule {
    pub settings: crate::settings::settings::Settings,
    state: ModuleState,
    data: Data,
    context: shunting::MathContext,
}

impl MathsModule {
    pub fn new(settings: &crate::settings::settings::Settings) -> Self {
        let mut state = ModuleState::default();
        Self {
            settings: settings.clone(),
            state,
            data: Data {
                equations: Vec::new(),
            },
            context: shunting::MathContext::new(),
        }
    }
}

impl Module for MathsModule {
    fn get_state(&mut self) -> &mut ModuleState {
        &mut self.state
    }

    fn on_search(&mut self, query: &str) -> bool {
        let mut equation = Equation {
            expression: query.to_string(),
            result: "✕".to_string(),
            valid: false,
        };
        if query.is_empty() {
            self.state.search.results.clear();
        }

        let expr = ShuntingParser::parse_str(query);

        let result = if expr.is_ok() {
            match self.context.eval(&expr.unwrap()) {
                Ok(value) => {
                    self.state.is_candidate = true;
                    log::info!("Evaluated expression: {} = {}", query, value);
                    Equation {
                        expression: query.to_string(),
                        result: value.to_string(),
                        valid: true,
                    }
                }

                Err(_) => {
                    log::info!("Invalid expression: {}", query);
                    equation
                }
            }
        } else {
            log::info!("Failed to parse expression: {}", query);
            equation
        };
        self.data.equations.insert(0, result);

        let results_pointers = self
            .data
            .equations
            .iter()
            .enumerate()
            .map(|(idx, _)| (1, idx))
            .collect::<Vec<(u16, usize)>>();

        self.state.search.previous_query = query.to_string();
        self.state.search.previous_results = self.state.search.results.clone();
        self.state.search.results = results_pointers;
        self.state.ui.set_selected_result_index(0);

        log::info!("MathsModule is candidate for query {}", query);
        true
    }
    fn on_execute(&mut self) {}

    fn render(&mut self) -> &mut UIState {
        // let results_formatted = self
        //     .state
        //     .search
        //     .results
        //     .iter()
        //     .map(|score| {
        //         let s = score.0;
        //         let idx = score.1;

        //         let app = self.data.applications.get(idx).unwrap();

        //         Result {
        //             result: app.name.clone(),
        //             score: s.to_string(),
        //         }
        //     })
        //     .collect();

        self.state.ui.result_box_state.previous_results = self.state.ui.get_results().clone();
        // self.state.ui.set_results(results_formatted);

        self.state
            .ui
            .set_search_query(self.state.search.query.clone());

        self.state.ui.set_search_post_fix(
            self.data
                .equations
                .get(0)
                .map(|eq| format!("= {}", eq.result.clone()))
                .unwrap_or_default(),
        );
        &mut self.state.ui
    }
}
