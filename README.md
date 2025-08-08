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
<img width="2560" height="1440" alt="image" src="https://github.com/user-attachments/assets/b511c3df-2fbf-4796-b7df-f9950c55a470" />


