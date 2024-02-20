use ggez::{
    event::{self, MouseButton},
    input::keyboard::KeyInput,
    glam::*,
    graphics::{self, Color},
    Context, GameResult
};

mod wordle;
use crate::wordle::Wordle;

const WINDOW_WIDTH: f32 = 1080.0;
const WINDOW_HEIGHT: f32 = 800.0;

const BACKGROUND_COLOR: Color = Color::new(0.08, 0.08, 0.08, 1.0);

struct MainState {
    wordle: Wordle,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let wordle = Wordle::new(ctx);
        Ok (MainState {wordle})
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let _ = Wordle::update_wordle(&mut self.wordle, ctx);
        let k = ggez::timer::TimeContext::new();
        println!("{}", k.fps());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, BACKGROUND_COLOR);

        let _ = Wordle::draw_wordle(&mut self.wordle, &mut canvas, ctx);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        self.wordle.detect_click (_ctx, button, x, y);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        self.wordle.detect_typing(ctx, input);
        Ok(())
    }
}


pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("wordle", "az")
        .window_setup(ggez::conf::WindowSetup::default()
        .title("Wordle"))
        .window_mode(ggez::conf::WindowMode::default()
        .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state);
}