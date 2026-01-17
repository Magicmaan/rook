use color_eyre::Result;
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};
use ratatui::{
    layout::{Constraint, Position},
    prelude::Rect,
};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Add, Sub},
    rc::Rc,
    sync::Arc,
};
use tokio::sync::{Mutex, mpsc};
use tracing::{debug, info};

use crate::{
    action::{Action, Search},
    components::{Component, results::ResultsBox, search::SearchBox, wizard::WizardBox},
    database::Database,
    search_modules::{
        SearchModule, applications::desktop_files_module::DesktopFilesModule,
        maths::maths_module::MathsModule,
    },
    settings::settings::{SerializableKeyEvent, Settings, get_settings_path},
    tui::{Event, Tui},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FocusArea {
    WizardBox = 0,
    Search = 1,
}
impl FocusArea {
    pub const fn length() -> u8 {
        2
    }
}

impl From<u8> for FocusArea {
    fn from(value: u8) -> Self {
        match value {
            0 => FocusArea::WizardBox,
            1 => FocusArea::Search,
            _ => FocusArea::Search, // default to Search if out of range
        }
    }
}

impl Add<u8> for FocusArea {
    type Output = FocusArea;

    fn add(self, rhs: u8) -> FocusArea {
        let i = ((self as u8).saturating_add(rhs) % FocusArea::length()) as u8;
        let s = FocusArea::from(i);
        s
    }
}

impl Sub<u8> for FocusArea {
    type Output = FocusArea;

    fn sub(self, rhs: u8) -> FocusArea {
        let i = ((self as u8).saturating_sub(rhs) % FocusArea::length()) as u8;
        let s = FocusArea::from(i);
        s
    }
}

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
    last_tick_mouse_events: Vec<MouseEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    focused_area: Option<FocusArea>,
    database: Arc<Mutex<Database>>,
    root_layout: crate::common::layout::RootLayout,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub async fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        let settings = Settings::new();
        let database_path = get_settings_path()
            .join("rook.db")
            .to_string_lossy()
            .to_string();
        log::info!("Database path: {:?}", database_path);
        let database = Arc::new(Mutex::new(Database::new(&database_path)?));
        database.lock().await.initialise()?;

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
            settings,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            last_tick_mouse_events: Vec::new(),
            action_tx,
            action_rx,
            focused_area: Some(FocusArea::Search),
            database,
            root_layout: crate::common::layout::RootLayout::default(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        for module in self.search_modules.iter_mut() {
            module.register_database_handler(self.database.clone())?;
            module.register_settings_handler(self.settings.clone())?;

            module.init()?;
        }

        let mut tui = Tui::new()?
            // .mouse(true) // uncomment this line to enable mouse support
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
            component.register_settings_handler(self.settings.clone())?;
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

    fn update_focus(&mut self, focused_area: Option<FocusArea>) -> Result<()> {
        if focused_area.is_none() {
            return Ok(());
        }
        // no change in focus
        if self.focused_area == focused_area {
            return Ok(());
        }
        self.focused_area = focused_area.clone();
        let action_tx = self.action_tx.clone();
        action_tx
            .send(Action::Focus(focused_area.unwrap()))
            .unwrap();
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

        let mut region: Option<FocusArea> = None;

        match mouse.kind {
            MouseEventKind::Down(button) => {
                for component in self.components.iter_mut() {
                    if component.contains(&mouse) {
                        region = Some(component.focus_area());
                        break;
                    }
                }
            }
            MouseEventKind::Moved => {
                for component in self.components.iter_mut() {
                    if component.contains(&mouse) {
                        region = Some(component.focus_area());
                        log::info!("Mouse moved in region: {:?}", region);
                        break;
                    }
                }
            }
            _ => {}
        }
        self.update_focus(region)?;
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let keymap = self.settings.keybinds.clone().get_event_mapping();

        let key_serialised: SerializableKeyEvent = key.into();
        match keymap.get(&key_serialised) {
            Some(action) => {
                info!("Got action: {action:?}");
                action_tx.send(action.clone()).unwrap();
            }
            _ => {
                // // If the key was not handled as a single key action,
                // // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // // Check for multi-key combinations
                // if let Some(action) = m.get(&self.last_tick_key_events[0].into()) {
                //     info!("Got action: {action:?}");
                //     action_tx.send(action.clone()).unwrap();
                // }
                log::info!("No action mapped for key: {:?}", key_serialised);
            } // None => {}
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
                    // sleep::sleep(std::time::Duration::from_millis(100));
                    action_tx.send(Action::Quit).unwrap();
                }

                Action::FocusNext => {
                    let new = self.focused_area.clone().unwrap_or(FocusArea::Search) + 1;
                    self.update_focus(Some(new))?;
                }
                Action::FocusPrevious => {
                    let new = self.focused_area.clone().unwrap_or(FocusArea::Search) - 1;
                    self.update_focus(Some(new))?;
                }
                Action::ToggleWizard => {
                    if self.root_layout.left_right_split > 0 {
                        self.root_layout.set_left_right_split(0);
                    } else {
                        self.root_layout.set_left_right_split(25);
                    }
                    self.root_layout.queue_update();
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
            let has_layout_changed = self
                .root_layout
                .calculate_split(frame.area(), &self.settings);
            if has_layout_changed {
                let action_tx = self.action_tx.clone();
                action_tx
                    .send(Action::UpdateLayout(self.root_layout.clone()))
                    .unwrap();
            }
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
