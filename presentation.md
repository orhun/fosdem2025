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

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

<!-- jump_to_middle -->

Chapter 1: **Web to Terminal** 💻

<!-- column: 1 -->

<!-- new_lines: 6 -->

![image:width:50%](assets/ratatui-hat.gif)

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

<!-- pause -->

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```bash +exec +acquire_terminal
tv
```

`https://github.com/alexpasmantier/television`

<!-- pause -->

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

## Minitel

Rust stack for the French videotex terminal system from the 1980s.

`https://github.com/plule/minitel`

```bash +exec
mpv --quiet --loop-file assets/minitel-ratatui.mp4
```

<!-- end_slide -->

```rust
Text::raw("Hello World!");
```

<!-- pause -->

```rust
Paragraph::new("Hello World!").wrap(Wrap { trim: true });
```

<!-- pause -->

```rust
let text = Paragraph::new("Hello World!")
    .wrap(Wrap { trim: true })
    .centered()
    .block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title_top(Line::from("Title").centered())
            .title_bottom(Line::from("!").right_aligned()),
    );
```

<!-- end_slide -->

```rust
let block = Block::bordered()
    .style(Style::default().bg(Color::Black).fg(Color::White))
    .border_type(BorderType::Rounded)
    .title(Line::from(" ✨ Title ✨ ".black().on_blue().bold()).centered())
    .title_bottom(Line::from(" ⚡ Bottom ⚡ ".black().on_green().italic()).right_aligned());

let text = Paragraph::new("Hello, World!".red())
    .wrap(Wrap { trim: true })
    .centered()
    .block(block);
```

```bash +exec +acquire_terminal
cargo run --manifest-path code/Cargo.toml --bin styling
```

<!-- end_slide -->

```bash +exec +acquire_terminal
exabind
```

<!-- pause -->

# TachyonFX

Shader-like effects library for Ratatui applications

`https://github.com/junkdog/tachyonfx`

`https://github.com/junkdog/exabind`

<!-- pause -->

```bash +exec +acquire_terminal
cargo run --manifest-path tachyonfx/Cargo.toml --example fx-chart
```

<!-- end_slide -->

```rust
Text::raw("Hello World!");
```

<!-- pause -->

```rust
pub struct Buffer {
    pub area: Rect,
    pub content: Vec<Cell>,
}
```

![](assets/rat-buffer.gif)

<!-- end_slide -->

```
    0   1   2   3   4   5   6   7   8   9  10  11
  ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
0 │ H │ e │ l │ l │ o │   │ W │ o │ r │ l │ d │ ! │
  ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤
1 │   │   │   │   │ ▲ │   │   │   │   │   │   │   │
  ├───┼───┼───┼───┼─│─┼───┼───┼───┼───┼───┼───┼───┤
2 │   │   │   │   │ │ │   │   │   │   │   │   │   │
  ├───┼───┼───┼───┼─│─┼───┼───┼───┼───┼───┼───┼───┤
3 │   │   │ ┌─────┴─────┐ │   │   │   │   │   │   │
  └───┴───┴─│ ┴───┴───┴ │ ┴───┴───┴───┴───┴───┴───┘
            │           │
      ┌─────┴─────┐  ┌──┴──┐
      │  symbol   │  │style│
      │   “o”     │  │Reset│
      └───────────┘  └─────┘
```

<!-- reset_layout -->

<!-- column_layout: [1, 7] -->

<!-- column: 1 -->

`https://ratatui.rs/concepts/rendering/under-the-hood/`

<!-- end_slide -->

### Tek

A music making program for 24-bit unicode terminals.

`https://codeberg.org/unspeaker/tek`

```bash +exec +acquire_terminal
tek_sequencer
```

![](assets/rat-dance.gif)

<!-- end_slide -->

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

<!-- jump_to_middle -->

Chapter 2: **Terminal to Web** 🌐

<!-- column: 1 -->

<!-- new_lines: 5 -->

![image:width:70%](assets/absolute-cinema.gif)

<!-- end_slide -->

# Xterm.js

```html {1-14|11}
<html>
  <head>
    <link rel="stylesheet" href="node_modules/@xterm/xterm/css/xterm.css" />
    <script src="node_modules/@xterm/xterm/lib/xterm.js"></script>
  </head>
  <body>
    <div id="terminal"></div>
    <script>
      var term = new Terminal();
      term.open(document.getElementById("terminal"));
      term.write("Hello from \x1B[1;3;31mxterm.js\x1B[0m");
    </script>
  </body>
</html>
```

<!-- end_slide -->

## Ratatui's Backend

```rust {1-16|2-3}
pub trait Backend {
    fn draw<'a, I>(&mut self, content: I) -> Result<()>
       where I: Iterator<Item = (u16, u16, &'a Cell)>;
    fn hide_cursor(&mut self) -> Result<()>;
    fn show_cursor(&mut self) -> Result<()>;
    fn get_cursor_position(&mut self) -> Result<Position>;
    fn set_cursor_position<P: Into<Position>>(
        &mut self,
        position: P,
    ) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
    fn size(&self) -> Result<Size>;
    fn window_size(&mut self) -> Result<WindowSize>;
    fn flush(&mut self) -> Result<()>;
    // ...
}
```

<!-- end_slide -->

```rust
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub underline_color: Color,
    pub modifier: Modifier,
    pub skip: bool,
}
```

```rust {1-9|4}
impl Backend for TestBackend {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            self.buffer[(x, y)] = c.clone();
        }
        Ok(())
    }
}
```

<!-- end_slide -->

# egui_ratatui

A ratatui backend that is also an egui widget.

`https://github.com/gold-silver-copper/egui_ratatui`

Demo: https://gold-silver-copper.github.io/

```bash +exec
cargo run --manifest-path egui_ratatui/bevy_example/Cargo.toml
```

<!-- end_slide -->

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<BevyTerminal<RataguiBackend>>()
```

```rust
impl Backend for RataguiBackend { ... }
```

```rust
impl egui::Widget for &mut RataguiBackend { ... }
```

![](assets/rat-cup.gif)

<!-- end_slide -->

# webatui

An integration between the Yew framework and Ratatui  
for making TUI-themed WASM webapps

`https://github.com/TylerBloom/webatui`

Demo: https://avid-rustacean.shuttleapp.rs/

```bash +exec
pkill trunk
cd webatui/examples/counter
trunk serve
```

<!-- end_slide -->

1. Create a struct that will hold your app's logic
2. Implement the `TerminalApp` trait for it
3. Run the `run_tui` function with an instance of your app.

```rust
struct MyApp { title: String }

impl TerminalApp for MyApp {
    type Message = ();
    fn update(
      &mut self, _: TermContext<'_, Self>,
      _: Self::Message) -> bool { false }

    fn render(&self, area: Rect, frame: &mut Frame<'_>) {
        let para = Paragraph::new(self.title.as_str());
        frame.render_widget(para, area);
    }
}

run_tui(MyApp { title: "Hello WWW!".into() })
```

<!-- end_slide -->

![image:width:90%](assets/render-diagram-1.png)

![image:width:20%](assets/rat-hydration.gif)

<!-- end_slide -->

![image:width:30%](assets/rat-work.gif)

<!-- column_layout: [1, 4] -->

<!-- column: 1 -->

We want:

1. No Javascript, only Rust
2. Only depend on Ratatui
3. Easy to set up
4. Easy to convert existing apps

_"Write Rust once, run everywhere."_

<!-- end_slide -->

<!-- new_lines: 3 -->

![](assets/rat-cook.gif)

<!-- end_slide -->

![image:width:40%](assets/render-diagram-2.png)
