//! NewSession subcommand helpers

use lib::{options::create_script, tmux_option};
use std::{fs, io::Write, path::PathBuf};

pub(in crate::options) fn create_session_script(
    content: String,
    s_name: &str,
    tmuxsg_home: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let s_dir = session_dir(tmuxsg_home, s_name)?;
    let mut file = create_script(s_dir, s_name)?;
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

/// Creates the session directory and returns its path.
fn session_dir(tsg_home: PathBuf, s_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let s_dir = tsg_home.join(s_name);
    fs::create_dir(&s_dir)?;

    Ok(s_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{home_dirs::tmuxsg_home_dir, options::Opts};
    use lib::{err::ScriptError, test::CreationTest};

    /// Test script creation process
    #[test]
    fn create_session_script_success() -> Result<(), Box<dyn std::error::Error>> {
        let tsg_test = CreationTest::setup()?;
        let home_d = tsg_test.path;
        let tsg_home = tmuxsg_home_dir(home_d)?;

        const S_NAME: &str = "new_session";
        let s_dir = session_dir(tsg_home, S_NAME)?;
        let script_path_expected = PathBuf::from(&format!("{}/{}.sh", &s_dir.display(), S_NAME));
        create_script(s_dir, S_NAME)?;

        assert!(script_path_expected.is_file());

        Ok(())
    }

    #[test]
    fn create_session_directory_success() -> Result<(), Box<dyn std::error::Error>> {
        let tsg_test = CreationTest::setup()?;
        let home_d = tsg_test.path;
        let tsg_home = tmuxsg_home_dir(home_d)?;

        const S_NAME: &str = "new_session";
        let s_dir_expected = PathBuf::from(&format!("{}/{}", &tsg_home.display(), S_NAME));
        session_dir(tsg_home, "new_session")?;

        assert!(s_dir_expected.is_dir());

        Ok(())
    }

    #[test]
    fn check_session_script_content_detach() -> Result<(), Box<dyn std::error::Error>> {
        let error = "Window content related".to_owned();

        let test_command = "~".to_owned();
        let test_session_name = "detach_test_session".to_owned();

        let detach_test_session = Opts::NewSession {
            command: test_command,
            detach: true,
            name_window: None,
            session_name: test_session_name,
            target_session: None,
            x: None,
            y: None,
        };

        let detach_test_session_content = if let Opts::NewSession {
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
            let content_err = ScriptError(error);

            return Err(Box::new(content_err));
        };

        let test_content = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/test/script_content_checks/session/detach_test_session.sh");

        let expected_test_session_content = fs::read_to_string(test_content)?;

        assert_eq!(detach_test_session_content, expected_test_session_content);

        Ok(())
    }

    #[test]
    fn check_session_script_content_attach() -> Result<(), Box<dyn std::error::Error>> {
        let error = "Window content related".to_owned();

        let test_command = "~".to_owned();
        let test_session_name = "attach_test_session".to_owned();

        let attach_test_session = Opts::NewSession {
            command: test_command,
            detach: false,
            name_window: None,
            session_name: test_session_name,
            target_session: None,
            x: None,
            y: None,
        };

        let detach_test_session_content = if let Opts::NewSession {
            command,
            detach,
            name_window,
            session_name,
            target_session,
            x,
            y,
        } = attach_test_session
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
            let content_err = ScriptError(error);

            return Err(Box::new(content_err));
        };

        let test_content = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/test/script_content_checks/session/attach_test_session.sh");

        let expected_test_session_content = fs::read_to_string(test_content)?;

        assert_eq!(detach_test_session_content, expected_test_session_content);

        Ok(())
    }
}
