# systemctl-menu

> A minimal fast systemctl menu written in raylib-rs for i3wm or sway


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
<img width="2560" height="1440" alt="image" src="https://github.com/user-attachments/assets/f209d8d1-0766-4441-9ca6-d6489c777d00" />
