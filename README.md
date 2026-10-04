<h3 align="center">
    <video
        src="https://github.com/user-attachments/assets/c9b4a0f9-0216-4fa4-8e88-fe9376beb771"
        width="400px"
        autoplay
        loop
        muted
        playsinline
    ></video>
</h3>
<h3 align="center">oxidec</h3>
<p align="center"><b>Manage your desktop appearance with ease!</b></h3>

## Features

- **Generate** files with colors from **templates**!
- **Update** the colors with **reloaders**!
- **Generate** colorscheme from **wallpaper**!
- **Save** the current look into a **theme**!
- **Avoid** theme breakage by **dependency checking**!
- **Apply GTK themes** with [Themix](https://github.com/themix-project/themix-gui)![^1]

## Install

```sh
cargo install oxidec
```

### Recommended aliases

```sh
alias cs="oxidec colorscheme"
alias wl="oxidec wallpaper"
alias wp="oxidec wallpaper"
alias th="oxidec theme"
```

## Quirks

- Adds `name` variable if colorscheme doesn't contain it

## FAQ

### GTK live reloading doesn't work for me.

Try [xsettingsd](https://codeberg.org/derat/xsettingsd)
