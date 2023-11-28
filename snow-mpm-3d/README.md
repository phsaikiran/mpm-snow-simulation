# Material Point Method 3D

## For Snow Simulation

- Install Rust lang
  https://www.rust-lang.org/tools/install

```bash
cargo build --release
cargo run --release
```

- Convert saved frames to a video

```bash
ffmpeg -r 120 -f image2 -i frames/frame-%d.png -vcodec libx264 -b 20M video.mp4 
```