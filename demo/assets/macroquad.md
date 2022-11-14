# Macroquad

---

## Vad är Macroquad?

Macroquad är ett enkelt och lättanvänt spelramverk i Rust

---

## Stödda plattformar

 * Windows
 * MacOS
 * Linux
 * HTML5 / WebAssembly
 * Android
 * iOS

---

## Allt inkluderat

 * Immediate mode UI
 * 2D-rendering
 * Audio
 * Ett kommando för att bygga för android

---

## Baserat på Miniquad

Miniquad är ett minimalt grafikabstraktionslager

---

## Designmål

Några av Macroquads designmål:

 * Snabb kompilering
 * Korsplattform
 * Stödjer budgetenheter
 * Hackbar

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

```rust
use macroquad::prelude::*;

#[macroquad::main("Textur")]
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

## Rita

```rust
use macroquad::prelude::*;

#[macroquad::main("Rita")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text("HEJ!", 20.0, 20.0, 30.0, DARKGRAY);
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

## Shapes API lines

 * `draw_circle_lines( ... )`
 * `draw_poly_lines( ... )`
 * `draw_rectangle_lines( ... )`
 * `draw_triangle_lines( ... )`

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

 * `get_char_pressed -> Option<char>`
 * `get_last_key_pressed -> Option<KeyCode>`
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

## Olle Wreede

@ollej
