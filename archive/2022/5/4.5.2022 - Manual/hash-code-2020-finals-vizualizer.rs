use std::cmp::{max, min};

use crate::{Operation, Point, Solution, TestInfo};

use skulpin::{
    app::AppBuilder,
    skia_bindings::SkPaint,
    skia_safe::{self, colors, paint, Canvas, Color, Color4f, Handle, Paint, Rect},
    LogicalSize,
};

use skulpin::skia_safe::matrix::ScaleToFit;
use skulpin_app_winit::{AppHandler, InputState, MouseButton, MouseScrollDelta};

use skulpin::skia_safe::Point as Point2D;

const COORD_MULT: f32 = 0.01;

pub fn visualize(sol: Solution, test_name: &str, t: TestInfo) {
    dbg!("visualize!");

    const WINDOW_SIZE: u32 = 1300;
    AppBuilder::new()
        .inner_size(LogicalSize::new(WINDOW_SIZE, WINDOW_SIZE))
        .window_title(format!(
            "Google HashCode 2022: {}, score = {}",
            test_name, sol.score
        ))
        .run(App::new(t, sol));
}

struct App {
    t: TestInfo,
    center: Point2D,
    window_size: f32,
    sol: Solution,
    last_mouse_drag_sum: Option<Point2D>,
}

impl App {
    pub fn new(t: TestInfo, sol: Solution) -> Self {
        let mut max_coord = 1.0;
        for g in t.gifts.iter() {
            for &c in [g.p.x, g.p.y].iter() {
                let c = c as f32;
                if c > max_coord {
                    max_coord = c;
                }
            }
        }

        Self {
            t,
            center: Point2D::new(0.0, 0.0),
            window_size: max_coord * 1.1 * COORD_MULT,
            sol,
            last_mouse_drag_sum: None,
        }
    }

    // real_x = center.x + `coef` * (pixel_x - w/2)
    pub fn calc_coef(&self, input_state: &InputState) -> f32 {
        let window_size = input_state.window_size();
        let smallest_size = (min(window_size.width, window_size.height) as f32) / 2.0;
        self.window_size / smallest_size
    }

    fn draw_gifts(&self, canvas: &mut Canvas) {
        for op in self.sol.ops.iter() {
            if let Operation::DeliverGift(gift_id) = op {
                draw_filled_circle(
                    canvas,
                    self.t.gifts[*gift_id].p,
                    (self.t.range as f32) * COORD_MULT,
                    colors::BLUE,
                );
            }
        }
    }

    fn draw_ops(&self, canvas: &mut Canvas, input_state: &InputState) {
        self.draw_gifts(canvas);

        let coef = self.calc_coef(input_state);

        let mut v = Point::ZERO;
        let mut pos = Point::ZERO;
        for op in self.sol.ops.iter() {
            match op {
                crate::Operation::Accelerate(p) => {
                    v += *p;
                    draw_segment(canvas, pos, pos + *p, colors::MAGENTA);
                }
                crate::Operation::Float(cnt) => {
                    let radius = 3.0 * coef;
                    draw_filled_circle(canvas, pos, radius, colors::RED);
                    let vsum = Point::new(v.x * *cnt, v.y * *cnt);
                    draw_segment(canvas, pos, pos + vsum, colors::RED);
                    pos += vsum;
                }
                crate::Operation::LoadCarrots(_) => {}
                crate::Operation::LoadGift(_) => {}
                crate::Operation::DeliverGift(_) => {}
            }
        }
    }

    fn convert_to_real_pos(&self, input_state: &InputState, p: Point2D) -> Point2D {
        let coef = self.calc_coef(input_state);
        let dx = p.x - (input_state.window_size().width / 2) as f32;
        let dy = p.y - (input_state.window_size().height / 2) as f32;
        Point2D::new(self.center.x + coef * dx, self.center.y + coef * dy)
    }
}

fn conv_point(p: Point) -> Point2D {
    Point2D::new((p.x as f32) * COORD_MULT, (p.y as f32) * COORD_MULT)
}

fn get_paint(color: Color4f) -> Handle<SkPaint> {
    let mut paint = Paint::new(color, None);
    paint.set_anti_alias(true);
    paint.set_style(paint::Style::Stroke);
    paint
}

fn get_paint_fill(color: Color4f) -> Handle<SkPaint> {
    let mut paint = Paint::new(color, None);
    paint.set_anti_alias(true);
    paint.set_style(paint::Style::StrokeAndFill);
    // paint.set_stroke_width(2.0);
    paint
}

fn get_paint_text(color: Color4f) -> Handle<SkPaint> {
    let mut paint = Paint::new(color, None);
    paint.set_anti_alias(true);
    paint
}

fn draw_circle(canvas: &mut Canvas, center: Point, radius: f32, color: Color4f) {
    canvas.draw_circle(conv_point(center), radius, &get_paint(color));
}

fn draw_filled_circle(canvas: &mut Canvas, center: Point, radius: f32, color: Color4f) {
    canvas.draw_circle(conv_point(center), radius, &get_paint_fill(color));
}

fn draw_segment(canvas: &mut Canvas, from: Point, to: Point, color: Color4f) {
    canvas.draw_line(conv_point(from), conv_point(to), &get_paint(color));
}

fn draw_text(canvas: &mut Canvas, str: impl AsRef<str>, p: impl Into<Point2D>, font_size: f32) {
    let mut font = skia_safe::Font::default();
    font.set_size(font_size);
    canvas.draw_str(str, p, &font, &get_paint_text(colors::BLACK));
}

const WINDOW_SCROLL_MUL: f32 = 0.96;

impl AppHandler for App {
    fn update(&mut self, update_args: skulpin_app_winit::AppUpdateArgs) {
        let input_state = update_args.input_state;

        let coef = self.calc_coef(&input_state);

        if let MouseScrollDelta::LineDelta(_dx, dy) = input_state.mouse_wheel_delta() {
            if dy > 0.0 {
                // zoom in
                self.window_size *= WINDOW_SCROLL_MUL;
            } else if dy < 0.0 {
                self.window_size /= WINDOW_SCROLL_MUL;
            }

            let mouse_pos = input_state.mouse_position();

            let pixel_x = mouse_pos.x as f32 - (input_state.window_size().width / 2) as f32;
            let pixel_y = mouse_pos.y as f32 - (input_state.window_size().height / 2) as f32;

            let real_x = self.center.x + coef * pixel_x;
            let real_y = self.center.y + coef * pixel_y;

            let ncoef = self.calc_coef(input_state);
            self.center.x = real_x - ncoef * pixel_x;
            self.center.y = real_y - ncoef * pixel_y;
        }

        if let Some(mouse_drag_state) = input_state.mouse_drag_in_progress(MouseButton::Left) {
            let old = self.last_mouse_drag_sum.unwrap_or(Point2D::new(0.0, 0.0));
            let new = mouse_drag_state.accumulated_frame_delta;
            let new = Point2D::new(new.x as f32, new.y as f32);
            let delta = Point2D::new(new.x - old.x, new.y - old.y);
            self.last_mouse_drag_sum = Some(new);
            self.center.x -= (delta.x as f32) * coef;
            self.center.y -= (delta.y as f32) * coef;
        } else {
            self.last_mouse_drag_sum = None;
        }
    }

    fn draw(&mut self, draw_args: skulpin_app_winit::AppDrawArgs) {
        let canvas = draw_args.canvas;
        let input_state = draw_args.input_state;
        let coordinate_system = draw_args.coordinate_system_helper;

        canvas.clear(Color::WHITE);

        let visible_range = Rect::new(
            self.center.x - self.window_size,
            self.center.y - self.window_size,
            self.center.x + self.window_size,
            self.center.y + self.window_size,
        );
        coordinate_system
            .use_visible_range(canvas, visible_range, ScaleToFit::Center)
            .expect("Can't set coordiante system");

        let mut paint = Paint::new(colors::BLUE, None);
        paint.set_anti_alias(true);
        paint.set_style(paint::Style::Stroke);
        paint.set_stroke_width(2.0);

        for gift in self.t.gifts.iter() {
            draw_circle(
                canvas,
                gift.p,
                (max(1, self.t.range) as f32) * COORD_MULT,
                colors::BLUE,
            );
        }

        self.draw_ops(canvas, input_state);

        // draw_text(
        //     canvas,
        //     format!("score: {}", self.sol.score),
        //     self.convert_to_real_pos(input_state, Point2D::new(100.0, 50.0)),
        //     self.calc_coef(input_state) * 20.0,
        // );
    }

    fn fatal_error(&mut self, error: &skulpin_app_winit::AppError) {
        eprintln!("error: {:?}", error);
    }
}
