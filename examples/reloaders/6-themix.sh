#!/bin/sh

THEMIX_PATH="/opt/oomox"
GTK_THEMES_PATH="${HOME}/.themes"
GTK_ICONS_PATH="${HOME}/.icons"

if [ ! -d "${THEMIX_PATH}" ]; then
    exit 0
fi

if [ -z "${OXIDEC_GTK}" ]; then
    exit 0
fi

THEMIX_PLUGINS_PATH="${THEMIX_PATH}/plugins"

THEMIX_THEME_PATH="${THEMIX_PLUGINS_PATH}/theme_oomox"
THEMIX_ICONS_PATH="${THEMIX_PLUGINS_PATH}/icons_papirus"

OXIDEC_CACHE_PATH="${HOME}/.cache/oxidec"
TEMPLATE_PATH="${OXIDEC_CACHE_PATH}/templates/colors-themix"

if [ ! -d "${GTK_THEMES_PATH}/${OXIDEC_COLORSCHEME}" ]; then
    echo "! Generating GTK theme. This may take a while."
    ${THEMIX_THEME_PATH}/change_color.sh -o ${OXIDEC_COLORSCHEME} ${TEMPLATE_PATH} &>/dev/null
fi

if [ ! -d "${GTK_ICONS_PATH}/${OXIDEC_COLORSCHEME}" ]; then
    echo "! Generating GTK icons. This may take a while."
    ${THEMIX_ICONS_PATH}/change_color.sh -o ${OXIDEC_COLORSCHEME} ${TEMPLATE_PATH} &>/dev/null
fi
