#!/bin/sh

if pidof -x qtile >/dev/null; then
    pkill -SIGUSR1 qtile
fi
