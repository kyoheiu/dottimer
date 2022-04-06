# dottimer
a systemd timer generator that is not insane

![sample.gif](screenshots/sample.gif)

## Installation
```
git clone https://github.com/kyoheiu/dottimer.git
cd dottimer
cargo install --path .
```

## Usage
```
OPTIONS:
    -H, --help
            Print help information
    -o
            Enable to choose options like AccuracySec or Persistent
```

This app does NOT make `foo.timer` and place it in the right place automatially, but simply generate a text that can be used for `.timer` file as it is.

For usage of systemd timer itself, see [systemd.timer(5)](https://man.archlinux.org/man/systemd.timer.5) and [systemd.time(7)](https://man.archlinux.org/man/systemd.time.7.en).
