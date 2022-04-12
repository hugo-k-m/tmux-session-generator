use std::{
    fs::{self, DirEntry},
    path::PathBuf,
    str::Chars,
};

use anyhow::Ok;
use lib::err::CustomResult;

use super::new_session::create_session_script;

fn get_file_without_extension<'a>(file_path: &'a PathBuf) -> Chars<'a> {
    let name_of_file = &file_path.file_name().expect(
        "Could not determine the final \
         component of the path.",
    );

    let mut file_without_extension = name_of_file
        .to_str()
        .expect("Could not convert file name to string.")
        .chars();

    for _ in 0..3 {
        file_without_extension.next_back();
    }

    file_without_extension
}

fn get_underscore_position(file_without_extension: &Chars) -> usize {
    let underscore_position = file_without_extension.clone().count()
        - 1
        - file_without_extension
            .clone()
            .rev()
            .position(|c| c == '_')
            .expect("Could not determine the position of the last underscore.");

    underscore_position
}

fn get_file_name_without_index(file_path: &PathBuf) -> String {
    let file_without_extension = get_file_without_extension(file_path);
    let underscore_position = get_underscore_position(&file_without_extension);

    let chars_before_underscore: Vec<String> = file_without_extension
        .take(underscore_position)
        .map(|x| x.to_string())
        .collect();

    let script_name = chars_before_underscore.concat();

    script_name
}

fn get_script_index_from_name(file_path: PathBuf) -> CustomResult<usize> {
    let file_without_extension = get_file_without_extension(&file_path);
    let underscore_position = get_underscore_position(&file_without_extension);

    let chars_after_underscore: Vec<String> = file_without_extension
        .skip(underscore_position + 1)
        .map(|x| x.to_string())
        .collect();

    let script_index = chars_after_underscore.concat().parse::<usize>()?;

    Ok(script_index)
}

fn determine_new_session_index(
    dir_size: usize,
    session_name: String,
    target_dir_path: PathBuf,
) -> CustomResult<usize> {
    let mut count = 0;

    loop {
        let mut session_dir: Vec<DirEntry> = fs::read_dir(target_dir_path.clone())?
            .map(|r| r.unwrap())
            .collect();

        session_dir.sort_by_key(|dir| dir.path());

        let last_script = &session_dir[dir_size - count - 1];
        let file_path = last_script.path();
        let script_name = get_file_name_without_index(&file_path);

        if script_name == session_name {
            return Ok(get_script_index_from_name(file_path)? + 1);
        }

        count = count + 1;

        if count >= dir_size.to_owned() {
            return Ok(0);
        }
    }
}

/// Create a session script file specified by the target option.
///
/// If a directory corresponding to the target already exists, then the new script will be added to
/// the session group. If no such group exists, then this function will create a new session group
/// named after the specified target.
///
/// Returns the path for the newly created session.
pub(in crate::options) fn handle_new_session_options(
    content: String,
    session_name: &String,
    target_session: &Option<String>,
    tmuxsg_home: PathBuf,
) -> CustomResult<()> {
    if let Some(target) = target_session {
        let target_dir_path = tmuxsg_home.join(target);

        if target_dir_path.is_dir() {
            let dir_size = fs::read_dir(target_dir_path.clone())?.count();
            let group_option_file = target_dir_path.join("__session_group_option.sh");
            let group_option_file_data = fs::read_to_string(&group_option_file)?;

            if group_option_file_data != "is_session_group" {
                let new_group_session_name = session_name.to_owned() + "_0";

                fs::write(group_option_file, "is_session_group")?;

                create_session_script(
                    content,
                    &new_group_session_name,
                    Some(target.to_owned()),
                    tmuxsg_home,
                )?;

                return Ok(());
            } else {
                let new_session_index = determine_new_session_index(
                    dir_size,
                    session_name.clone(),
                    target_dir_path.clone(),
                )?;

                let new_group_session_name = &format!("{}_{}", session_name, new_session_index);

                create_session_script(
                    content,
                    new_group_session_name,
                    Some(target.to_owned()),
                    tmuxsg_home,
                )?;

                return Ok(());
            }
        }
    }

    let new_group_session_name = session_name.to_owned() + "_0";

    create_session_script(content, &new_group_session_name, None, tmuxsg_home)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use lib::{
        err::CustomResult,
        mocks::{TestObject, TestSessionDirGroupMultipleScripts},
    };

    use crate::options::parser_helpers::{get_file_name_without_index, get_underscore_position};

    use super::{
        determine_new_session_index, get_file_without_extension, get_script_index_from_name,
        handle_new_session_options,
    };

    #[test]
    fn determine_new_session_index_test() -> CustomResult<()> {
        let tsg_test = TestSessionDirGroupMultipleScripts::setup(None)?;
        let target_dir_path = tsg_test.test_session_path;
        let dir_size = fs::read_dir(&target_dir_path)?.count().to_owned();

        assert_eq!(
            0,
            determine_new_session_index(
                dir_size,
                "fourth_test_session".to_owned(),
                target_dir_path.clone()
            )?
        );

        assert_eq!(
            10,
            determine_new_session_index(
                dir_size,
                "second_test_session".to_owned(),
                target_dir_path.clone()
            )?
        );

        Ok(())
    }

    #[test]
    fn file_without_extension_test() {
        let session_path = PathBuf::from("/test/path/test_session_0.sh");
        let expected_file_name = "test_session_0".chars();
        let file_without_extension = get_file_without_extension(&session_path);
        let expected_result = expected_file_name.eq(file_without_extension);

        assert_eq!(expected_result, true);
    }

    #[test]
    fn file_without_index_test() {
        let session_path = PathBuf::from("/test/path/test_session_0.sh");
        let expected_result = "test_session";
        let file_without_index = get_file_name_without_index(&session_path);

        assert_eq!(expected_result, file_without_index);
    }

    #[test]
    fn get_script_index_from_name_test() -> CustomResult<()> {
        let session_path = PathBuf::from("/test/path/test_session_15.sh");
        let expected_result = 15;
        let script_index = get_script_index_from_name(session_path)?;

        assert_eq!(expected_result, script_index);

        Ok(())
    }

    #[test]
    fn handle_new_session_options_session_group_case_test() -> CustomResult<()> {
        let tsg_test = TestSessionDirGroupMultipleScripts::setup(Some(true))?;
        let tmuxsg_home = tsg_test.test_tmuxsg_path;
        let session_dir = tsg_test.test_session_path;
        let content = "Dummy content".to_owned();
        let target_session = Some("test_session".to_owned());

        handle_new_session_options(
            content.clone(),
            &"fourth_test_session".to_owned(),
            &target_session,
            tmuxsg_home.clone(),
        )?;

        handle_new_session_options(
            content.clone(),
            &"first_test_session".to_owned(),
            &target_session,
            tmuxsg_home.clone(),
        )?;

        assert!(session_dir.join("fourth_test_session_0.sh").is_file());
        assert!(session_dir.join("first_test_session_3.sh").is_file());

        Ok(())
    }

    #[test]
    fn underscore_position_test() {
        let file_without_extension = "test_session_0".chars();
        let expected_result = 12;
        let underscore_position = get_underscore_position(&file_without_extension);

        assert_eq!(expected_result, underscore_position);
    }
}
