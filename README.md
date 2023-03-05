# ariadne
Experimental game based on macroquad

Very much a work in progress

[Check out a `wasm` version that runs in your web browser][1] - give it a few seconds to load

`ariadne` build is failing on `github` with error "/usr/bin/ld: cannot find -lasound", but does build on my `ubuntu` 22.04 setup after installing:

```sh
# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
 pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib


[1]: https://eekkaiia.github.io/ariadne
