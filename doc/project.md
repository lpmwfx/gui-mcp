# GUI Vision MCP Server (Rust)

Public repo github gennem gh lpmwfx repo navn gui vision mcp server - med README.md om projektet. Der er logget på gh

Sprog er Engelsk i al kode og dokumentation 

## Formål

En MCP-server skrevet i Rust der giver AI-assistenter (fx Claude Code) syn og betjening
af et kørende Slint-vindue på Windows. Løser problemet med at AI gætter på UI-placering
og udseende ved at give direkte visuel feedback og billedbaseret interaktion.

## Kernekoncept

I stedet for koordinatbaseret klik bruges **billedgenkendelse** (template matching):
AI sender et crop af et UI-element → serveren finder det på skærmen → klikker præcist.
Robust over for layout-ændringer. AI behøver ikke kende koordinater på forhånd.

---

## Stack

- **Rust** (stable)
- **rmcp** – MCP-protokol (community crate, `rmcp` på crates.io)
- **windows** crate – Win32 API, HWND-baseret window capture
- **image** crate – billedbehandling og PNG encode/decode
- **imageproc** crate – template matching (normalized cross-correlation)
- **enigo** crate – mus og tastaturinput (cross-platform, fungerer på Windows)
- **base64** crate – encode/decode af billeder til/fra MCP
- **serde / serde_json** – JSON serialisering
- **tokio** – async runtime (kræves af rmcp)

---

## Projektstruktur

```
slint-gui-mcp/
├── src/
│   ├── main.rs         # MCP-server entry point og tool dispatch
│   ├── window.rs       # Win32 HWND window capture og info
│   ├── vision.rs       # Template matching
│   └── actions.rs      # Klik, scroll, tastatur via enigo
├── Cargo.toml
└── README.md
```

---

## Cargo.toml

```toml
[package]
name = "slint-gui-mcp"
version = "0.1.0"
edition = "2021"

[dependencies]
rmcp = { version = "0.1", features = ["server", "transport-io"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
base64 = "0.22"
image = "0.25"
imageproc = "0.25"
enigo = "0.2"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_Graphics_Gdi",
    "Win32_System_Memory",
] }
```

---

## MCP Tools der skal implementeres

### `screenshot_window`
Tag et screenshot af det Slint-vindue der matcher `window_title`.

**Input:**
```json
{
  "window_title": "string  // Delvist match på vinduestitel er nok"
}
```

**Output:**
```json
{
  "image_base64": "string  // Base64-encoded PNG",
  "width": 800,
  "height": 600,
  "window_title": "My Slint App"
}
```

**Implementation notes:**
- Brug `EnumWindows` til at finde HWND via delvist titelmatch (case-insensitive)
- Bring vinduet i forgrunden med `SetForegroundWindow` inden capture
- Capture via `GetWindowDC` + `CreateCompatibleBitmap` + `BitBlt`
- Konverter BGRA bitmap-bits til `image::RgbImage` og encode som PNG til base64

---

### `find_element`
Find et UI-element på skærmen vha. template matching. Returnerer koordinater uden at klikke.

**Input:**
```json
{
  "window_title": "string",
  "template_base64": "string  // Base64-encoded PNG-crop af elementet der søges",
  "confidence": 0.8
}
```

**Output:**
```json
{
  "found": true,
  "x": 450,
  "y": 230,
  "width": 80,
  "height": 30,
  "confidence": 0.94
}
```

**Implementation notes:**
- Tag screenshot internt via `screenshot_window`
- Kør normalized cross-correlation (se vision.rs nedenfor)
- Returner centrum-koordinat af bedste match
- `found: false` hvis bedste match er under `confidence`-tærsklen

---

### `click_element`
Find et UI-element vha. template matching og klik på det.

**Input:**
```json
{
  "window_title": "string",
  "template_base64": "string",
  "confidence": 0.8,
  "button": "left"
}
```

**Output:**
```json
{
  "success": true,
  "clicked_at": {"x": 450, "y": 230},
  "confidence": 0.94,
  "screenshot_after": "string  // Base64 PNG efter klik – AI bekræfter resultatet"
}
```

**Implementation notes:**
- Kald internt `find_element`
- Bring vinduet i forgrunden
- Brug `enigo` til at flytte mus og klikke på absolutte skærmkoordinater
  (vinduets rect.left + match_x, rect.top + match_y)
- Tag screenshot efter klik og returner det

---

### `type_text`
Skriv tekst (antager fokus er sat korrekt via foregående klik).

**Input:**
```json
{
  "text": "string"
}
```

**Output:**
```json
{ "success": true }
```

---

### `send_keys`
Send tastaturgenvej eller specialtast.

**Input:**
```json
{
  "keys": "string  // fx 'enter', 'tab', 'escape', 'ctrl+s'"
}
```

**Output:**
```json
{ "success": true }
```

**Implementation notes:**
- Parse key-streng og map til `enigo::Key`
- Håndter modifier+key kombinationer som "ctrl+s":
  press ctrl → click s → release ctrl

---

### `get_window_info`
Hent information om et specifikt vindue.

**Input:**
```json
{
  "window_title": "string"
}
```

**Output:**
```json
{
  "found": true,
  "title": "My Slint App",
  "hwnd": 12345678,
  "rect": {"left": 100, "top": 100, "right": 900, "bottom": 700},
  "width": 800,
  "height": 600,
  "is_visible": true,
  "is_minimized": false
}
```

---

### `list_windows`
List alle synlige vinduer – bruges til at finde det rigtige vinduestitel.

**Input:** ingen

**Output:**
```json
{
  "windows": [
    {"hwnd": 12345, "title": "My Slint App"},
    {"hwnd": 67890, "title": "Explorer"}
  ]
}
```

---

## Implementationsdetaljer

### window.rs – HWND capture

```rust
use windows::Win32::Foundation::{HWND, BOOL, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;
use image::RgbImage;

pub fn find_window_by_partial_title(partial: &str) -> Option<HWND> {
    let partial_lower = partial.to_lowercase();
    let mut found: Option<HWND> = None;

    unsafe {
        EnumWindows(Some(enum_callback), LPARAM(&mut found as *mut _ as isize));
    }

    // enum_callback filtrerer på partial_lower og sætter found
    found
}

pub fn capture_window(hwnd: HWND) -> Option<RgbImage> {
    unsafe {
        let mut rect = RECT::default();
        GetWindowRect(hwnd, &mut rect).ok()?;

        let width  = (rect.right  - rect.left) as i32;
        let height = (rect.bottom - rect.top)  as i32;

        let hdc_window = GetWindowDC(hwnd);
        let hdc_mem    = CreateCompatibleDC(hdc_window);
        let hbm        = CreateCompatibleBitmap(hdc_window, width, height);
        SelectObject(hdc_mem, hbm);

        BitBlt(hdc_mem, 0, 0, width, height,
               hdc_window, 0, 0, SRCCOPY).ok()?;

        let mut bmi = BITMAPINFOHEADER {
            biSize:        std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth:       width,
            biHeight:      -height,   // Negativ = top-down
            biPlanes:      1,
            biBitCount:    32,
            biCompression: BI_RGB.0,
            ..Default::default()
        };

        let mut buf = vec![0u8; (width * height * 4) as usize];
        GetDIBits(hdc_mem, hbm, 0, height as u32,
                  Some(buf.as_mut_ptr() as *mut _),
                  &mut BITMAPINFO { bmiHeader: bmi, ..Default::default() },
                  DIB_RGB_COLORS);

        // Cleanup
        DeleteObject(hbm);
        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_window);

        // BGRA → RGB
        let rgb: Vec<u8> = buf.chunks(4)
            .flat_map(|p| [p[2], p[1], p[0]])
            .collect();

        RgbImage::from_raw(width as u32, height as u32, rgb)
    }
}
```

---

### vision.rs – Template matching uden OpenCV

```rust
use image::RgbImage;

pub struct MatchResult {
    pub x: u32,          // Centrum-x relativt til screenshot
    pub y: u32,          // Centrum-y relativt til screenshot
    pub confidence: f32,
}

/// Normalized cross-correlation template matching.
/// Returnerer centrum-koordinat af bedste match hvis over threshold.
pub fn find_template(
    screenshot: &RgbImage,
    template:   &RgbImage,
    threshold:  f32,
) -> Option<MatchResult> {
    let (sw, sh) = screenshot.dimensions();
    let (tw, th) = template.dimensions();

    if tw > sw || th > sh { return None; }

    // Forudberegn template-statistik
    let t_pixels: Vec<f32> = template.pixels()
        .flat_map(|p| p.0.iter().map(|&c| c as f32))
        .collect();
    let t_mean: f32 = t_pixels.iter().sum::<f32>() / t_pixels.len() as f32;
    let t_std:  f32 = variance(&t_pixels, t_mean).sqrt();

    let mut best_score = f32::NEG_INFINITY;
    let mut best_x = 0u32;
    let mut best_y = 0u32;

    for y in 0..=(sh - th) {
        for x in 0..=(sw - tw) {
            let score = ncc_at(screenshot, &t_pixels, x, y, tw, th, t_mean, t_std);
            if score > best_score {
                best_score = score;
                best_x = x;
                best_y = y;
            }
        }
    }

    if best_score >= threshold {
        Some(MatchResult {
            x: best_x + tw / 2,
            y: best_y + th / 2,
            confidence: best_score,
        })
    } else {
        None
    }
}

fn ncc_at(
    screen:  &RgbImage,
    t_pixels: &[f32],
    ox: u32, oy: u32, tw: u32, th: u32,
    t_mean: f32, t_std: f32,
) -> f32 {
    let n = (tw * th * 3) as f32;

    let r_pixels: Vec<f32> = (oy..oy+th)
        .flat_map(|y| (ox..ox+tw)
            .flat_map(move |x| screen.get_pixel(x, y).0.iter().map(|&c| c as f32).collect::<Vec<_>>()))
        .collect();

    let r_mean: f32 = r_pixels.iter().sum::<f32>() / n;
    let r_std:  f32 = variance(&r_pixels, r_mean).sqrt();

    if r_std < 1e-6 || t_std < 1e-6 { return 0.0; }

    r_pixels.iter().zip(t_pixels.iter())
        .map(|(&r, &t)| (r - r_mean) * (t - t_mean))
        .sum::<f32>() / (n * r_std * t_std)
}

fn variance(pixels: &[f32], mean: f32) -> f32 {
    pixels.iter().map(|&v| (v - mean).powi(2)).sum::<f32>() / pixels.len() as f32
}
```

**Performance note:** NCC er O(W×H×tw×th) – tilstrækkeligt til Slint UI-elementer
der typisk er små (knapper, inputfelter). Til store templates kan FFT-baseret matching
tilføjes med `rustfft` i en senere version.

---

### actions.rs – Mus og tastatur

```rust
use enigo::{Enigo, Mouse, Keyboard, Button, Key, Settings, Coordinate, Direction};

pub fn click_at(x: i32, y: i32, button: &str) -> Result<(), String> {
    let mut e = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    e.move_mouse(x, y, Coordinate::Abs).map_err(|e| e.to_string())?;

    let btn = match button {
        "right"  => Button::Right,
        "middle" => Button::Middle,
        _        => Button::Left,
    };
    e.button(btn, Direction::Click).map_err(|e| e.to_string())
}

pub fn type_text(text: &str) -> Result<(), String> {
    let mut e = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    e.text(text).map_err(|e| e.to_string())
}

pub fn send_keys(keys: &str) -> Result<(), String> {
    let mut e = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

    let parts: Vec<&str> = keys.split('+').collect();
    let (modifiers, last) = parts.split_at(parts.len() - 1);

    for m in modifiers {
        e.key(parse_key(m)?, Direction::Press).map_err(|err| err.to_string())?;
    }
    e.key(parse_key(last[0])?, Direction::Click).map_err(|err| err.to_string())?;
    for m in modifiers.iter().rev() {
        e.key(parse_key(m)?, Direction::Release).map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn parse_key(s: &str) -> Result<Key, String> {
    match s.to_lowercase().as_str() {
        "ctrl" | "control" => Ok(Key::Control),
        "alt"              => Ok(Key::Alt),
        "shift"            => Ok(Key::Shift),
        "enter" | "return" => Ok(Key::Return),
        "tab"              => Ok(Key::Tab),
        "escape" | "esc"   => Ok(Key::Escape),
        "space"            => Ok(Key::Space),
        "backspace"        => Ok(Key::Backspace),
        "delete" | "del"   => Ok(Key::Delete),
        "up"               => Ok(Key::UpArrow),
        "down"             => Ok(Key::DownArrow),
        "left"             => Ok(Key::LeftArrow),
        "right"            => Ok(Key::RightArrow),
        c if c.len() == 1  => Ok(Key::Unicode(c.chars().next().unwrap())),
        other              => Err(format!("Ukendt key: {other}")),
    }
}
```

---

### main.rs – MCP Server

```rust
use rmcp::{ServerHandler, model::*, tool, Error as McpError};

#[derive(Clone)]
struct SlintGuiServer;

#[tool(tool_box)]
impl SlintGuiServer {
    #[tool(description = "Tag screenshot af et navngivet Slint-vindue")]
    async fn screenshot_window(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        todo!()
    }

    #[tool(description = "Find UI-element via billedgenkendelse uden at klikke")]
    async fn find_element(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] template_base64: String,
        #[tool(param)] confidence: Option<f32>,
    ) -> Result<CallToolResult, McpError> {
        todo!()
    }

    #[tool(description = "Find UI-element via billedgenkendelse og klik på det")]
    async fn click_element(
        &self,
        #[tool(param)] window_title: String,
        #[tool(param)] template_base64: String,
        #[tool(param)] confidence: Option<f32>,
        #[tool(param)] button: Option<String>,
    ) -> Result<CallToolResult, McpError> {
        todo!()
    }

    #[tool(description = "Skriv tekst til fokuseret element")]
    async fn type_text(
        &self,
        #[tool(param)] text: String,
    ) -> Result<CallToolResult, McpError> {
        todo!()
    }

    #[tool(description = "Send tastaturgenvej fx 'ctrl+s', 'enter', 'tab'")]
    async fn send_keys(
        &self,
        #[tool(param)] keys: String,
    ) -> Result<CallToolResult, McpError> {
        todo!()
    }

    #[tool(description = "Hent info om et specifikt vindue")]
    async fn get_window_info(
        &self,
        #[tool(param)] window_title: String,
    ) -> Result<CallToolResult, McpError> {
        todo!()
    }

    #[tool(description = "List alle synlige vinduer")]
    async fn list_windows(&self) -> Result<CallToolResult, McpError> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let transport = rmcp::transport::io::stdio();
    rmcp::serve_server(SlintGuiServer, transport).await.unwrap();
}
```

---

## Typisk workflow for AI dev

```
1. list_windows()
   → Find titlen på Slint-vinduet

2. screenshot_window(window_title: "My App")
   → Se hvad der aktuelt vises

3. [AI inspicerer screenshot og laver et crop af det element den vil interagere med]

4. click_element(window_title: "My App", template_base64: "<crop>")
   → Server finder elementet og klikker – ingen koordinater nødvendige
   → Returnerer screenshot_after så AI automatisk bekræfter at UI reagerede

5. [Gentag efter behov]
```

---

## MCP konfiguration (claude_desktop_config.json)

```json
{
  "mcpServers": {
    "slint-gui": {
      "command": "C:/path/to/slint-gui-mcp.exe"
    }
  }
}
```

Et enkelt binary – ingen runtime, ingen dependencies der skal installeres.

---

## Kendte begrænsninger

- **Kun Windows** – `windows` crate og HWND er Windows-specifikt.
- **Minimerede vinduer** – `BitBlt` via HWND virker ikke for minimerede vinduer.
  Kald `ShowWindow(hwnd, SW_RESTORE)` inden capture.
- **DPI-skalering** – ved Windows display scaling (fx 150%) afviger koordinater.
  Sæt DPI-awareness i manifest eller kald `SetProcessDpiAwarenessContext` ved startup.
- **NCC performance** – tilstrækkelig til små UI-elementer. Overvej `rustfft`-baseret
  matching hvis store templates er et problem.
- **rmcp stabilitet** – community crate, pin til specifik version i Cargo.toml.
