//! Utility functions for unit tests.

#[cfg(test)]
pub(in crate::options) mod test_utils {
    use lib::{
        err::{CustomResult, ScriptError},
        produce_script_error,
    };

    use crate::options::{
        new_session::create_session_script_content, new_window::create_window_script_content, Opts,
    };

    /// Test helper function; returns test session script content.
    ///
    /// This function tests the `create_session_script_content` function.
    pub fn create_session_test_content(
        command: String,
        detach: bool,
        name_window: Option<String>,
        session_name: String,
        target_session: Option<String>,
        x: Option<usize>,
        y: Option<usize>,
    ) -> CustomResult<String> {
        let error = "Session content related".to_owned();

        let detach_test_session = Opts::NewSession {
            command,
            detach,
            name_window,
            session_name,
            target_session,
            x,
            y,
        };

        let test_session_content = if let Opts::NewSession {
            command,
            detach,
            name_window,
            session_name,
            target_session,
            x,
            y,
        } = detach_test_session
        {
            create_session_script_content(
                &command,
                &detach,
                &name_window,
                &session_name,
                &target_session,
                &x,
                &y,
            )
        } else {
            produce_script_error!(error);
        };

        Ok(test_session_content)
    }

    /// Test helper function; returns test window script content
    ///
    /// This function tests the `create_window_script_content` function.
    pub fn create_window_test_content(
        a: bool,
        kill: bool,
        command: String,
        detach: bool,
        name_window: Option<String>,
        target_window: Option<String>,
    ) -> CustomResult<(String, String, String)> {
        let error = "Window content related".to_owned();

        let test_window = Opts::NewWindow {
            a,
            kill,
            command,
            detach,
            name_window,
            target_window,
        };

        let test_window_content = if let Opts::NewWindow {
            a,
            kill,
            command,
            detach,
            name_window,
            target_window,
        } = test_window
        {
            create_window_script_content(&a, &kill, &command, &detach, &name_window, &target_window)
        } else {
            produce_script_error!(error);
        }?;

        Ok(test_window_content)
    }
}
