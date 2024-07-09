#![windows_subsystem = "windows"]

use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 450.0;

struct Bead {
    position: Vector2,
    color: Color,
    size: f32,
}

impl Bead {
    fn draw_bead(&self, d: &mut RaylibDrawHandle) {
        let v1 = Vector2::new(self.position.x - self.size, self.position.y);
        let v2 = Vector2::new(self.position.x, self.position.y + self.size);
        let v3 = Vector2::new(self.position.x, self.position.y);
        d.draw_triangle(v1, v2, v3, self.color);

        let v1 = Vector2::new(self.position.x + self.size, self.position.y);
        let v2 = Vector2::new(self.position.x + self.size, self.position.y + self.size);
        let v3 = Vector2::new(self.position.x + 2.0 * self.size, self.position.y);
        d.draw_triangle(v1, v2, v3, self.color);

        let v1 = Vector2::new(self.position.x, self.position.y - self.size);
        let v2 = Vector2::new(self.size, self.size * 2.0);
        //        d.draw_rectangle_v(v1, Vector2{x: self.size, y: self.size*2.0}, self.color);
        d.draw_rectangle_v(v1, v2, self.color);

        let v1 = Vector2::new(self.position.x - self.size, self.position.y);
        let v2 = Vector2::new(self.position.x, self.position.y);
        let v3 = Vector2::new(self.position.x, self.position.y - self.size);
        d.draw_triangle(v1, v2, v3, self.color);

        let v1 = Vector2::new(self.position.x + self.size, self.position.y - self.size);
        let v2 = Vector2::new(self.position.x + self.size, self.position.y);
        let v3 = Vector2::new(self.position.x + 2.0 * self.size, self.position.y);
        d.draw_triangle(v1, v2, v3, self.color);
    }
}

fn main() {
    let mut value = "0000000".to_string();
    let mut position = 0;
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Soroban")
        .vsync()
        .build();

    // rl.set_target_fps(60);

    let mut b = Vec::with_capacity(7);
    for i in 1..8 {
        for v in 1..5 {
            b.push(Bead {
                position: Vector2::new(
                    (i as f32 * WIDTH / 8.0) - 6.0,
                    30.0 + HEIGHT - v as f32 * 62.0,
                ),
                color: Color::ORANGE,
                size: 30.0,
            });
        }

        for v in 1..2 {
            b.push(Bead {
                position: Vector2::new((i as f32 * WIDTH / 8.0) - 6.0, v as f32 * 62.0 - 30.0),
                color: Color::ORANGE,
                size: 30.0,
            });
        }
    }

    for v in 0..5 {
        b[v + position * 5].color = Color::BLUE;
    }

    while !rl.window_should_close() {
        for i in 0..b.len() {
            if rl.is_key_pressed(KEY_UP) && rl.get_key_pressed() != None {
                let z = &value[position..position + 1];
                let mut z = (z.to_string()).parse::<i32>().unwrap();
                z += 1;
                z = z % 5;

                for v in 0..z {
                    b[(3 - v as i32 + position as i32 * 5) as usize].position.y =
                        (170 + v * 62) as f32;
                }
                if z == 0 {
                    for v in 0..4 {
                        b[(v + position as i32 * 5) as usize].position.y = (418 - v * 62) as f32;
                    }
                }

                if b[4 + position * 5].position.y == 92.0 {
                    z += 5;
                }

                let z = z.to_string();
                value.replace_range(position..position + 1, &z);
            }
            if rl.is_key_pressed(KEY_DOWN) && rl.get_key_pressed() != None {
                let z = &value[position..position + 1];
                let mut z = (z.to_string()).parse::<i32>().unwrap();
                z += 5;
                z = z % 10;
                match z {
                    5..=9 => b[4 + position * 5].position.y = 92.0,
                    _ => b[4 + position * 5].position.y = 30.0,
                }

                let z = z.to_string();
                value.replace_range(position..position + 1, &z);
            }
            if rl.is_key_pressed(KEY_LEFT) && rl.get_key_pressed() != None {
                b[i].position.x -= 1.0;
                if (position as i32) > 0 {
                    position -= 1
                }

                for v in 0..5 {
                    b[v + position * 5].color = Color::BLUE;
                    b[v + (position + 1) * 5].color = Color::ORANGE;
                }
            }
            if rl.is_key_pressed(KEY_RIGHT) && rl.get_key_pressed() != None {
                b[i].position.x += 1.0;
                if position < 6 {
                    position += 1
                }
                for v in 0..5 {
                    b[v + position * 5].color = Color::BLUE;
                    b[v + (position - 1) * 5].color = Color::ORANGE;
                }
            }
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for i in 1..10 {
            let v1 = Vector2::new(i as f32 * WIDTH / 8.0, 0.0);
            let v2 = Vector2::new(15.0, HEIGHT as f32);
            d.draw_rectangle_v(v1, v2, Color::BROWN);
        }
        d.draw_rectangle_v(
            Vector2 {
                x: 0.0,
                y: (HEIGHT / 2.0) - 99.0,
            },
            Vector2 { x: WIDTH, y: 11.0 },
            Color::WHITE,
        );

        for i in 0..b.len() {
            b[i].draw_bead(&mut d);
        }
        d.draw_text(&value, 10, 10, 48, Color::LIME);
    }
}
