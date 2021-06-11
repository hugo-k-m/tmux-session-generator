//! NewSession subcommand helpers

use lib::options::create_script;
use lib::tmux_option;
use std::io::Write;
use std::{fs, path::PathBuf};

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
    tmux_option!(
        name_w, n
        target_s, t
        width, x
        height, y
    );

    const SESSION_VAR: &str = "session";
    const PATH_VAR: &str = "session_path";

    let mut content = "#!/bin/sh\n\n".to_owned();

    content.push_str(&format!("{}=\"{}\"\n", SESSION_VAR, session_name));
    content.push_str(&format!("{}={}\n", PATH_VAR, command));

    content.push_str(&format!(
        "tmux new-session -d -s ${} -c ${} {} {} {} {}\n",
        SESSION_VAR, PATH_VAR, name_w, target_s, width, height
    ));

    if detach.to_owned() {
        return content;
    } else {
        content.push_str("\n# Attach\n");
        content.push_str(&format!("tmux attach -t ${}", SESSION_VAR));
    };

    content
}

// TODO: Handle error with custom error
/// Creates the session directory and returns its path.
fn session_dir(tsg_home: PathBuf, s_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let s_dir = tsg_home.join(s_name);
    fs::create_dir(&s_dir)?;

    Ok(s_dir)
}

// TODO: test script and content creation
#[cfg(test)]
mod tests {
    use super::*;
    use crate::home_dirs::tmuxsg_home_dir;
    use lib::test::CreationTest;

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
}
