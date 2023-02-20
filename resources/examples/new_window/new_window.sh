#!/bin/sh

session="new_session"
session_path=~
tmux new-window -t $session:1 -n new_window -c $session_path
