use super::*;
use crate::options::Opts;
use lib::{
    dir::create_dir,
    err::ScriptError,
    mocks::{TestObject, TestSessionDir, TestSessionDirGroupScript, TestTmuxHomeDir},
    produce_script_error,
};
use std::fs;

/// Test session script creation process.
#[test]
fn create_session_script_success() -> CustomResult<()> {
    const SESSION_NAME: &str = "new_session";
    let content = "test content".to_owned();
    let group_option = false;

    let tsg_test = TestTmuxHomeDir::setup(None)?;
    let tsg_home_dir = tsg_test.test_tmuxsg_path;
    let session_dir = PathBuf::from(&format!("{}/{}", tsg_home_dir.display(), SESSION_NAME));

    let script_path_expected =
        PathBuf::from(&format!("{}/{}.sh", session_dir.display(), SESSION_NAME));

    create_session_script(content, SESSION_NAME, tsg_home_dir, group_option)?;

    assert!(script_path_expected.is_file());

    Ok(())
}

#[test]
fn session_script_unaffected_if_already_exists() -> CustomResult<()> {
    let session_name = "test_session_0";

    let tsg_test = TestSessionDir::setup(None)?;
    let session_dir = tsg_test.test_session_path;
    let expected_session_script = session_dir.join("test_session_0.sh");

    assert!(expected_session_script.is_file());

    create_script(session_dir, session_name)?;

    assert!(expected_session_script.is_file());

    Ok(())
}

#[test]
fn create_session_directory_success() -> CustomResult<()> {
    let session_name = "new_session".to_owned();

    let tsg_test = TestTmuxHomeDir::setup(None)?;
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

    let test_content = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "resources/test/script_content_checks/\
            session/attach_test_session.sh",
    );

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

    let test_content = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "resources/test/script_content_checks/\
            session/detach_test_session.sh",
    );

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
        "resources/test/script_content_checks/\
            session/name_window_option_test_session.sh",
    );

    let expected_test_session_content = fs::read_to_string(test_content)?;

    assert_eq!(detach_test_session_content, expected_test_session_content);

    Ok(())
}

#[test]
fn session_group_option_script_creation_success() -> CustomResult<()> {
    let group_option = false;
    let tsg_test = TestSessionDir::setup(None)?;
    let session_dir = tsg_test.test_session_path;
    let expected_group_script = session_dir.join("__session_group_option.sh");

    set_session_group_option(&session_dir, group_option)?;

    assert!(expected_group_script.is_file());

    Ok(())
}

#[test]
fn group_script_creation_doesnt_affect_existing_script() -> CustomResult<()> {
    let group_option = true;
    let tsg_test = TestSessionDirGroupScript::setup(Some(group_option)).unwrap();
    let session_dir = tsg_test.test_session_path;
    let expected_group_script = session_dir.join("__session_group_option.sh");

    assert!(expected_group_script.is_file());

    set_session_group_option(&session_dir, group_option)?;
    let data = fs::read_to_string(expected_group_script)?;

    assert_eq!(data, "is_session_group");

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
