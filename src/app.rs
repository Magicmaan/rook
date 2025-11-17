use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};
use ratatui::{layout::Position, prelude::Rect};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::{
    common::action::{Action, Search},
    components::{Component, results::ResultsBox, search::SearchBox, wizard::WizardBox},
    search_modules::{
        SearchModule, applications::desktop_files_module::DesktopFilesModule,
        maths::maths_module::MathsModule,
    },
    settings::settings::Settings,
    tui::{Event, Tui},
};

pub struct App {
    settings: Settings,
    tick_rate: f64,
    frame_rate: f64,
    components: Vec<Box<dyn Component>>,
    search_modules: Vec<Box<dyn SearchModule>>,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                Box::new(SearchBox::new()),
                Box::new(ResultsBox::new()),
                Box::new(WizardBox::new()),
            ],
            search_modules: vec![
                Box::new(DesktopFilesModule::new()),
                Box::new(MathsModule::new()),
            ],
            should_quit: false,
            should_suspend: false,
            settings: Settings::new(),
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            // .mouse(true) // uncomment this line to enable mouse support
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }
        for component in self.components.iter_mut() {
            component.register_settings_handler(self.settings.clone())?;
        }
        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume).unwrap();
                action_tx.send(Action::ClearScreen).unwrap();
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit).unwrap(),
            Event::Tick => action_tx.send(Action::Tick).unwrap(),
            Event::Render => action_tx.send(Action::Render).unwrap(),
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y)).unwrap(),
            Event::Mouse(mouse) => self.handle_mouse_event(mouse).unwrap(),
            Event::Key(key) => self.handle_key_event(key).unwrap(),
            _ => {}
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action).unwrap();
            }
        }
        Ok(())
    }
    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();

        fn contains(component: &Box<dyn Component>, mouse: &MouseEvent) -> bool {
            let area = component.area();
            log::info!("Component area: {:?}", area);
            area.contains(Position {
                x: mouse.column,
                y: mouse.row,
            })
        }

        // Currently, no mouse events are handled.
        match mouse.kind {
            MouseEventKind::Down(button) => {
                log::info!(
                    "Mouse button {:?} down at ({}, {})",
                    button,
                    mouse.column,
                    mouse.row
                );
                for component in self.components.iter_mut() {
                    if contains(component, &mouse) {
                        // info!("Component {:?} focused", component);
                        component.update(Action::Focus).unwrap();
                    } else {
                        // info!("Component {:?} unfocused", component);
                        // action_tx.send(Action::Unfocus).unwrap();
                        component.update(Action::Unfocus).unwrap();
                    }
                }
            }
            MouseEventKind::Moved => {
                log::info!("Mouse moved to ({}, {})", mouse.column, mouse.row);
                for component in self.components.iter_mut() {
                    if contains(component, &mouse) {
                        // info!("Component {:?} focused", component);
                        // action_tx.send(Action::Focus).unwrap();
                        component.update(Action::Focus).unwrap();
                    } else {
                        // info!("Component {:?} unfocused", component);
                        component.update(Action::Unfocus).unwrap();
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let keymap = self.settings.keybinds.clone().keybinds;
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone()).unwrap();
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone()).unwrap();
                }
            }
            None => {}
        }
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        let action_tx = self.action_tx.clone();
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }
            match &action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, *w, *h)?,
                Action::Render => self.render(tui)?,
                Action::Search(Search::Execute(query)) => {
                    self.search_modules.iter_mut().for_each(|module| {
                        let has_results = module.search(query).unwrap_or_else(|err| {
                            log::info!(
                                "Module {} failed to search for query: {}: {:?}",
                                module.name(),
                                query,
                                err
                            );
                            return false;
                        });
                        if has_results {
                            log::info!(
                                "Module {} found results for query: {}",
                                module.name(),
                                query
                            );
                            let results = module.get_ui_results();
                            info!("Results: {:?}", results);
                            action_tx.send(Action::SearchResults(results)).unwrap();
                        }
                    });
                }
                Action::ItemExecute(result) => {
                    info!("Executing result: {:?}", result);
                    result.launch.as_ref()();
                }
                _ => {}
            }
            for component in self.components.iter_mut() {
                if let Some(next_action) = component.update(action.clone()).unwrap() {
                    self.action_tx.send(next_action).unwrap();
                };
            }
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            for component in self.components.iter_mut() {
                if let Err(err) = component.draw(frame, frame.area()) {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("Failed to draw: {:?}", err)));
                }
            }
        })?;
        Ok(())
    }
}
