use std::fs;
use std::io::Write;

use super::Opts;

impl Opts {
    pub(in crate::options) fn session_script(
        content: String,
        s_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir(&format!("/tmp/{}", s_name))?;

        let mut file = fs::File::create(&format!("/tmp/{}/{}.sh", s_name, s_name))?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

impl Opts {
    pub(in crate::options) fn session_script_content(
        command: &String,
        detach: &bool,
        name_window: &Option<String>,
        session_name: &String,
        target_session: &Option<String>,
        x: &Option<usize>,
        y: &Option<usize>,
    ) -> String {
        let name_w = if let Some(nw) = name_window {
            format!("-n ${}", nw)
        } else {
            "".to_string()
        };

        let target_s = if let Some(ts) = target_session {
            format!("-t ${}", ts)
        } else {
            "".to_string()
        };

        let width = if let Some(w) = x {
            format!("-x ${}", w)
        } else {
            "".to_string()
        };

        let height = if let Some(h) = y {
            format!("-y ${}", h)
        } else {
            "".to_string()
        };

        const SESSION_VAR: &str = "session";
        const PATH_VAR: &str = "session_path";

        let mut content = "#!/bin/sh\n\n".to_owned();

        content.push_str(&format!("{}={:?}\n", SESSION_VAR, session_name));
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
}
