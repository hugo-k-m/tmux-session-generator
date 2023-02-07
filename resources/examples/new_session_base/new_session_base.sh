#!/bin/sh

session="new_session"
session_path=~
tmux new-session -d -s $session -c $session_path

# Attach
tmux attach -t $session
