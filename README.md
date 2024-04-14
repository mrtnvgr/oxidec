<h3 align="center">
    <img
        src="https://github.com/mrtnvgr/oxidec/assets/48406064/8ce1565d-f65a-4664-92fe-995aacb74d7c"
        width="400px"
        height="400px"
    />
</h3>
<h3 align="center" style="color: #ed8796;"><b>OUTDATED</b></h3>
<h3 align="center">oxidec</h3>
<!-- <p align="center">Oxidized delicious eye candies 🍬</p> -->
<p align="center"><b>Manage your desktop appearance with ease!</b></h3>

## Features

- **Generate** files with colors from **templates**!
- **Update** the colors with **reloaders**!
- **Generate** colorscheme from **wallpaper**!
- **Save** the current look into a **theme**!
- **Import** your **pywal colorschemes**!
- **Avoid** theme breakage by **dependency checking**!
- **Apply GTK themes** with [Themix](https://github.com/themix-project/themix-gui)![^1]

## Install

```sh
cargo install oxidec
```

If you want to install new files from examples directory on every update, enable `new-examples` feature:

```sh
cargo install oxidec --features new-examples
```

### Recommended aliases

```sh
alias cs="oxidec colorscheme"
alias wl="oxidec wallpaper"
alias wp="oxidec wallpaper"
alias th="oxidec theme"
```

## FAQ

### GTK live reloading doesn't work for me.

Try [xsettingsd](https://codeberg.org/derat/xsettingsd)

[^1]: GTK Theme generation is optional. Use `--gtk` flag or set `OXIDEC_GTK` env variable.
