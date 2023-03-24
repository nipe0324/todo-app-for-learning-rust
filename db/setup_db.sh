#!/usr/bin/env bash
cd $(dirname "$0")
sqlite3 ../todo.db < create_todos.sql
