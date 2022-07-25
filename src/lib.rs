mod dom_util;
mod vec2d;

use dom_util::*;
use vec2d::Vec2d;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::console::log_1;

#[derive(Clone)]
struct Ball {
    position: Vec2d, // 位置
    direction: Vec2d, // 動く方向
    speed: f64, // 速さ
    size: f64, // 大きさ
    is_inverse: bool, // 反転するか
    color: String, // 色
}

#[derive(Serialize, Deserialize)]
pub struct AppOptions {
    pub canvas_id: String, // canvas属性のid
    pub window_x: Option<u32>, // 画面幅
    pub window_y: Option<u32>, // 画面の高さ
}

#[wasm_bindgen]
pub struct App {
    context: web_sys::CanvasRenderingContext2d, // canvasオブジェクト
    window_x: u32, // 画面幅
    window_y: u32, // 画面高さ
    initial_n_balls: u32, // ボールの数
    balls: Vec<Box<Ball>>, // ボール
}

#[wasm_bindgen]
impl App {
    pub fn new(options: JsValue) -> App {
        let opts: AppOptions = options.into_serde().unwrap();
        App {
            window_x: opts.window_x.unwrap(),
            window_y: opts.window_y.unwrap(),
            context: context2d(&opts.canvas_id),
            initial_n_balls: 10,
            balls: vec![],
        }
    }

    pub fn init(&mut self) {
        // 初期化して、Ballを生成する
        let canvas = self.context.canvas().unwrap();
        canvas.set_width(self.window_x);
        canvas.set_height(self.window_y);
        self.balls = Vec::with_capacity(self.initial_n_balls as usize);

        for _ in 0..self.initial_n_balls {
            let ball = Box::new(self.generate_ball());
            self.balls.push(ball);
        }
    }

    // ボールの生成
    fn generate_ball(&self) -> Ball {
        Ball {
            position: Vec2d {
                x: random_number(0., self.window_x as f64),
                y: random_number(0., self.window_y as f64),
            },
            direction: Vec2d {
                x: random_number(-0.5, 0.5),
                y: random_number(-0.5, 0.5),
            },
            speed: random_number(5., 10.),
            size: random_number(50., 100.0),
            is_inverse: if random_number(- 1., 1.) > 0. {true} else {false},
            color: ball_color()
        }
    }

    pub fn on_animation_frame(&mut self) -> bool {
        self.moves();
        self.render();
        true
    }

    fn moves(&mut self) {
        // direction に従って移動する
        for ball in self.balls.iter_mut() {
            ball.position.x = ball.position.x + ball.direction.x * ball.speed;
            ball.position.y = ball.position.y + ball.direction.y * ball.speed;

            // 境界を超えたら反転する
            if ball.is_inverse {
                if ball.position.x < 0. || ball.position.x > self.window_x as f64 {
                    ball.position.x -= ball.direction.x;
                    ball.direction.x = -ball.direction.x;
                }
                if ball.position.y < 0. || ball.position.y > self.window_y as f64 {
                    ball.position.y -= ball.direction.y;
                    ball.direction.y = -ball.direction.y;
                }
            }
        }
    }

    // 描画
    fn render(&self) {
        self.context.save();
        self.render_bg();
        self.render_balls(&self.balls);
        self.context.restore();
    }

    // 背景の描画
    fn render_bg(&self) {
        self.context.set_fill_style(&JsValue::from(bg_color()));
        self.context.fill_rect(0., 0., self.window_x as f64, self.window_y as f64);
    }

    // ボールの描画
    fn render_balls(&self, balls: &Vec<Box<Ball>>) {
        for (_, ball) in balls.iter().enumerate() {
            self.context.begin_path();
            self.context
                .arc(
                    ball.position.x.into(),
                    ball.position.y.into(),
                    ball.size,
                    0.,
                    std::f64::consts::PI * 2.0,
                )
                .unwrap();
            self.context.set_fill_style(&JsValue::from(&ball.color));
            self.context.fill();
        }
    }
}

#[wasm_bindgen]
pub fn start_animation(app: App) -> Result<(), JsValue> {
    let closure_owner_captured = Rc::new(RefCell::new(None));
    let closure_owner = closure_owner_captured.clone();
    let app_holder_captured = Rc::new(RefCell::new(app));

    *closure_owner.borrow_mut() = Some(Closure::wrap(Box::new(move |_| {
        app_holder_captured.borrow_mut().on_animation_frame();
        request_animation_frame(closure_owner_captured.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));
    request_animation_frame(closure_owner.borrow().as_ref().unwrap());
    Ok(())
}

// アニメーションの更新をリクエスト
fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// ボールの色の生成
fn ball_color() -> String {
    let red =random_number(8., 248.0).to_string();
    let green = random_number(8., 248.0).to_string();
    let blue = random_number(8., 248.0).to_string();
    let opacity = random_number(0.5, 1.0).to_string();
    String::from(format!("rgb({}, {}, {}, {})", red, green, blue, opacity))
}

// 背景色の生成
fn bg_color() -> String {
    String::from("rgb(0, 0, 0, 1)")
}

// 乱数生成
fn random_number(low:f64,high:f64) -> f64{
    rand::thread_rng().gen_range(low, high)
}

// ログ
fn log(s: &String) {
    log_1(&JsValue::from(s));
}