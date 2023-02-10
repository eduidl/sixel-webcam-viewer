# sixel-webcam-viewer

## 環境

sixel をサポートするターミナル (ex: [wezterm](https://wezfurlong.org/wezterm/))

## 依存ライブラリ

```sh-session
apt install libsixel1 libv4l-dev
```

## 実行

```sh-session
$ cargo build --release
$ ./target/release/sixel-webcam-viewer --help
webcam to sixel 0.1.0

USAGE:
    sixel-webcam-viewer [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --device <device>     [default: /dev/video0]
    -f, --fps <fps>           [default: 30]
    -h, --height <height>     [default: 480]
    -w, --width <width>       [default: 640]

$ ./target/release/sixel-webcam-viewer
```
