# FOSDEM 2025 Rust Devroom Presentation 🦀

**Session**: [Bringing terminal aesthetics to the Web with Rust (and vice versa)](https://fosdem.org/2025/schedule/event/fosdem-2025-5496-bringing-terminal-aesthetics-to-the-web-with-rust-and-vice-versa-/)

## Presenting

To start the presentation, first install [presenterm](https://github.com/mfontanini/presenterm):

```bash
cargo install --git https://github.com/mfontanini/presenterm
```

Update submodules:

```bash
git submodule update --init --recursive
```

Then simply run:

```bash
presenterm presentation.md -X -c config/presenterm.yml -p
```

> [!TIP]  
> Or you can use the [`present.sh`](./present.sh) script.

> [!IMPORTANT]  
> It is recommended to use [wezterm](https://github.com/wez/wezterm) for good image rendering.
