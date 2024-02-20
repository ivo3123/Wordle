pub mod lua_wrapper {
    use rlua::{ Function, Lua, Table, Value };

    pub fn get_five_letter_words() -> Vec<String> {
        let lua = Lua::new();

        let lua_script_content = std::fs::read_to_string("src/wordle/words.lua")
                                        .expect("Error when opening words.lua");
        
        // defined outside the context scope
        let mut result: Vec<String> = Vec::new();

        lua.context(|lua_ctx| {
            lua_ctx.load(&lua_script_content).exec().unwrap();
            let lua_function: Function = lua_ctx.globals().get("get_all_words").unwrap();
            let table: Table = lua_function.call(()).unwrap();

            let mut ctr: i64 = 1;
            
            loop {
                match table.get::<Value, String>(rlua::Value::Integer(ctr)) {
                    Err(_err) => break,
                    Ok(value) => result.push(value),
                }
                ctr += 1;
            }
        });
        result
    }

    pub fn get_random_word() -> [char; crate::wordle::WORDLE_COLS] {
        let lua = Lua::new();

        let lua_script_content = std::fs::read_to_string("src/wordle/words.lua")
                                        .expect("Error when opening words.lua");
        
        // defined outside the context scope
        let mut result: [char; crate::wordle::WORDLE_COLS] = [' '; crate::wordle::WORDLE_COLS];

        lua.context(|lua_ctx| {
            lua_ctx.load(&lua_script_content).exec().unwrap();
            let lua_function: Function = lua_ctx.globals().get("get_random_word").unwrap();
            let str: String = lua_function.call(()).unwrap();
            let str = str.to_uppercase();

            let mut i: usize = 0;
            for ch in str.chars() {
                result[i] = ch;
                i += 1;
            }
        });
        result
    }
}

use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use rand::Rng;

use ggez::{mint::Point2, graphics::{Rect, DrawParam}};
pub use ggez::{
    glam::*,
    graphics::{self, Color, Text, PxScale, TextFragment},
    Context,
    GameResult,
};

#[derive(Copy, Clone)]
#[derive(PartialEq, PartialOrd)]
pub enum State {
    NotFinalized,
    NotInWord,
    IncorrectInWord,
    CorrectInWord,
}

impl State {
    fn get_color_of_upper_letter(state: &State) -> Color {
        return match state {
            State::NotFinalized => UpperLetter::RECT_DARKER_GRAY,
            State::NotInWord => UpperLetter::RECT_BRIGHTER_GRAY,
            State::IncorrectInWord => UpperLetter::RECT_YELLOW,
            State::CorrectInWord => UpperLetter::RECT_GREEN,
        }
    }

    fn get_color_of_lower_letter(state: &State) -> Color {
        return match state {
            State::NotFinalized => LowerLetter::RECT_BRIGHTER_GRAY,
            State::NotInWord => LowerLetter::RECT_DARKER_GRAY,
            State::IncorrectInWord => LowerLetter::RECT_YELLOW,
            State::CorrectInWord => LowerLetter::RECT_GREEN,
        }
    }
}

pub struct UpperLetter {
    rect: graphics::Rect,
    letter: Option<Text>,
    state: State,
}

impl UpperLetter {
    pub const RECT_WIDTH: f32 = 57.6;
    pub const RECT_HEIGHT: f32 = 57.6;
    pub const RECT_DARKER_GRAY: Color = Color::new(0.275, 0.275, 0.275, 1.0);
    pub const RECT_BRIGHTER_GRAY: Color = Color::new(0.35, 0.35, 0.35, 1.0);
    pub const RECT_YELLOW: Color = Color::new(0.6, 0.6, 0.0, 1.0);
    pub const RECT_GREEN: Color = Color::new(0.0, 0.6, 0.0, 1.0);

    pub const LETTER_SIZE: f32 = 53.6;
    pub const LETTER_COLOR: Color = Color::WHITE;

    pub fn new(
        _ctx: &mut Context,
        offset_x: f32,
        offset_y: f32
    ) -> UpperLetter {
        let rect = graphics::Rect::new(offset_x, offset_y, UpperLetter::RECT_WIDTH, UpperLetter::RECT_HEIGHT);
        UpperLetter {
            rect,
            letter: None,
            state: State::NotFinalized,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let color = State::get_color_of_upper_letter(&self.state);
        let mode: graphics::DrawMode = match self.state {
            State::NotInWord => graphics::DrawMode::fill(),
            State::NotFinalized => graphics::DrawMode::stroke(4.0),
            _ => graphics::DrawMode::stroke(3.2),
        };
        let rect: graphics::Mesh = graphics::Mesh::new_rectangle(
            ctx,
            mode,
            self.rect,
            color,
        ).unwrap();

        canvas.draw(&rect, graphics::DrawParam::default());
        
        if self.letter.is_some() {
            let letter_width = self.letter.as_ref().unwrap().measure(ctx).unwrap().x;
            let letter_height = self.letter.as_ref().unwrap().measure(ctx).unwrap().y;
            canvas.draw(self.letter.as_ref().unwrap(), Vec2::new(
                self.rect.x + self.rect.w/2.0 - letter_width/2.0,
                self.rect.y + self.rect.h/2.0 - letter_height/2.0,
            )
            );
        }
        Ok(())
    }

    pub fn set_state(&mut self, _ctx: &mut Context, state: State) {
        self.state = state;
    }
    
    pub fn set_letter(&mut self, _ctx: &mut Context, typed_letter: char) {
        let letter: TextFragment = TextFragment::new(typed_letter.to_string())
                                .color(UpperLetter::LETTER_COLOR)
                                .scale(PxScale::from(UpperLetter::LETTER_SIZE));
        self.letter = Some(Text::new(letter));
    }

    pub fn clear_letter(&mut self) {
        self.letter = None;
    }

    pub fn get_value(&self) -> Option<char> {
        if self.letter.is_none() {
            return None;
        }
        Some(self.letter.as_ref().unwrap().fragments()[0].text.chars().next().unwrap())
    }
}

type ActionWhenClicked = fn (ctx: &mut Context, &mut super::Wordle, value: Option<char>);

#[derive(Clone)]
pub struct LowerLetter {
    rect: graphics::Rect,
    text: Text,
    state: State,
    action_when_clicked: ActionWhenClicked,
}

impl LowerLetter {
    pub const RECT_WIDTH: f32 = 42.8;
    pub const RECT_HEIGHT: f32 = 57.6;
    pub const RECT_DARKER_GRAY: Color = Color::new (0.145, 0.145, 0.145, 1.0);
    pub const RECT_BRIGHTER_GRAY: Color = Color::new (0.455, 0.455, 0.455, 1.0);
    pub const RECT_YELLOW: Color = UpperLetter::RECT_YELLOW;
    pub const RECT_GREEN: Color = UpperLetter::RECT_GREEN;

    pub const LETTER_SIZE: f32 = 27.0;  // for the cases of just a single letter - its size
    pub const STRING_SIZE: f32 = 18.4;  // for the cases of strings (delete, enter) - the size of a single letter of the string
    pub const LETTER_COLOR: Color = Color::WHITE;

    pub fn new(
        _ctx: &mut Context,
        value: &String,
        width: f32,
        height: f32,
        offset_x: f32,
        offset_y: f32,
        letter_size: f32,
        action_when_clicked: ActionWhenClicked
    ) -> LowerLetter {
        let rect = graphics::Rect::new(offset_x, offset_y, width, height);

        let text: TextFragment = TextFragment::new(value)
                                .color(LowerLetter::LETTER_COLOR)
                                .scale(PxScale::from(letter_size));

        LowerLetter {
            rect,
            text: Text::new(text),
            state: State::NotFinalized,
            action_when_clicked,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let rect: graphics::Mesh = graphics::Mesh::new_rectangle (
            ctx,
            graphics::DrawMode::fill(),
            self.rect,
            State::get_color_of_lower_letter(&self.state),
        ).unwrap();
        canvas.draw(&rect, Vec2::new (0.0, 0.0));
        let text_width = self.text.measure(ctx).unwrap().x; 
        let text_height = self.text.measure(ctx).unwrap().y;
        canvas.draw(&self.text, Vec2::new(
            self.rect.x + self.rect.w/2.0 - text_width/2.0,
            self.rect.y + self.rect.h/2.0 - text_height/2.0,
        )
        );
        Ok(())
    }

    pub fn set_state(&mut self, _ctx: &mut Context, state: State) {
        if self.state >= state {
            return;
        }
        self.state = state;
    }

    pub fn point_is_in(&self, x: f32, y: f32) -> bool {
        self.rect.contains(Point2 { x, y })
    }

    pub fn update(&mut self, ctx: &mut Context, wordle: &mut super::Wordle, value: Option<char>) {
        (self.action_when_clicked)(ctx, wordle, value);
    }

    pub fn get_value(&self) -> &String {
        &self.text.fragments()[0].text
    }

    pub fn get_value_char(&self) -> Option<char> {
        let str = self.get_value();
        return if str.len() > 1 {None} else {str.chars().next()}
    }
}








#[derive(PartialEq)]
#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Unkwon,
}

impl Direction {
    fn generate_direction() -> Direction {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1..=4);
        return match random_number {
            1 => Direction::Left,
            2 => Direction::Right,
            3 => Direction::Up,
            4 => Direction::Down,
            _ => Direction::Unkwon,
        }
    }
}

#[derive(Clone)]
pub struct AnimatedArguments {
    direction: Direction,
    change_amount: f32,  // by how much the object is moving; will be multiplied by delta time
    initial_time: Option<Duration>,
    start_moving: Option<Duration>,  
    stop_moving: Option<Duration>,
    lower_bound: Option<f32>,
    upper_bound: Option<f32>,
}

impl AnimatedArguments {
    pub fn for_swipe_animation(ctx: &mut Context, change_amount: f32, start_moving: Duration, stop_moving: Duration) -> AnimatedArguments {
        let initial_time = ctx.time.time_since_start();
        AnimatedArguments {
            direction: Direction::Unkwon,
            change_amount,
            initial_time: Some(initial_time),
            start_moving: Some(start_moving),
            stop_moving: Some(stop_moving),
            lower_bound: None,
            upper_bound: None,
        }
    }

    pub fn for_roll_animation(change_amount: f32, direction: Direction, lower_bound: f32, upper_bound: f32) -> AnimatedArguments {
        AnimatedArguments {
            direction: direction,
            change_amount,
            initial_time: None,
            start_moving: None,
            stop_moving: None,
            lower_bound: Some(lower_bound),
            upper_bound: Some(upper_bound),
        }
    }
}

impl Default for AnimatedArguments {
    fn default() -> AnimatedArguments {
        AnimatedArguments {
            direction: Direction::Unkwon,
            change_amount: 0.0,
            initial_time: None,
            start_moving: None,
            stop_moving: None,
            lower_bound: None,
            upper_bound: None,
        }
    }
}

type AnimationFunction = fn (
    &mut AnimatedBox,
    ctx: &mut Context,
    args: &mut AnimatedArguments, 
);

pub fn swipe_animation(
    obj: &mut AnimatedBox,
    ctx: &mut Context,
    args: &mut AnimatedArguments,
) {
    if obj.is_visible == false {
        return;
    }

    if args.direction == Direction::Unkwon {
        args.initial_time = Some(ctx.time.time_since_start());
        args.direction = Direction::generate_direction();
    }

    if ctx.time.time_since_start() >= args.start_moving.unwrap() + args.initial_time.unwrap() {
        match args.direction {
            Direction::Left => obj.flating_box.x -= args.change_amount * ctx.time.delta().as_secs_f32(),
            Direction::Right => obj.flating_box.x += args.change_amount * ctx.time.delta().as_secs_f32(),
            Direction::Up => obj.flating_box.y -= args.change_amount * ctx.time.delta().as_secs_f32(),
            Direction::Down => obj.flating_box.y += args.change_amount * ctx.time.delta().as_secs_f32(),
            _ => {}
        }
    }

    if ctx.time.time_since_start() >= args.stop_moving.unwrap() + args.initial_time.unwrap() {
        obj.is_visible = false;
        obj.flating_box.x = obj.inicial_position.x;
        obj.flating_box.y = obj.inicial_position.y;
        args.direction = Direction::Unkwon;
    }
}

pub fn roll_animation(
    obj: &mut AnimatedBox,
    ctx: &mut Context,
    args: &mut AnimatedArguments,
) {
    let lower_bound = args.lower_bound.unwrap();  // lower bound is the bound that is seen higer up on the screen but has a lower y value
    let upper_bound = args.upper_bound.unwrap() - obj.flating_box.h;
    match args.direction {
        Direction::Up => {
            obj.flating_box.y -= args.change_amount * ctx.time.delta().as_secs_f32();
            if obj.flating_box.y <= lower_bound {
                args.direction = Direction::Down;
            }
        },
        Direction::Down => {
            obj.flating_box.y += args.change_amount * ctx.time.delta().as_secs_f32();
            if obj.flating_box.y >= upper_bound {
                args.direction = Direction::Up;
            }
        }
        _ => {},
    }
}

pub fn no_animation(
    _obj: &mut AnimatedBox,
    _ctx: &mut Context,
    _args: &mut AnimatedArguments,
) {}

#[derive(Clone)]
pub struct AnimatedBox {
    inicial_position: Point2<f32>,
    flating_box: ggez::graphics::Rect,
    box_color: Color,
    text: Text,
    animation: AnimationFunction,
    pub args: AnimatedArguments,
    pub is_visible: bool,
}

impl AnimatedBox {
    pub fn new(
        offset_x: f32,
        offset_y: f32,
        width: f32,
        height: f32,
        box_color: Color,
        text: &String,
        text_size: f32,
        text_color: Color,
        animation_function: AnimationFunction,
        args: AnimatedArguments,
    ) -> AnimatedBox {
        let rect: Rect = Rect::new(offset_x, offset_y, width, height);

        let text: TextFragment = TextFragment::new(text)
                                .color(text_color)
                                .scale(PxScale::from(text_size));
        AnimatedBox {
            inicial_position: Point2 {x: offset_x, y: offset_y},
            flating_box: rect,
            box_color,
            text: Text::new(text),
            animation: animation_function,
            is_visible: false,
            args,
        }
    }

    pub fn update_animated_box(&mut self, ctx: &mut Context, args: &mut AnimatedArguments) -> GameResult {
        if self.is_on_screen() {
            (self.animation)(self, ctx, args);
        }
        Ok(())
    }

    pub fn is_on_screen(&self) -> bool {
        self.is_visible == true
    }

    pub fn put_on_screen(&mut self) {
        self.is_visible = true;
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
    ) -> GameResult {
        let rect: graphics::Mesh = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.flating_box,
            5.0,
            self.box_color,
        ).unwrap();

        let _ = canvas.draw(&rect, graphics::DrawParam::default());

        let letter_width = self.text.measure(ctx).unwrap().x;
        let letter_height = self.text.measure(ctx).unwrap().y;
        canvas.draw(&self.text, Vec2::new(
            self.flating_box.x + self.flating_box.w/2.0 - letter_width/2.0,
            self.flating_box.y + self.flating_box.h/2.0 - letter_height/2.0,
        )
        );
        Ok(())
    }

    pub fn point_is_in(&self, x: f32, y: f32) -> bool {
        self.flating_box.contains(Point2{ x, y })
    }
}









pub struct Statistics {
    games_won_by_first_try: u32,
    games_won_by_second_try: u32,
    games_won_by_third_try: u32,
    games_won_by_fourth_try: u32,
    games_won_by_fifth_try: u32,
    games_won_by_sixth_try: u32,
    games_lost: u32,
    is_being_shown: bool,
    hit_box_of_closing_button: graphics::Rect,
    last_guessed_by_attempt: Option<u8>,
}

impl Statistics {
    pub fn new() -> Statistics {
        let file = File::open("src/wordle/stats");
        if file.is_err() {
            panic!("Could not open the stats file!");
        }
        let mut file = file.unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        let mut numbers = contents
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap());

        drop(file);

        Statistics {
            games_won_by_first_try: numbers.next().unwrap(),
            games_won_by_second_try: numbers.next().unwrap(),
            games_won_by_third_try: numbers.next().unwrap(),
            games_won_by_fourth_try: numbers.next().unwrap(),
            games_won_by_fifth_try: numbers.next().unwrap(),
            games_won_by_sixth_try: numbers.next().unwrap(),
            games_lost: numbers.next().unwrap(),
            is_being_shown: false,
            hit_box_of_closing_button: Rect::default(),
            last_guessed_by_attempt: None,
        }
    }

    fn get_games_played(&self) -> u32 {
        self.games_won_by_first_try +
        self.games_won_by_second_try +
        self.games_won_by_third_try +
        self.games_won_by_fourth_try +
        self.games_won_by_fifth_try +
        self.games_won_by_sixth_try +
        self.games_lost
    }

    fn get_win_rate(&self) -> f32 {
        let res: f32 = ((self.get_games_played() - self.games_lost) as f32 / (self.get_games_played()) as f32) * 100.0;
        res
    }

    fn get_most_wins(&self) -> u32 {
        let wins = [
            self.games_won_by_first_try,
            self.games_won_by_second_try,
            self.games_won_by_third_try,
            self.games_won_by_fourth_try,
            self.games_won_by_fifth_try,
            self.games_won_by_sixth_try,
        ];
        let max_number = *wins.iter().max().unwrap();
        max_number
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // the black background
        let bigger_rect: Rect = Rect::new(
            0.0,
            0.0,
            super::super::WINDOW_WIDTH,
            super::super::WINDOW_HEIGHT
        );
        let _f = bigger_rect.center();
        let width = super::super::WINDOW_WIDTH as f32 * (0.62);
        let height = super::super::WINDOW_HEIGHT as f32 * (0.62);

        // the actual background of the statistics
        let smaller_rect = Rect::new(
            (super::super::WINDOW_WIDTH - width) / 2.0,
            (super::super::WINDOW_HEIGHT - height) / 2.0,
            width,
            height
        );
        let my_color = Color::new(0.15, 0.0, 0.23, 1.0);
        let top_most_row_height: f32 = 90.0;
        let second_row_height: f32 = 80.0;
        let background: graphics::Mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            smaller_rect,
            my_color,
        ).unwrap();
        let _ = canvas.draw(&background, graphics::DrawParam::default());

        // the number of games played
        let games_played = Text::new(
            TextFragment::new(Self::get_games_played(&self).to_string())
            .color(Color::WHITE)
            .scale(PxScale::from(60.0))
        );
        let games_played_text_width = games_played.measure(ctx).unwrap().x;
        let games_played_text_height = games_played.measure(ctx).unwrap().y;

        // the number that represents the win rate
        let win_rate = Text::new(
            TextFragment::new(Self::get_win_rate(&self).round().to_string() + "%")
            .color(Color::WHITE)
            .scale(PxScale::from(60.0))
        );
        let win_rate_text_width = win_rate.measure(ctx).unwrap().x; 
        let win_rate_text_height = win_rate.measure(ctx).unwrap().y;

        let slight_offset_y: f32 = 12.0;
        canvas.draw(&games_played, Vec2::new(
            smaller_rect.x + (smaller_rect.w / 2.0 - games_played_text_width) / 2.0,
            smaller_rect.y + (top_most_row_height - games_played_text_height) / 2.0 + slight_offset_y,
        ));
        canvas.draw(&win_rate, Vec2::new(
            smaller_rect.x + (smaller_rect.w * (3.0 / 2.0) - win_rate_text_width) / 2.0,
            smaller_rect.y + (top_most_row_height - win_rate_text_height) / 2.0 + slight_offset_y,
        ));

        // the actual text "GAMES PLAYED"
        let games_played = Text::new(
            TextFragment::new(String::from("GAMES PLAYED"))
            .color(Color::WHITE)
            .scale(PxScale::from(28.0))
        );
        let games_played_text_width = games_played.measure(ctx).unwrap().x;
        canvas.draw(&games_played, Vec2::new(
            smaller_rect.x + (smaller_rect.w / 2.0 - games_played_text_width) / 2.0,
            smaller_rect.y + top_most_row_height,
        ));

        // the actual text 'WIN RATE"
        let win_rate = Text::new(
            TextFragment::new(String::from("WIN RATE"))
            .color(Color::WHITE)
            .scale(PxScale::from(28.0))
        );
        let win_rate_text_width = win_rate.measure(ctx).unwrap().x;
        canvas.draw(&win_rate, Vec2::new(
            smaller_rect.x + (smaller_rect.w * (3.0/2.0) - win_rate_text_width) / 2.0,
            smaller_rect.y + top_most_row_height,
        ));

        // the actual text "GUESS DISTRIBUTION"
        let guess_distribution = Text::new(
            TextFragment::new(String::from("GUESS DISTRIBUTION"))
            .color(Color::WHITE)
            .scale(PxScale::from(38.0))
        );
        let text_width = guess_distribution.measure(ctx).unwrap().x;
        let text_height = guess_distribution.measure(ctx).unwrap().y;
        canvas.draw(&guess_distribution, Vec2::new(
            bigger_rect.w / 2.0 - text_width / 2.0,
            smaller_rect.y + top_most_row_height + second_row_height - text_height / 2.0,
        ));

        // defining variables (constants) to help draw the the ordinal number, the rectangles and the number of games won by a certain amount of guesses
        let rect_slice_height: f32 = 34.5;
        let separation_between_slices: f32 = 6.2;
        let inicial_dy: f32 = smaller_rect.y + top_most_row_height + second_row_height +
            (smaller_rect.h - top_most_row_height - second_row_height - rect_slice_height*super::WORDLE_ROWS as f32 - separation_between_slices*(super::WORDLE_ROWS - 1) as f32) / 2.0;
        let dist_from_left_or_right: f32 = 50.0;
        let width_of_smallest_rect: f32 = 30.0;
        let width_of_biggest_rect: f32 = smaller_rect.w - dist_from_left_or_right*2.0;
        for i in 0..super::WORDLE_ROWS {
            let curr_number_of_wins = match i {
                0 => self.games_won_by_first_try,
                1 => self.games_won_by_second_try,
                2 => self.games_won_by_third_try,
                3 => self.games_won_by_fourth_try,
                4 => self.games_won_by_fifth_try,
                5 => self.games_won_by_sixth_try,
                _ => panic!("Number of rows do not match the statistics!"),
            };

            let curr_width = (curr_number_of_wins as f32 / self.get_most_wins() as f32) * width_of_biggest_rect;
            let curr_width: f32 = if curr_width <= width_of_smallest_rect {width_of_smallest_rect} else {curr_width};

            let rect_i = Rect::new(
                smaller_rect.x + dist_from_left_or_right,
                inicial_dy + rect_slice_height * i as f32 + separation_between_slices * i as f32,
                curr_width,
                rect_slice_height,
            );

            let color: Color = if self.last_guessed_by_attempt.is_some() && self.last_guessed_by_attempt.unwrap() == i as u8
             {super::LowerLetter::RECT_GREEN} 
                else {Color::BLUE};
            let first_slice = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect_i,
                color,
            ).unwrap();

            canvas.draw(&first_slice, DrawParam::default());

            let number_of_wins_as_text = Text::new(
                TextFragment::new(curr_number_of_wins.to_string())
                .color(Color::WHITE)
                .scale(PxScale::from(28.0))
            );

            let letter_width = number_of_wins_as_text.measure(ctx).unwrap().x;
            let letter_height = number_of_wins_as_text.measure(ctx).unwrap().y;

            let offset_width_from_right: f32 = (width_of_smallest_rect-letter_width) / 2.0;

            canvas.draw(&number_of_wins_as_text, Vec2::new(
                smaller_rect.x + dist_from_left_or_right + curr_width - offset_width_from_right - letter_width,
                inicial_dy + rect_slice_height * i as f32 + separation_between_slices * i as f32 + (rect_slice_height - letter_height).abs() / 2.0,
            ));

            let curr_number = Text::new(
                TextFragment::new((i + 1).to_string() + ".")
                .color(Color::WHITE)
                .scale(PxScale::from(28.0))
            );

            let letter_width = curr_number.measure(ctx).unwrap().x;
            let letter_height = curr_number.measure(ctx).unwrap().y;

            canvas.draw(&curr_number, Vec2::new(
                smaller_rect.x + (dist_from_left_or_right - letter_width) / 2.0,
                inicial_dy + rect_slice_height * i as f32 + separation_between_slices * i as f32 + (rect_slice_height - letter_height).abs() / 2.0,
            ));

            let closing_button = Text::new(
                TextFragment::new(String::from("x"))
                .color(Color::RED)
                .scale(PxScale::from(28.0))
            );

            let letter_width = closing_button.measure(ctx).unwrap().x;
            let letter_height = closing_button.measure(ctx).unwrap().y;

            canvas.draw(&closing_button, Vec2::new(
                smaller_rect.x + smaller_rect.w - letter_width - 5.0,
                smaller_rect.y + 5.0,
            ));
            let hit_box_of_closing_button: graphics::Rect = Rect::new(
                smaller_rect.x + smaller_rect.w - letter_width - 5.0,
                smaller_rect.y + 5.0,
                letter_width,
                letter_height,  
            );
            self.hit_box_of_closing_button = hit_box_of_closing_button;
        }

        Ok(())
    }

    pub fn is_on_screen(&self) -> bool {
        self.is_being_shown == true
    }

    pub fn put_on_screen(&mut self) {
        self.is_being_shown = true;
    }

    pub fn remove_from_screen(&mut self) {
        self.is_being_shown = false;
    }

    pub fn quit_stats_screen_request(&self, x: f32, y: f32) -> bool {
        self.hit_box_of_closing_button.contains(Point2 {x, y})
    }

    // if None |> Lost else |> number of guesses used
    pub fn update_stats(&mut self, input: Option<u32>) {
        if input.is_none() {
            self.games_lost += 1;
        }
        else {
            match input.unwrap() {
                1 => self.games_won_by_first_try += 1,
                2 => self.games_won_by_second_try += 1,
                3 => self.games_won_by_third_try += 1,
                4 => self.games_won_by_fourth_try += 1,
                5 => self.games_won_by_fifth_try += 1,
                6 => self.games_won_by_sixth_try += 1,
                _ => panic!("Input to put into statistics was not valid!"),
            }
            self.last_guessed_by_attempt = Some(input.unwrap() as u8 - 1);
        }

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("src/wordle/stats").unwrap();
    
        let numbers = vec![
            self.games_won_by_first_try,
            self.games_won_by_second_try,
            self.games_won_by_third_try,
            self.games_won_by_fourth_try,
            self.games_won_by_fifth_try,
            self.games_won_by_sixth_try,
            self.games_lost,
        ];
        let numbers_str = numbers.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        
        let _ = file.write(numbers_str.as_bytes());
    }
}