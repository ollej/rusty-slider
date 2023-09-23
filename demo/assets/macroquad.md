# Macroquad

---

## Vad är Macroquad?

Ett enkelt och lättanvänt spelramverk i Rust

Innehåller allt för att bygga ett 2D-spel!

Utvecklas av Fedor Logachev

https://macroquad.rs

---

## Designmål

Några av Macroquads designmål:

 * Snabb kompilering
 * Korsplattform
 * Stödjer budgetenheter
 * Hackbar

---

## Allt inkluderat

 * 2D-rendering
 * Audio
 * Immediate mode UI
 * Bygg för Android/iOS/WASM med endast ett kommando

---

## Baserat på Miniquad

Miniquad är ett minimalt grafikabstraktionslager

---

## Stödda plattformar

 * Windows
 * MacOS
 * Linux
 * HTML5 / WebAssembly
 * Android
 * iOS

---

## Minimalt exempel

```rust
use macroquad::prelude::*;

#[macroquad::main("Macroquad")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        next_frame().await
    }
}
```

---

## Minimalt exempel

 * Importera macroquad
 * Macro för main-funktionen
 * Asynkron main-funktion
 * Rensa bakgrunden i början av loopen
 * Avsluta med att vänta på nästa frame

---

## Fönsterkonfiguration

 * Macrot skapar ett fönster.
 * Det går även att konfigurera fönstret.
 * Använd en funktion som returnerar `Conf`.
 * Ändra storlek, fullskärm, titel med mera

---

## Fönsterkonfiguration

```rust
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window name".to_owned(),
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        next_frame().await
    }
}
```

---

## Visa bild

 * Ladda bild med `load_texture()`
 * Stödjer endast PNG
 * Visa bild med `draw_texture()`
 * Placera med `screen_width()` och `screen_height()`
 * Använd bakgrundsfärgen `WHITE`
 * Laddar bilden asynkront
 * Fungerar båda lokalt och via WebAssembly

---

## Visa bild

```rust
use macroquad::prelude::*;

#[macroquad::main("Bild")]
async fn main() {
    let texture: Texture2D = load_texture("examples/ferris.png").await.unwrap();
    loop {
        clear_background(LIGHTGRAY);
        draw_texture(
            texture,
            screen_width() / 2. - texture.width() / 2.,
            screen_height() / 2. - texture.height() / 2.,
            WHITE,
        );
        next_frame().await
    }
}
```

---

## Texture API

Metoder för att arbeta med texturer.

 * `build_textures_atlas()`
 * `draw_texture( ... )`
 * `draw_texture_ex( ... )`
 * `get_screen_data()`
 * `load_image( ... )`
 * `load_texture( ... )`
 * `render_target( ... )`

---

## Rita

Stöd för att rita enklare figurer med bakgrundsfärg.

 * Cirkel
 * Rektangel
 * Linje
 * Hexagon
 * Triangel
 * Polygon

---

## Rita

```rust
use macroquad::prelude::*;

#[macroquad::main("Rita")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        next_frame().await
    }
}
```

---

## Shapes API

 * `draw_circle(x: f32, y: f32, r: f32, color: Color)`
 * `draw_hexagon( ... )`
 * `draw_line( ... )`
 * `draw_poly( ... )`
 * `draw_rectangle( ... )`
 * `draw_triangle( ... )`

---

## Rita

Rita figurer med konturer.

---

## Shapes API lines

 * `draw_circle_lines( ... )`
 * `draw_poly_lines( ... )`
 * `draw_rectangle_lines( ... )`
 * `draw_triangle_lines( ... )`

---

### Text

Fonter kan användas för att visa text

 * Ladda font med `load_ttf_font()`
 * Använd `measure_text()` för att räkna ut baslinjen
 * Visa texten med `draw_text()` eller `draw_text_ex()`
 * Y-koordinaten börjar uppifrån
 * Y-koordinaten anger baslinjen

---

## Text

```rust
use macroquad::prelude::*;

#[macroquad::main("Text")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        let text = "Hej världen!";
        let font_size = 30;
        let font = load_ttf_font("examples/Hack-Regular.ttf").await.ok();
        let dim = measure_text(text, font, font_size, 1.0);
        draw_text(text, 30.0, dim.offset_y, font_size as f32, DARKGRAY);
        next_frame().await
    }
}
```

---

### Text API

 * `camera_font_scale( ... )`
 * `draw_text( ... )`
 * `draw_text_ex( ... )`
 * `get_text_center( ... )`
 * `load_ttf_font( ... )`
 * `load_ttf_font_from_bytes( ... )`
 * `measure_text( ... )`

---

## Hantera input

```rust
use macroquad::prelude::*;

#[macroquad::main("Input")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Right) { x += 1.0; }
        if is_key_down(KeyCode::Left) { x -= 1.0; }
        if is_key_down(KeyCode::Down) { y += 1.0; }
        if is_key_down(KeyCode::Up) { y -= 1.0; }

        draw_circle(x, y, 15.0, YELLOW);
        next_frame().await
    }
}
```

---

## Tangentbordsinput

 * `get_char_pressed() -> Option<char>`
 * `get_last_key_pressed() -> Option<KeyCode>`
 * `is_key_down(key_code: KeyCode)`
 * `is_key_pressed(key_code: KeyCode)`
 * `is_key_released(key_code: KeyCode)`

---

## KeyCode enum

 * `Space`
 * `Up`
 * `Down`
 * `Left`
 * `Right`
 * `A .. Z`
 * `0 .. 9`
 * ...

---

## Mus-input

 * `is_mouse_button_down(btn: MouseButton)`
 * `is_mouse_button_pressed(btn: MouseButton)`
 * `is_mouse_button_released(btn: MouseButton)`
 * `mouse_position -> (f32, f32)`
 * `mouse_position_local -> Vec2`
 * `mouse_wheel -> (f32, f32)`
 * `set_cursor_grab(grab: bool)`
 * `show_mouse(shown: bool)`

---

## MouseButton enum

 * `Right`
 * `Left`
 * `Middle`
 * `Unknown`

---

## Touch input för mobiler

 * `simulate_mouse_with_touch(option: bool)`
 * `touches -> Vec<Touch>`
 * `touches_local -> Vec<Touch>`

---

## TouchPhase enum

 * `Started`
 * `Stationary`
 * `Moved`
 * `Ended`
 * `Cancelled`

---

## Ljud

```rust
use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams},
    window::next_frame,
};

#[macroquad::main("Ljud")]
async fn main() {
    let sound = load_sound("examples/sound.wav").await.unwrap();
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume: 1.,
        },
    );
    loop {
        set_sound_volume(sound, 0.1);
        next_frame().await;
    }
}
```

---

## Sound API

 * `load_sound(path: &str)`
 * `load_sound_from_bytes(data: &[u8])`
 * `play_sound(sound: Sound, params: PlaySoundParams)`
 * `play_sound_once(sound: Sound)`
 * `set_sound_volume(sound: Sound, volume: f32)`
 * `stop_sound(sound: Sound)`

---

## Filladdning

```rust
use macroquad::prelude::*;

#[macroquad::main("File")]
async fn main() -> Result<(), FileError> {
    set_pc_assets_folder("assets");
    let text = load_string("test.txt").await?;
    println!("Content: {}", text);
    Ok(())
}
```

---

## File API

 * `load_file(path: &str)`
 * `load_string(path: &str)`
 * `set_pc_assets_folder(path: &str)`

---

## Tid

```rust
use macroquad::prelude::*;

#[macroquad::main("Time")]
async fn main() {
    let start_time = get_time();
    let mut elapsed: f32 = 0.0;
    loop {
        elapsed += get_frame_time();
        if elapsed > 1.0 {
            println!(
                "FPS: {} Elapsed time: {:.3}",
                get_fps(),
                get_time() - start_time
            );
            elapsed = 0.0;
        }
        next_frame().await
    }
}
```

---

## Time API

 * `get_fps() -> i32`
 * `get_frame_time() -> f32`
 * `get_time() -> f64`

---

## Färger

```rust
use macroquad::prelude::*;

#[macroquad::main("Colors")]
async fn main() {
    let red = Color::from_rgba(255, 0, 0, 0);
    loop {
        clear_background(red);
        next_frame().await
    }
}
```

---

## Color API

 * `Color::new(r: f32, g: f32, b: f32, a: f32)`
 * `Color::from_rgba(r: u8, g: u8, b: u8, a: u8)`
 * `Color::from_vec(vec: Vec4)`
 * `hsl_to_rgb(h: f32, s: f32, l: f32)`
 * `rgb_to_hsl(color: Color) -> (f32, f32, f32)`
 * `macroquad::color::colors::BLACK` ...

---

## Slumpning

```rust
use macroquad::{miniquad, rand::{self, ChooseRandom}};

#[macroquad::main("Particles")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let dice = rand::gen_range(1, 6);
    let list = vec!["foo", "bar", "baz"];
    let string = list.choose().unwrap();
    println!("Tärningsslag: {}, Slumpad text: {}", dice, string);
}
```

---

## Rand API

 * `gen_range<T>(low: T, high: T) -> T`
 * `rand() -> u32`
 * `srand(seed: u64)`

---

## ChooseRandom trait

 * `shuffle`
 * `choose`
 * `choose_mut`
 * `choose_multiple(amount: usize)`

---

## Partiklar

```rust
use macroquad::prelude::*;
use macroquad_particles::{Curve, Emitter, EmitterConfig, ParticleShape};

#[macroquad::main("Particles")]
async fn main() {
    let mut emitter = Emitter::new(EmitterConfig {
        initial_velocity: 500.0,
        initial_direction_spread: 2. * std::f32::consts::PI,
        shape: ParticleShape::Circle { subdivisions: 360 },
        size_curve: Some(Curve {
            points: vec![(0.0, 0.5), (0.5, 1.0), (1.0, 0.0)],
            ..Default::default()
        }),
        ..Default::default()
    });

    loop {
        clear_background(BLACK);
        emitter.draw(vec2(screen_width() / 2., screen_height() / 2.));
        next_frame().await
    }
}
```

---

## Olle Wreede

@ollej@hachyderm.io

![Agical](assets/agical-logo.png)
