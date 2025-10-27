use crate::common::{
    app_state::AppState,
    module_state::{ModuleState, UIResult, UIState, UIStateUpdate},
};

/// A trait that defines the core functionality for application modules.
///
/// A module is a small "machine" that operates on its own internal data to process
/// queries and generate UI content. Each module maintains its own state and data,
/// and can determine its relevance to search queries.
///
/// # Type Parameters
///
/// * `Data` - The type representing the module's internal data
/// * `State` - The type representing the module's internal state

pub trait ModuleData {}

pub trait Module {
    type State;

    /// Processes a search query and determines module candidacy.
    ///
    /// This method evaluates whether the module is relevant to the given search query.
    /// For example, a math module would return true for queries like "1+1" or "calculate",
    /// while a file search module might return true for filesystem-related queries.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string to evaluate
    /// * `app_state` - Reference to the application's global state
    ///
    /// # Returns
    ///
    /// * `bool` - True if this module is a candidate for handling the query, false otherwise
    fn on_search(&mut self, query: &str, app_state: &AppState) -> bool;

    /// Executes the module's primary action.
    ///
    /// This method performs the module's main functionality, typically triggered
    /// when the user confirms or executes a search/action.
    ///
    /// # Arguments
    ///
    /// * `app_state` - Mutable reference to the application's global state
    ///
    /// # Returns
    ///
    /// * `bool` - True if the execution was successful, false otherwise

    /// Renders the module's UI by processing its internal data into display elements.
    ///
    /// This method acts upon the module's stored data to produce a UIStateUpdate
    /// containing results, query information, and other display elements. The module
    /// transforms its internal data state into UI components that can be displayed
    /// in the terminal interface.
    ///
    /// # Returns
    ///
    /// * `UIStateUpdate` - Contains the rendered results, query display, and other
    ///   UI elements derived from the module's current data state
    fn get_results(&mut self) -> Box<Vec<UIResult>>;

    /// Retrieves a mutable reference to the module's state.
    ///
    /// # Returns
    ///
    /// * `&mut ModuleState` - Mutable reference to the module's internal state
    fn get_state(&mut self) -> &mut ModuleState;
}
