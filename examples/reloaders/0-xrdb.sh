#!/bin/sh

if which xrdb &>/dev/null; then
    xrdb -merge -quiet ~/.Xresources
    xrdb -merge -quiet ~/.cache/oxidec/templates/colors.Xresources
fi
