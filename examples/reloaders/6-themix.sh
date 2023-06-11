#!/bin/sh

THEMIX_PATH="/opt/oomox"

if [ ! -d "${THEMIX_PATH}" ]; then
    exit 0
fi

THEMIX_PLUGINS_PATH="${THEMIX_PATH}/plugins"

THEMIX_THEME_PATH="${THEMIX_PLUGINS_PATH}/theme_oomox"
THEMIX_ICONS_PATH="${THEMIX_PLUGINS_PATH}/icons_papirus"

TEMPLATE_PATH="${HOME}/.cache/oxidec/templates/colors-themix"

${THEMIX_THEME_PATH}/change_color.sh -o oxidec ${TEMPLATE_PATH} &>/dev/null &
${THEMIX_ICONS_PATH}/change_color.sh -o oxidec ${TEMPLATE_PATH} &>/dev/null &
