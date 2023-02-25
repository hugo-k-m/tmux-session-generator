//! NewWindow subcommand helpers.

use lib::{
    err::{CustomResult, DirectoryError, ScriptError},
    options::create_script,
    produce_directory_error, produce_script_error, tmux_bool_option, tmux_option,
};
use std::{io::Write, path::PathBuf};

pub(in crate::options) fn create_window_script(
    content: (String, String, String),
    tmuxsg_home: PathBuf,
) -> CustomResult<()> {
    let script_content = content.0;
    let session_name = content.1;
    let window_name = content.2;

    // test this; consider using bytes() instead of collect()
    let isolated_session_name = if session_name.contains(":") {
        session_name.split(":").collect::<Vec<&str>>()[0].to_owned()
    } else {
        session_name
    };

    let session_dir = tmuxsg_home.join(&isolated_session_name);

    if !session_dir.is_dir() {
        produce_directory_error!(format!("{}", isolated_session_name));
    } else {
        let mut file = create_script(session_dir, window_name)?;
        file.write_all(script_content.as_bytes())?;
    }

    Ok(())
}

pub(in crate::options) fn create_window_script_content(
    a: &bool,
    k: &bool,
    command: &String,
    d: &bool,
    n: &Option<String>,
    t: &Option<String>,
) -> CustomResult<(String, String, String)> {
    let error = "Window content related".to_owned();

    let session_name = if let Some(target) = t {
        target
    } else {
        produce_script_error!(error);
    };

    let window_name = if let Some(name) = n {
        name
    } else {
        "new_window"
    };

    let mut content = "#!/bin/sh\n\n".to_owned();
    content.push_str(&format!("tmux new-window -c {}", command));

    tmux_option!(
        t n,
        content
    );

    tmux_bool_option!(
        a, content
        d, content
        k, content
    );

    // This is to account for editors such as vim that add a newline at the end
    // of the file.
    content.push_str(&format!("\n"));

    Ok((content, session_name.to_owned(), window_name.to_owned()))
}

// TODO: write tests
#[cfg(test)]
mod tests {
    use lib::{
        err::CustomResult,
        mocks::{TestObject, TestSessionDir, TestTmuxHomeDir},
    };
    use std::{fs, path::PathBuf};

    use crate::options::{
        new_window::create_window_script, test_utils::test_utils::create_window_test_content,
    };

    #[test]
    fn create_window_script_success() -> CustomResult<()> {
        const WINDOW_NAME: &str = "test_window";

        let content = (
            "test content".to_owned(),
            "test_session".to_owned(),
            WINDOW_NAME.to_owned(),
        );

        let tsg_test = TestSessionDir::setup(None)?;
        let tmuxsg_home = tsg_test.test_tmuxsg_path;
        let session_dir = tsg_test.test_session_path;

        let script_path_expected =
            PathBuf::from(&format!("{}/{}.sh", session_dir.display(), WINDOW_NAME));

        create_window_script(content, tmuxsg_home)?;

        assert!(script_path_expected.is_file());

        Ok(())
    }

    #[test]
    fn session_directory_doesnt_exist() -> CustomResult<()> {
        const WINDOW_NAME: &str = "test_window";

        let content = (
            "test content".to_owned(),
            "test_session".to_owned(),
            WINDOW_NAME.to_owned(),
        );

        let tsg_test = TestTmuxHomeDir::setup(None)?;
        let tmuxsg_home = tsg_test.test_tmuxsg_path;
        let test_result = create_window_script(content, tmuxsg_home).is_err();

        assert_eq!(test_result, true);

        Ok(())
    }

    #[test]
    fn create_window_script_content_test() -> CustomResult<()> {
        let test_window_content = create_window_test_content(
            false,
            false,
            "~".to_owned(),
            false,
            Some("new_window".to_owned()),
            Some("new_session:1".to_owned()),
        )?;

        let ex_test_content = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/examples/new_window/new_window.sh");

        let expected_test_session_content = fs::read_to_string(ex_test_content)?;

        assert_eq!(test_window_content.0, expected_test_session_content);

        Ok(())
    }
}
