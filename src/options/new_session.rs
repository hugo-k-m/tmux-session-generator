//! NewSession subcommand helpers

use anyhow::Context;
use lib::{dir::create_dir, err::CustomResult, options::create_script, tmux_option};
use std::{io::Write, path::PathBuf};

pub(in crate::options) fn create_session_script(
    content: String,
    s_name: &str,
    tmuxsg_home: PathBuf,
) -> CustomResult<()> {
    let s_dir = create_dir(tmuxsg_home, s_name.to_owned())?;
    let mut file =
        create_script(s_dir, s_name).with_context(|| format!("could not create session script"))?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub(in crate::options) fn session_script_content(
    command: &String,
    detach: &bool,
    n: &Option<String>,
    session_name: &String,
    t: &Option<String>,
    x: &Option<usize>,
    y: &Option<usize>,
) -> String {
    const SESSION_VAR: &str = "session";
    const PATH_VAR: &str = "session_path";

    let mut content = "#!/bin/sh\n\n".to_owned();

    content.push_str(&format!("{}=\"{}\"\n", SESSION_VAR, session_name));
    content.push_str(&format!("{}={}\n", PATH_VAR, command));

    content.push_str(&format!(
        "tmux new-session -d -s ${} -c ${}",
        SESSION_VAR, PATH_VAR
    ));

    tmux_option!(
        n t x y,
        content
    );

    if detach.to_owned() {
        return content;
    } else {
        content.push_str("\n\n# Attach\n");
        content.push_str(&format!("tmux attach -t ${}", SESSION_VAR));
    };

    content
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::Opts;
    use lib::{
        dir::create_dir,
        err::ScriptError,
        produce_script_error,
        test::{SessionTestObject, TestObject, WindowTestObject},
    };
    use std::fs;

    /// Test session script creation process.
    #[test]
    fn create_session_script_success() -> CustomResult<()> {
        const SESSION_NAME: &str = "new_session";
        let content = "test content".to_owned();

        let tsg_test = SessionTestObject::setup()?;
        let tsg_home_dir = tsg_test.test_tmuxsg_path;
        let session_dir = PathBuf::from(&format!("{}/{}", tsg_home_dir.display(), SESSION_NAME));

        let script_path_expected =
            PathBuf::from(&format!("{}/{}.sh", session_dir.display(), SESSION_NAME));

        create_session_script(content, SESSION_NAME, tsg_home_dir)?;

        assert!(script_path_expected.is_file());

        Ok(())
    }

    #[test]
    fn session_script_already_exists() -> CustomResult<()> {
        let session_name = "test_session";

        let tsg_test = WindowTestObject::setup()?;
        let session_dir = tsg_test.test_session_path;

        assert!(create_script(session_dir, session_name).is_err());

        Ok(())
    }

    #[test]
    fn create_session_directory_success() -> CustomResult<()> {
        let session_name = "new_session".to_owned();

        let tsg_test = SessionTestObject::setup()?;
        let tsg_home_dir_path = tsg_test.test_tmuxsg_path;

        let s_dir_expected = PathBuf::from(&format!(
            "{}/{}",
            &tsg_home_dir_path.display(),
            session_name
        ));

        create_dir(tsg_home_dir_path, session_name)?;

        assert!(s_dir_expected.is_dir());

        Ok(())
    }

    #[test]
    fn session_script_content_attach_test() -> CustomResult<()> {
        let attach_test_session_content = test_session_content(
            "~".to_owned(),
            false,
            None,
            "attach_test_session".to_owned(),
            None,
            None,
            None,
        )?;

        let test_content = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/test/script_content_checks/session/attach_test_session.sh");

        let expected_test_session_content = fs::read_to_string(test_content)?;

        assert_eq!(attach_test_session_content, expected_test_session_content);

        Ok(())
    }

    #[test]
    fn session_script_content_detach_test() -> CustomResult<()> {
        let detach_test_session_content = test_session_content(
            "~".to_owned(),
            true,
            None,
            "detach_test_session".to_owned(),
            None,
            None,
            None,
        )?;

        let test_content = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/test/script_content_checks/session/detach_test_session.sh");

        let expected_test_session_content = fs::read_to_string(test_content)?;

        assert_eq!(detach_test_session_content, expected_test_session_content);

        Ok(())
    }

    #[test]
    fn session_script_content_window_name_test() -> CustomResult<()> {
        let detach_test_session_content = test_session_content(
            "~".to_owned(),
            true,
            Some("window_name".to_owned()),
            "window_name_test_session".to_owned(),
            None,
            None,
            None,
        )?;

        let test_content = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "resources/test/script_content_checks/session/name_window_option_test_session.sh",
        );

        let expected_test_session_content = fs::read_to_string(test_content)?;

        assert_eq!(detach_test_session_content, expected_test_session_content);

        Ok(())
    }

    /// Test helper function; returns test session script content
    fn test_session_content(
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
