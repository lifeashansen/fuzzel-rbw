<h3 align="center">
    <img src="https://raw.githubusercontent.com/sammhansen/fuzzel-rbw/develop/.assets/bitwarden.png" width="100" alt="Logo"/><br/>
    Fuzzel RBW
</h3>

<p align="center">
    <a href="https://github.com/sammhansen/fuzzel-rbw/stargazers"><img src="https://img.shields.io/github/stars/sammhansen/fuzzel-rbw?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
    <a href="https://github.com/sammhansen/fuzzel-rbw/issues"><img src="https://img.shields.io/github/issues/sammhansen/fuzzel-rbw?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
    <a href="https://github.com/sammhansen/fuzzel-rbw/contributors"><img src="https://img.shields.io/github/contributors/sammhansen/fuzzel-rbw?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

<p align="center">
    🦀 frbw —  a minimal rust-powered tool that brings the power of Bitwarden to your Wayland desktop using <a href="https://github.com/doy/rbw">rbw</a> and <a href="https://codeberg.org/dnkl/fuzzel">fuzzel</a>.
</p>

# Configuration
- Fuzzel RBW currently supports a custom
   - `prompt`: shows just before the placeholder, default `> `
   - `placeholder`: self explanatory, default `select an entry`
   - `lines`: number of vertical lines for fuzzel to display, default `6`
   - `notifications`: whether to send notifications via `notify-send`, default `true`
   - `copy_user_exit_code`: exit code for copying a username to clipboard
   - `copy_password_exit_code`: exit code for copying a password to clipboard
   - `type_user_exit_code`: exit code for typing the username individually
   - `type_password_exit_code`: exit code for typing the password individually
     
- The config file is expected to be at `$HOME/.config/fuzzel-rbw/config.json` and will be automatically generated if it does not exist
```
{
  "placeholder": "select an entry",
  "prompt": "> ",
  "lines": 6,
  "notifications": true,
  "copy_user_exit_code": 10,
  "copy_password_exit_code": 11,
  "type_user_exit_code": 12,
  "type_password_exit_code": 13
}
```

- You also need to set custom keybinds in your fuzzel configuration file `$HOME/.config/fuzzel/fuzzel.ini` to return different exit status codes
```
{
  [key-bindings]
  custom-1=Control+Shift+u # sends code 10
  custom-2=Control+Shift+p
  custom-3=Control+Mod1+u
  custom-4=Control+Mod1+p
}
```

- You can use anything from `custom-1` to `custom-19` and they return exit status codes `10 - 28` respectively.
- If nothing happens when you press a keybind, you might be trying to remap a default keybind. Run fuzzel on terminal to see where the conflict is

- To understand more about how custom keybinds work in fuzzel, please [read the wiki !](https://man.archlinux.org/man/fuzzel.ini.5.en)

# Pinentry
- RBW allows you to specify the pinentry binary to use. Fuzzel RBW comes with `pinentry-fuzzel`. To use it add the block below to `~/.config/rbw/config.json`
  <br>
  ```json
  {
    "pinentry": "pinentry-fuzzel"
  }

> [!NOTE]
> Currently pinentry-fuzzel only supports `GETPIN`
  
> [!WARNING]
> Before using pinentry-fuzzel, please take your time to understand why pinentry exists in the first place and the risks involved. Pinentry-fuzzel only aims to make things more convenient and does not provide the security that comes with traditional pinentry clients

# Installation
## AUR
  ```
  paru -S fuzzel-rbw
  ```
  alternatively if you use yay
  ```
  yay -S fuzzel-rbw
  ```
## Build from source
### Prequisites
- You need rust - <a href="https://www.rust-lang.org/tools/install">install rust</a>

  
- Clone this repo
  ```
  git clone https://github.com/sammhansen/fuzzel-rbw.git
  ```
- Build
  ```
  cargo build --release --locked
  ```
- Copy the binary to `/usr/bin/`
  ```
  sudo cp target/release/frbw /usr/bin/
  ```
- Copy the `pinentry-fuzzel` binary to `/usr/bin/`
  ```
  sudo cp target/release/pinentry-fuzzel /usr/bin/
  ```
- Copy the logo to `/usr/share/pixmaps` 
  ```
  sudo cp .assets/bitwarden.png /usr/share/pixmaps/bitwarden.png
  ```
  
