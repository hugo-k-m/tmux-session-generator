//! Utility functions for unit tests.

#[cfg(test)]
pub(in crate::options) mod test_utils {
    use lib::{
        err::{CustomResult, ScriptError},
        produce_script_error,
    };

    use crate::options::{new_session::session_script_content, Opts};

    /// Test helper function; returns test session script content
    pub fn test_session_content(
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
            session_script_content(
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
}
