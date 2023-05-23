#!/bin/sh

function merge_configs {
    python3 - <<END
from os import environ
from os.path import exists

from yaml import safe_dump, safe_load

home = environ["HOME"]
template = f"{home}/.cache/oxidec/templates/colors-qutebrowser.yml"
quteconfig = f"{home}/.config/qutebrowser/autoconfig.yml"

if exists(template):
    colors = safe_load(open(template))

    if exists(quteconfig):
        autoconfig = safe_load(open(quteconfig))
    else:
        autoconfig = {}

    for color in colors:
        autoconfig["settings"][color] = {"global": colors[color]}

    safe_dump(autoconfig, open(quteconfig, "w"))
END
}

if which qutebrowser &>/dev/null; then
    merge_configs &
fi

if pidof -x qutebrowser >/dev/null; then
    (qutebrowser :config-source >&/dev/null &)
fi
