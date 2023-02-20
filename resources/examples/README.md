# Examples

This directory provides examples on how to the `tmuxsg` works. This main
`example_commands.md` file contains example prompts one can use with `tmuxsg`.
The outputs of these commands are provided in the accompanying subfolders in
this directory. The heading of the command and the respective output folder
share the same name. For instance, the `new_session_base`example's output is
located in the `new_session_base` subdirectory.

## `new_session_base`

Command:

```
tmuxsg new-session -d -s new_session -c ~

```

## `new_window_for_target_session`

Command:

```
tmuxsg new-window -t new_session:1 -n new_window -c ~

```
