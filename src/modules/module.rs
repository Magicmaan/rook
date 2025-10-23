use crate::model::{
    app_state::Model,
    module_state::{ModuleState, UIState},
};

/// A trait that defines the core functionality for application modules.
///
/// Modules are components that can handle events, maintain state, and render UI elements.
/// Each module can respond to navigation and search events, and is responsible for
/// rendering itself within the provided layout chunks.
///
/// # Type Parameters
///
/// * `State` - The type representing the internal state of the module
pub trait Module {
    /// The type representing the internal state of this module

    /// Updates the module based on incoming events and application state.
    ///
    /// This method processes a vector of events, taken in from the apps update() loop,
    /// handler methods based on the event type.
    ///
    /// # Arguments
    ///
    /// * `events` - A vector of events to process
    /// * `app_state` - Mutable reference to the application's global state model

    /// Handles navigation-related events for the module.
    ///
    /// This method is called when a navigation event is received and should
    /// update the module's internal state accordingly.
    ///
    /// # Arguments
    ///
    /// * `event` - The navigation event to process
    // fn update_navigation(&mut self, event: &Event);

    /// Handles search-related events for the module.
    ///
    /// This method is called when a search event is received and should
    /// update the module's search functionality and internal state.
    ///
    /// # Arguments
    ///
    /// * `event` - The search event to process
    ///   - `Add(char)` - Add a character to the search query
    ///   - `Remove(isize)` - Remove characters from the search query
    ///   - `Execute` - Execute the search with the current query
    ///   - `Clear` - Clear the current search query

    fn on_search(&mut self, query: &str, app_state: &Model) -> bool;

    fn on_execute(&mut self, app_state: &Model) -> bool;
    /// Renders the module's UI elements to the terminal frame.
    ///
    /// This method is responsible for drawing the module's interface within
    /// the provided layout chunks using the ratatui framework.
    ///
    /// the UI should be drawn from structs own settings and state.
    ///
    /// # Arguments
    ///
    /// * `frame` - Mutable reference to the ratatui frame for rendering
    /// * `chunks` - Reference-counted slice of layout rectangles defining the UI sections:
    ///   - `chunks[0]` - Search box area for user input
    ///   - `chunks[1]` - gap between search box and results (if any) (don't render anything here)
    ///   - `chunks[2]` - Results area for displaying search results or content
    ///   - `chunks[3]` - gap between results and bottom (if any) (don't render anything here)
    ///   - `chunks[4]` - Bottom area for status bars or additional info
    ///
    fn render(&mut self) -> &mut UIState;

    fn get_state(&mut self) -> &mut ModuleState;
}

// pub trait Update {
//     // update will handle events and update the modules state accordingly.
//     // possibility to add custom state for each module in the future?
//     // although I don't know how this would work yet.
//     fn update(&mut self, events: &Vec<Event>);
// }

// // idea:
// // A Module trait that defines update, render.
// // their will be module state, and UI state.
// // during update, module state is updated based on events. this could be stuff like search queries , raw data, objects etc.
// // this will then update UIState, which is specifically for rendering.
// // during render, the UIState is used to render the widgets.

// pub trait UI {
//     // UI state will hold data specifically meant to be rendered
//     // i.e. results, list state, selected item etc.
//     type UIState;

//     fn search_box(&self) -> &dyn StatefulWidget<State = Self::UIState>;
//     fn results_box(&self) -> &dyn StatefulWidget<State = Self::UIState>;
// }
