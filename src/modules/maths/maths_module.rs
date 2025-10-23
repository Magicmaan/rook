use std::collections::VecDeque;

use nucleo::{Config, Matcher};
use shunting::ShuntingParser;

use crate::{
    model::{
        app_state::AppState,
        module_state::{ModuleState, Result, UIState, UIStateUpdate},
    },
    modules::module::{Module, ModuleData},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Equation {
    pub expression: String,
    pub result: String,
    pub valid: bool,
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]

pub struct MathsData {
    pub equations: VecDeque<Equation>,
}
impl ModuleData for MathsData {}

pub struct MathsModule {
    pub settings: crate::settings::settings::Settings,
    state: ModuleState,
    data: Box<MathsData>,
    context: shunting::MathContext,
    time_since_last_eval: std::time::Instant,
}

impl MathsModule {
    pub fn new(settings: &crate::settings::settings::Settings) -> Self {
        let state = ModuleState::default();
        Self {
            settings: settings.clone(),
            state,
            data: Box::new(MathsData::default()),
            context: shunting::MathContext::new(),
            time_since_last_eval: std::time::Instant::now(),
        }
    }

    pub fn test_duplicates(&mut self, equation: &mut Equation) {
        let mut matcher = Matcher::new(Config::DEFAULT);
        if !self.data.equations.is_empty() {
            let result_full = equation.expression.to_string();
            for i in 0..(2.min(self.data.equations.len())) {
                let default = Equation {
                    expression: "".to_string(),
                    result: "".to_string(),
                    valid: false,
                };
                let eq = self.data.equations.get(i).unwrap_or(&default);
                let previous_full = eq.expression.to_string();

                if result_full == previous_full {
                    equation.valid = false;
                    continue;
                }
                let substring_match_1 = matcher
                    .substring_match(
                        nucleo::Utf32Str::new(&result_full, &mut vec![]),
                        nucleo::Utf32Str::new(&previous_full, &mut vec![]),
                    )
                    .unwrap_or_default();

                let substring_match = matcher
                    .substring_match(
                        nucleo::Utf32Str::new(&result_full, &mut vec![]),
                        nucleo::Utf32Str::new(&previous_full, &mut vec![]),
                    )
                    .unwrap_or_else(|| substring_match_1);
                if substring_match > 0 {
                    equation.valid = false;
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
    type State = ModuleState;
    fn get_state(&mut self) -> &mut ModuleState {
        &mut self.state
    }

    fn on_search(&mut self, query: &str, app_state: &AppState) -> bool {
        let mut candidacy = false;
        if query.is_empty() {
            return false;
        }

        let formatted_query = query.trim().replace(" ", "");
        let equation = &mut Equation {
            expression: formatted_query.clone(),
            result: "✕".to_string(),
            valid: false,
        };

        let expr = ShuntingParser::parse_str(formatted_query.as_str());

        let result = if expr.is_ok() {
            match self.context.eval(&expr.unwrap()) {
                Ok(value) => {
                    log::info!("Evaluated expression: {} = {}", query, value);
                    if query.parse::<f64>().is_ok() {
                        candidacy = false;
                        &mut Equation {
                            expression: formatted_query,
                            result: value.to_string(),
                            valid: false,
                        }
                    } else {
                        candidacy = true;
                        &mut Equation {
                            expression: formatted_query,
                            result: value.to_string(),
                            valid: true,
                        }
                    }
                }

                Err(_) => {
                    if formatted_query.contains(['+', '-', '*', '/']) {
                        // log::info!("MathsModule is candidate for query {}", query);
                        candidacy = true;
                    } else {
                        candidacy = false;
                    }
                    // self.state.is_candidate = false;
                    log::info!("Invalid expression: {}", query);
                    equation
                }
            }
        } else {
            candidacy = false;
            log::info!("Failed to parse expression: {}", query);
            equation
        };
        // self.test_duplicates(result);

        if result.valid {
            self.data.equations.push_front(result.clone());
            candidacy = true;
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

        self.state.results = results_pointers.clone();

        log::info!("MathsModule is candidate for query {}", query);

        self.time_since_last_eval = std::time::Instant::now();

        candidacy
    }
    fn on_execute(&mut self, _: &mut AppState) -> bool {
        true
    }

    fn render(&mut self) -> UIStateUpdate {
        let results_formatted = self
            .state
            .results
            .iter()
            .map(|score| {
                let s = score.0;
                let idx = score.1;

                let equation = self.data.equations.get_mut(idx).unwrap();

                if equation.valid {
                    Some(Result {
                        result: format!(
                            "{} = {}",
                            equation.expression.clone(),
                            equation.result.clone()
                        ),
                        score: s.to_string(),
                    })
                } else {
                    None
                }
            })
            .filter_map(|x| x)
            .collect();

        UIStateUpdate {
            post_fix: self.data.equations.front().map_or("".to_string(), |e| {
                if e.valid {
                    format!("= {}", e.result)
                } else {
                    "".to_string()
                }
            }),
            results: results_formatted,
        }
    }
}
