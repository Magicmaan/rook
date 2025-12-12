use std::{collections::VecDeque, rc::Rc, vec};

use nucleo::{Config, Matcher};
use shunting::ShuntingParser;

use crate::{
    search_modules::{ListResult, SearchModule},
    settings::settings::Settings,
};
use color_eyre::Result;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Equation {
    pub expression: String,
    pub result: String,
}
impl Equation {
    pub fn launch(&self) -> bool {
        log::info!("Launching equation result: {}", self.result);
        // no launch action for equations
        true
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]

pub struct MathsData {
    pub equations: VecDeque<Equation>,
}

pub struct MathsModule {
    data: Box<MathsData>,
    context: shunting::MathContext,
    time_since_last_eval: std::time::Instant,
}

impl MathsModule {
    pub fn new() -> Self {
        Self {
            data: Box::new(MathsData::default()),
            context: shunting::MathContext::new(),
            time_since_last_eval: std::time::Instant::now(),
        }
    }

    pub fn test_for_duplicate(&mut self, equation: &mut Equation) -> Result<(bool, usize)> {
        let mut matcher = Matcher::new(Config::DEFAULT);
        if self.data.equations.is_empty() {
            return Ok((false, 0));
        }

        let default = Equation::default();

        let expression = equation.expression.to_string();

        for i in 0..self.data.equations.len() {
            let compare_equation = self.data.equations.get(i).unwrap_or(&default);

            let compare_expression = compare_equation.expression.to_string();

            // exact match then return duplicate found
            if expression == compare_expression {
                return Ok((true, i));
            }
            let substring_match = matcher
                .substring_match(
                    nucleo::Utf32Str::new(&expression, &mut vec![]),
                    nucleo::Utf32Str::new(&compare_expression, &mut vec![]),
                )
                .unwrap_or_default();

            if substring_match > 0 {
                return Ok((true, i));
            }
        }

        Ok((false, 0))
    }
}

impl SearchModule for MathsModule {
    fn name(&self) -> &str {
        "maths_module"
    }

    fn search(&mut self, query: &str) -> Result<bool> {
        if query.is_empty() {
            return Err(color_eyre::eyre::eyre!("Empty query"));
        }

        let formatted_query = query.trim().replace(" ", "");
        let equation = &mut Equation {
            expression: formatted_query.clone(),
            result: "✕".to_string(),
        };

        let expr = ShuntingParser::parse_str(formatted_query.as_str());

        let result = if expr.is_ok() {
            match self.context.eval(&expr.unwrap()) {
                Ok(value) => {
                    // block to prevent expressions that are just numbers
                    log::info!("Evaluated expression: {} = {}", query, value);
                    if query.parse::<f64>().is_ok() {
                        return Err(color_eyre::eyre::eyre!("Expression is just a number"));
                    }
                    equation.result = value.to_string();
                    equation
                }
                Err(_) => {
                    return Err(color_eyre::eyre::eyre!(
                        "Expression cannot be just a number"
                    ));
                }
            }
        } else {
            return Err(color_eyre::eyre::eyre!(
                "Expression cannot be just a number"
            ));
        };

        let is_duplicate = self.test_for_duplicate(result)?;
        if is_duplicate.0 {
            log::trace!(
                "Duplicate equation found: {} = {}",
                result.expression,
                result.result
            );
            return Ok(false);
        } else {
            self.data.equations.push_front(result.clone());
        }

        if self.data.equations.len() > 100 {
            let _ = self.data.equations.split_off(99);
        }

        log::info!("MathsModule is candidate for query {}", query);

        self.time_since_last_eval = std::time::Instant::now();

        Ok(true)
    }

    fn get_ui_results(&self) -> Vec<ListResult> {
        self.data
            .equations
            .iter()
            .enumerate()
            .map(|(idx, eq)| {
                ListResult {
                    result: format!("{} = {}", eq.expression, eq.result),
                    score: idx as u16,
                    // source_module: self.name().to_string(),
                    launch: Rc::new(|| false),
                }
            })
            .collect()
    }
}
