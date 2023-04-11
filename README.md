# virtual-camera
# Compliation and running
In order to compile the app you'll have to have `sdl2` package on your native platform installed.
A simple guide is here : https://github.com/Rust-SDL2/rust-sdl2
After that, in the directory run:
```
cargo build --release
```
Then go to `target -> release -> virtual-camera(.exe)` and run the program.

# Controlls
- arrow keys to move around,
- `z` to zoom, `x` to de-zoom, 'c' to reset zoom,
- `w` to look up, `s` to look down,
- `q` to turn left, `e` to turn right
