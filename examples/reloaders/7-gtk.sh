#!/bin/sh

if which xsettingsd &>/dev/null; then
    (echo "Net/ThemeName \"${OXIDEC_COLORSCHEME}\"" > /tmp/xsettingsd.conf
    echo "Net/IconThemeName \"${OXIDEC_COLORSCHEME}\"" >> /tmp/xsettingsd.conf

    timeout 0.2 xsettingsd -c /tmp/xsettingsd.conf &>/dev/null
    rm -f /tmp/xsettingsd.conf) &
fi

if which gsettings &>/dev/null; then
    (gsettings set org.gnome.desktop.interface gtk-theme ""
    gsettings set org.gnome.desktop.interface icon-theme ""

    sleep 0.1

    gsettings set org.gnome.desktop.interface gtk-theme "${OXIDEC_COLORSCHEME}"
    gsettings set org.gnome.desktop.interface icon-theme "${OXIDEC_COLORSCHEME}") &
fi

if which xfconf-query &>/dev/null; then
    (xfconf-query -c xsettings -p /Net/ThemeName -s ""
    xfconf-query -c xsettings -p /Net/IconThemeName -s ""

    sleep 0.1

    xfconf-query -c xsettings -p /Net/ThemeName -s "${OXIDEC_COLORSCHEME}"
    xfconf-query -c xsettings -p /Net/IconThemeName -s "${OXIDEC_COLORSCHEME}") &
fi
