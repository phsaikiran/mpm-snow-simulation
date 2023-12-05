# Material Point Method 2D

## For Snow Simulation

- Install Rust lang
  https://www.rust-lang.org/tools/install

```bash
cd snow-mpm-2d
cargo build --release
cargo run --release
```

# Material Point Method 3D

## For Snow Simulation

- Install Rust lang
  https://www.rust-lang.org/tools/install

```bash
cd snow-mpm-3d
cargo build --release
cargo run --release 25
```

- Convert saved frames to a video

```bash
ffmpeg -r 120 -f image2 -i frames/frame-%d.png -vcodec libx264 -b 20M video.mp4 
```