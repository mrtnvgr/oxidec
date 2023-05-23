#!/bin/sh

termux_path=~/.termux/

if [ -d "$termux_path" ]; then
    cp ~/.cache/oxidec/templates/colors.termux $termux_path/colors.properties
    termux-reload-settings
fi
