#!/bin/sh

session="detach_test_session"
session_path=~
tmux new-session -d -s $session -c $session_path