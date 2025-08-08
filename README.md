# systemctl-menu

> A minimal fast systemctl menu written in raylib-rs, meant for i3wm


## Example i3wm Config Usage

```
set $alt Mod1

bindsym Control+$alt+Delete exec --no-startup-id ~/.local/bin/systemctl-menu

```

## Build

```
cargo build --release

# copy executable to your bin folder
cp ./target/release/systemctl-menu ~/.local/bin
```

## Screenshot
<img width="2560" height="1440" alt="image" src="https://github.com/user-attachments/assets/6a49cf65-0e97-4be8-b6a8-9e0793565252" />
