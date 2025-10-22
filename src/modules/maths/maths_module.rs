use std::collections::VecDeque;

use nucleo::{Config, Matcher};
use shunting::ShuntingParser;

use crate::{
    model::{
        model::Model,
        module::{ModuleState, Result, UIState},
    },
    modules::module::Module,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Equation {
    pub expression: String,
    pub result: String,
    pub valid: bool,
}

pub struct Data {
    pub equations: VecDeque<Equation>,
}

pub struct MathsModule {
    pub settings: crate::settings::settings::Settings,
    state: ModuleState,
    data: Data,
    context: shunting::MathContext,
    time_since_last_eval: std::time::Instant,
}

impl MathsModule {
    pub fn new(settings: &crate::settings::settings::Settings) -> Self {
        let state = ModuleState::default();
        Self {
            settings: settings.clone(),
            state,
            data: Data {
                equations: VecDeque::new(),
            },
            context: shunting::MathContext::new(),
            time_since_last_eval: std::time::Instant::now(),
        }
    }

    pub fn test_duplicates(&mut self, equation: &Equation) {
        let mut matcher = Matcher::new(Config::DEFAULT);
        if self.data.equations.len() > 0 {
            let result_full = format!("{}", equation.expression);
            for i in 0..(2.min(self.data.equations.len())) {
                let default = Equation {
                    expression: "".to_string(),
                    result: "".to_string(),
                    valid: false,
                };
                let eq = self.data.equations.get(i).unwrap_or(&default);
                let previous_full = format!("{}", eq.expression);

                if result_full == previous_full {
                    self.data.equations.remove(i);
                }
                let substring_match_1 = matcher
                    .substring_match(
                        nucleo::Utf32Str::new(&result_full, &mut vec![]),
                        nucleo::Utf32Str::new(&previous_full, &mut vec![]),
                    )
                    .unwrap_or_default();

                let substring_match = substring_match_1;
                if substring_match > 0 {
                    self.data.equations.remove(i);
                }
                log::info!(
                    "Substring match score between '{}' and '{}' is {}",
                    result_full,
                    previous_full,
                    substring_match
                );
            }
        }
    }
}

impl Module for MathsModule {
    fn get_state(&mut self) -> &mut ModuleState {
        &mut self.state
    }

    fn on_search(&mut self, query: &str, _: &Model) -> bool {
        let mut candidacy = false;
        let formatted_query = query.trim().replace(" ", "");
        let equation = Equation {
            expression: formatted_query.clone(),
            result: "✕".to_string(),
            valid: false,
        };
        if query.is_empty() {
            self.state.search.results.clear();
        }

        let expr = ShuntingParser::parse_str(formatted_query.as_str());

        let result = if expr.is_ok() {
            match self.context.eval(&expr.unwrap()) {
                Ok(value) => {
                    self.state.is_candidate = true;
                    log::info!("Evaluated expression: {} = {}", query, value);
                    candidacy = true;
                    Equation {
                        expression: formatted_query,
                        result: value.to_string(),
                        valid: true,
                    }
                }

                Err(_) => {
                    log::info!("Invalid expression: {}", query);
                    if equation.expression.contains(['+', '-', '*', '/']) {
                        candidacy = true;
                    } else {
                        candidacy = false;
                    }
                    equation
                }
            }
        } else {
            log::info!("Failed to parse expression: {}", query);
            candidacy = false;
            equation
        };
        self.test_duplicates(&result.clone());
        if candidacy {
            self.data.equations.push_front(result);
        }

        if self.data.equations.len() > 100 {
            self.data.equations.pop_back();
        }

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

        self.time_since_last_eval = std::time::Instant::now();
        candidacy
    }
    fn on_execute(&mut self, _: &Model) -> bool {
        true
    }

    fn render(&mut self) -> &mut UIState {
        let results_formatted = self
            .state
            .search
            .results
            .iter()
            .map(|score| {
                let s = score.0;
                let idx = score.1;

                let equation = self.data.equations.get(idx).unwrap();

                Result {
                    result: format!(
                        "{} = {}",
                        equation.expression.clone(),
                        equation.result.clone()
                    ),
                    score: s.to_string(),
                }
            })
            .collect();

        self.state.ui.result_box_state.previous_results = self.state.ui.get_results().clone();
        self.state.ui.set_results(results_formatted);

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
