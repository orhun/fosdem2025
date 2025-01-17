---
title: Bringing Terminal Aesthetics to the Web with Rust (and vice versa)
sub_title: FOSDEM 2025
author: Orhun Parmaksız
theme:
  path: config/theme.yml
---

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

<!-- new_lines: 1 -->

![](assets/orhun.png)

<!-- column: 1 -->

## _Orhun Parmaksız_

✨ Open source, Rust and terminals!

🦀 **Ratatui**, **git-cliff**, binsider, kmon, systeroid…

📦 **Arch Linux** (btw)

`https://orhun.dev             `

`https://github.com/orhun      `

`https://youtube.com/@orhundev `

<!-- end_slide -->

<!-- column_layout: [3, 6] -->

<!-- column: 0 -->

# Ratatui

![image:width:70%](assets/ratatui-spin.gif)

A Rust library that's all about cooking up terminal user interfaces (TUIs) 👨‍🍳🐀

`https://github.com/ratatui`

<!-- column: 1 -->

```rust {1-20|5|6|7,16-18|8-11|12-14|1-20}
use ratatui::crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw)?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
```

<!-- end_slide -->

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

## Widgets

- Block
- BarChart
- Calendar
- Canvas
- Chart
- Gauge
- LineGauge
- List
- Paragraph
- Scrollbar
- Sparkline
- Table
- Tabs

![](./assets/rat-spin.gif)

<!-- column: 1 -->

## Concepts

- Rendering
  - **Buffer** ✨
- Layout
- Application patterns
- Backends
- Event handling

`https://ratatui.rs/concepts`

# Demo

```bash +exec +acquire_terminal
cargo run --manifest-path ratatui/examples/apps/demo2/Cargo.toml
```

<!-- end_slide -->

# "The TUI Look"

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```bash +exec +acquire_terminal
tv
```

`https://github.com/alexpasmantier/television`

```bash +exec +acquire_terminal
tracker
```

`https://github.com/ShenMian/tracker`

<!-- end_slide -->

![image:width:75%](./assets/rusty-tape.gif)

<!-- column_layout: [1, 2] -->

<!-- column: 1 -->

🖥️ Pane-based layout  
🗂️ Pop-up menus  
🎨 Highlight colors

<!-- end_slide -->

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

**Minitel**: Rust stack for the French videotex terminal system from the 1980s.

`https://github.com/plule/minitel`

```bash +exec
mpv --quiet --loop-file assets/minitel-ratatui.mp4
```
