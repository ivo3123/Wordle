

use std::time::Duration;

use ggez::{
    event::MouseButton,
    input::keyboard::{KeyCode, KeyInput},
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

mod utility;
use self::utility::{UpperLetter, LowerLetter, lua_wrapper, AnimatedBox, swipe_animation, roll_animation, no_animation, AnimatedArguments, Statistics};

const WORDLE_ROWS: usize = 6;
const WORDLE_COLS: usize = 5;

const WORDLE_LETTERS_COUNT: usize = 26;


#[derive(PartialEq)]
enum GameState {
    NotOver,
    Won,
    Lost,
}

pub struct Wordle {
    game_board: [[UpperLetter; WORDLE_COLS]; WORDLE_ROWS],
    used_letters: [LowerLetter; WORDLE_LETTERS_COUNT],
    enter_button: LowerLetter,
    delete_button: LowerLetter,
    answer: [char; WORDLE_COLS],
    curr_letter: (usize, usize),
    game_state: GameState,
    you_won_box: AnimatedBox,
    shown_answer_box: AnimatedBox,
    invalid_word: AnimatedBox,
    replay_button: AnimatedBox,
    stats: Statistics,
    see_stats_button: AnimatedBox,
}

impl Wordle {
    const SEPARATION_BETWEEN_UPPER_LETTERS: f32 = 6.2;
    const SEPARATION_BETWEEN_LOWEER_LETTERS: f32 = 6.2;
    const DIST_TO_UPPER_BLOCK: f32 = 55.0;
    const DIST_TO_LOWER_BLOCK: f32 = 575.0;

    // gets the y coord of the top of the upper block
    fn dist_to_top_of_upper_block() -> f32 {
        Wordle::DIST_TO_UPPER_BLOCK
    }

    // gets the y coord of the bottom of the upper block
    fn dist_to_bottom_of_upper_block() -> f32 {
        Self::dist_to_top_of_upper_block() + Self::get_height_of_upper_block()
    }

    // gets the x coord of the left of the upper block
    fn dist_to_left_of_upper_block() -> f32 {
        (super::WINDOW_WIDTH - Self::get_width_of_upper_block()) / 2.0
    }

    // gets the x coord of the right of the upper block
    fn dist_to_right_of_upper_block() -> f32 {
        Self::dist_to_left_of_upper_block() + Self::get_width_of_upper_block()
    }

    // gets the y coord of the top of the lower block
    fn dist_to_top_of_lower_block() -> f32 {
        Wordle::DIST_TO_LOWER_BLOCK
    }

    // gets the x coord of the left of the lower block (the left most x coord)
    fn dist_to_left_of_lower_block() -> f32 {
        (super::WINDOW_WIDTH - (10.0*LowerLetter::RECT_WIDTH + 9.0*Wordle::SEPARATION_BETWEEN_LOWEER_LETTERS)) / 2.0
    }

    fn get_width_of_upper_block() -> f32 {
        UpperLetter::RECT_WIDTH*WORDLE_COLS as f32 + Wordle::SEPARATION_BETWEEN_UPPER_LETTERS*(WORDLE_COLS - 1) as f32
    }

    fn get_height_of_upper_block() -> f32 {
        UpperLetter::RECT_HEIGHT*WORDLE_ROWS as f32 + Wordle::SEPARATION_BETWEEN_UPPER_LETTERS*(WORDLE_ROWS - 1) as f32
    }

    pub fn new(ctx: &mut Context) -> Self {
        let mut offsets: [(f32, f32); WORDLE_COLS*WORDLE_ROWS] = [(0.0, 0.0); WORDLE_COLS*WORDLE_ROWS];
        let mut offset_x: f32 = Self::dist_to_left_of_upper_block();
        let mut offset_y: f32 = Self::dist_to_top_of_upper_block();
        for i in 0..WORDLE_ROWS*WORDLE_COLS {
            if i % 5 == 0 && i != 0 {
                offset_y += Wordle::SEPARATION_BETWEEN_UPPER_LETTERS + UpperLetter::RECT_HEIGHT;
                offset_x = Self::dist_to_left_of_upper_block();
            }
            offsets[i].0 = offset_x;
            offsets[i].1 = offset_y;
            offset_x += Wordle::SEPARATION_BETWEEN_UPPER_LETTERS + UpperLetter::RECT_WIDTH;
        }
        let letters: [[UpperLetter; WORDLE_COLS]; WORDLE_ROWS] = [
            [
                UpperLetter::new(ctx, offsets[0].0, offsets[0].1),
                UpperLetter::new(ctx, offsets[1].0, offsets[1].1),
                UpperLetter::new(ctx, offsets[2].0, offsets[2].1),
                UpperLetter::new(ctx, offsets[3].0, offsets[3].1),
                UpperLetter::new(ctx, offsets[4].0, offsets[4].1)
            ],
            [
                UpperLetter::new(ctx, offsets[5].0, offsets[5].1),
                UpperLetter::new(ctx, offsets[6].0, offsets[6].1),
                UpperLetter::new(ctx, offsets[7].0, offsets[7].1),
                UpperLetter::new(ctx, offsets[8].0, offsets[8].1),
                UpperLetter::new(ctx, offsets[9].0, offsets[9].1)
            ],
            [
                UpperLetter::new(ctx, offsets[10].0, offsets[10].1),
                UpperLetter::new(ctx, offsets[11].0, offsets[11].1),
                UpperLetter::new(ctx, offsets[12].0, offsets[12].1),
                UpperLetter::new(ctx, offsets[13].0, offsets[13].1),
                UpperLetter::new(ctx, offsets[14].0, offsets[14].1)
            ],
            [
                UpperLetter::new(ctx, offsets[15].0, offsets[15].1),
                UpperLetter::new(ctx, offsets[16].0, offsets[16].1),
                UpperLetter::new(ctx, offsets[17].0, offsets[17].1),
                UpperLetter::new(ctx, offsets[18].0, offsets[18].1),
                UpperLetter::new(ctx, offsets[19].0, offsets[19].1)
            ],
            [
                UpperLetter::new(ctx, offsets[20].0, offsets[20].1),
                UpperLetter::new(ctx, offsets[21].0, offsets[21].1),
                UpperLetter::new(ctx, offsets[22].0, offsets[22].1),
                UpperLetter::new(ctx, offsets[23].0, offsets[23].1),
                UpperLetter::new(ctx, offsets[24].0, offsets[24].1)
            ],
            [
                UpperLetter::new(ctx, offsets[25].0, offsets[25].1),
                UpperLetter::new(ctx, offsets[26].0, offsets[26].1),
                UpperLetter::new(ctx, offsets[27].0, offsets[27].1),
                UpperLetter::new(ctx, offsets[28].0, offsets[28].1),
                UpperLetter::new(ctx, offsets[29].0, offsets[29].1)
            ]
        ];

        let mut offsets: [(f32, f32); WORDLE_LETTERS_COUNT] = [(0.0, 0.0); WORDLE_LETTERS_COUNT];

        let initial_dx: f32 = Self::dist_to_left_of_lower_block();
        let initial_dy: f32 = Self::dist_to_top_of_lower_block();

        let mut changing_dx: f32 = initial_dx;
        let mut changing_dy: f32 = initial_dy;

        for i in 0..WORDLE_LETTERS_COUNT {
            offsets[i].0 = changing_dx;
            offsets[i].1 = changing_dy;
            changing_dx += LowerLetter::RECT_WIDTH + Wordle::SEPARATION_BETWEEN_LOWEER_LETTERS;
            if i == 9 {
                changing_dx = initial_dx + LowerLetter::RECT_WIDTH/2.0;
                changing_dy += LowerLetter::RECT_HEIGHT + Wordle::SEPARATION_BETWEEN_LOWEER_LETTERS;
            }
            else if i == 18 {
                changing_dx = initial_dx + 3.0*LowerLetter::RECT_WIDTH/2.0 + Wordle::SEPARATION_BETWEEN_LOWEER_LETTERS;
                changing_dy += LowerLetter::RECT_HEIGHT + Wordle::SEPARATION_BETWEEN_LOWEER_LETTERS;
            }
        }

        let used_letters: [LowerLetter; WORDLE_LETTERS_COUNT] = [
            LowerLetter::new(
                ctx,
                &'Q'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[0].0,
                offsets[0].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'W'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[1].0,
                offsets[1].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'E'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[2].0,
                offsets[2].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'R'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[3].0,
                offsets[3].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'T'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[4].0,
                offsets[4].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'Y'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[5].0,
                offsets[5].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'U'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[6].0,
                offsets[6].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'I'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[7].0,
                offsets[7].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'O'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[8].0,
                offsets[8].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'P'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[9].0,
                offsets[9].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'A'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[10].0,
                offsets[10].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'S'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[11].0,
                offsets[11].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'D'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[12].0,
                offsets[12].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'F'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[13].0,
                offsets[13].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'G'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[14].0,
                offsets[14].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'H'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[15].0,
                offsets[15].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'J'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[16].0,
                offsets[16].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'K'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[17].0,
                offsets[17].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'L'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[18].0,
                offsets[18].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'Z'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[19].0,
                offsets[19].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'X'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[20].0,
                offsets[20].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'C'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[21].0,
                offsets[21].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'V'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[22].0,
                offsets[22].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'B'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[23].0,
                offsets[23].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'N'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[24].0,
                offsets[24].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
            LowerLetter::new(
                ctx,
                &'M'.to_string(),
                LowerLetter::RECT_WIDTH,
                LowerLetter::RECT_HEIGHT,
                offsets[25].0,
                offsets[25].1,
                LowerLetter::LETTER_SIZE,
                letter_clicked
            ),
        ];
        
        let enter_button: LowerLetter = LowerLetter::new(
            ctx,
            &String::from("ENTER"),
            LowerLetter::RECT_WIDTH*3.0/2.0,
            LowerLetter::RECT_HEIGHT,
            initial_dx,
            initial_dy + 2.0*Wordle::SEPARATION_BETWEEN_LOWEER_LETTERS + 2.0*LowerLetter::RECT_HEIGHT,
            LowerLetter::STRING_SIZE,
            enter_clicked,
        );

        let delete_button: LowerLetter = LowerLetter::new(
            ctx,
            &String::from("DELETE"),
            LowerLetter::RECT_WIDTH*3.0/2.0,
            LowerLetter::RECT_HEIGHT,
            initial_dx + 8.0*Wordle::SEPARATION_BETWEEN_LOWEER_LETTERS + 8.5*LowerLetter::RECT_WIDTH,
            initial_dy + 2.0*Wordle::SEPARATION_BETWEEN_LOWEER_LETTERS + 2.0*LowerLetter::RECT_HEIGHT,
            LowerLetter::STRING_SIZE,
            delete_clicked,
        );

        let answer: [char; WORDLE_COLS] = lua_wrapper::get_random_word();

        let curr_letter: (usize, usize) = (0, 0);

        let width = LowerLetter::RECT_WIDTH*3.2;
        let height = LowerLetter::RECT_HEIGHT*(8.0/7.0);

        let offset_x = Self::dist_to_left_of_upper_block() + Self::get_width_of_upper_block() / 2.0 - width / 2.0;
        let offset_y = Self::dist_to_bottom_of_upper_block() + 
            (Self::dist_to_top_of_lower_block() - Self::dist_to_bottom_of_upper_block()) / 2.0 - height / 2.0;

        let text_size = 27.4;

        let you_won_box: AnimatedBox = AnimatedBox::new(
            offset_x,
            offset_y,
            width,
            height,
            UpperLetter::RECT_GREEN,
            &String::from("YOU WON"),
            text_size,
            Color::BLACK,
            no_animation,
            AnimatedArguments::default(),
        );

        let shown_answer_box: AnimatedBox = AnimatedBox::new(
            offset_x,
            offset_y,
            width,
            height,
            LowerLetter::RECT_DARKER_GRAY,
            &answer.iter().collect(),
            text_size,
            Color::WHITE,
            no_animation,
            AnimatedArguments::default(),
        );

        let purple: Color = Color { r: 0.25, g: 0.08, b: 0.38, a: 1.0 };

        let invalid_word: AnimatedBox = AnimatedBox::new(
                offset_x,
                offset_y,
                width,
                height,
                purple,
                &String::from("Invalid word"),
                21.0,
                Color::WHITE,
                swipe_animation,
                AnimatedArguments::for_swipe_animation(
                    ctx,
                    630.0,
                    Duration::from_secs_f32(1.25),
                    Duration::from_secs_f32(2.5),
                ),
        );

        let width = LowerLetter::RECT_WIDTH * 3.0;
        let height = LowerLetter::RECT_HEIGHT * 1.8;

        let offset_x = Self::dist_to_right_of_upper_block() +
            (super::WINDOW_WIDTH - Self::dist_to_right_of_upper_block()) / 2.0 - width / 2.0;
        let offset_y = Self::dist_to_top_of_upper_block() +
            (Self::dist_to_bottom_of_upper_block() - Self::dist_to_top_of_upper_block()) / 2.0 - height / 2.0;

        let replay_button: AnimatedBox = AnimatedBox::new(
            offset_x,
            offset_y,
            width,
            height,
            purple,
            &String::from("PLAY AGAIN"),
            21.0,
            Color::WHITE,
            roll_animation,
            AnimatedArguments::for_roll_animation(
                40.0,
                utility::Direction::Up,
                Wordle::dist_to_top_of_upper_block(),
                Wordle::dist_to_bottom_of_upper_block(),
            ),
        );

        let see_stats_button = AnimatedBox::new(
            Self::dist_to_left_of_upper_block() / 2.0 - width / 2.0,
            Self::dist_to_top_of_upper_block() + (Self::dist_to_bottom_of_upper_block() - Self::dist_to_top_of_upper_block()) / 2.0 - height / 2.0,
            width,
            height,
            purple,
            &"STATISTICS".to_string(),
            20.0,
            Color::WHITE,
            roll_animation,
            AnimatedArguments::for_roll_animation(
                40.0,
                utility::Direction::Down,
                Wordle::dist_to_top_of_upper_block(),
                Wordle::dist_to_bottom_of_upper_block(),
            ),
        );

        Wordle{
            game_board: letters,
            used_letters,
            enter_button,
            delete_button,
            answer,
            curr_letter,
            game_state: GameState::NotOver,
            you_won_box,
            shown_answer_box,
            invalid_word,
            replay_button,
            stats: Statistics::new(),
            see_stats_button,
        }
    }

    pub fn draw_wordle(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        if self.stats.is_on_screen() {
            let _ = self.stats.draw(ctx, canvas);
            return Ok(());
        }
        for i in 0..WORDLE_ROWS {
            for j in 0..WORDLE_COLS {
                let _ = self.game_board[i][j].draw(ctx, canvas);
            }
        }
        for i in 0..WORDLE_LETTERS_COUNT {
            let _ = self.used_letters[i].draw(ctx, canvas);
        }
        let _ = self.enter_button.draw(ctx, canvas);
        let _ = self.delete_button.draw(ctx, canvas);

        if self.you_won_box.is_on_screen() == true {
            let _ = self.you_won_box.draw(ctx, canvas);
        }
        if self.shown_answer_box.is_on_screen() == true {
            let _ = self.shown_answer_box.draw(ctx, canvas);
        }
        if self.invalid_word.is_on_screen() == true {
            let _ = self.invalid_word.draw(ctx, canvas);
        }
        if self.replay_button.is_on_screen() == true {
            let _ = self.replay_button.draw(ctx, canvas);
        }
        if self.see_stats_button.is_on_screen() {
            let _ = self.see_stats_button.draw(ctx, canvas);
        }
        Ok(())
    }

    pub fn update_wordle(&mut self, ctx: &mut Context) -> GameResult {
        let args: &mut AnimatedArguments = &mut self.invalid_word.args.clone();
        let _ = self.invalid_word.update_animated_box(ctx, args);
        self.invalid_word.args = args.clone();

        let args: &mut AnimatedArguments = &mut self.replay_button.args.clone();
        let _ = self.replay_button.update_animated_box(ctx, args);
        self.replay_button.args = args.clone();

        let args: &mut AnimatedArguments = &mut self.see_stats_button.args.clone();
        let _ = self.see_stats_button.update_animated_box(ctx, args);
        self.see_stats_button.args = args.clone();

        Ok(())
    }

    fn is_valid_word(word: [char; WORDLE_COLS]) -> bool {
        let mut temp: String = word.iter().collect();
        temp = temp.to_lowercase();
        let all_words = lua_wrapper::get_five_letter_words();
        let len = all_words.len();
        for i in 0..len {
            if temp == all_words[i] {
                return true;
            }
        }
        false
    }

    fn get_position(&self, ch: char) -> usize {
        for i in 0..WORDLE_LETTERS_COUNT {
            let curr = self.used_letters[i].get_value();
            let curr: char = curr.chars().next().unwrap();
            if ch == curr {
                return i;
            }
        }
        0
    }

    pub fn detect_click(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if self.stats.quit_stats_screen_request(x, y) {
            self.stats.remove_from_screen();
            return;
        }

        if self.curr_letter.1 != WORDLE_COLS {
            for curr_tile in &mut self.used_letters.clone() {
                if curr_tile.point_is_in(x, y) && button == MouseButton::Left {
                    (*curr_tile).update(ctx, self, curr_tile.get_value_char());
                }
            }
        }

        if self.delete_button.point_is_in(x, y) {
            self.delete_button.clone().update(ctx, self, None);
        }

        if self.enter_button.point_is_in(x, y) {
            self.enter_button.clone().update(ctx, self, None);
        }

        if self.see_stats_button.point_is_in(x, y) {
            self.stats.put_on_screen();
        }

        if self.replay_button.point_is_in(x, y) && !self.stats.is_on_screen() {
            *self = Wordle::new(ctx);
        }
    }

    fn get_value_of_typed_key(input: KeyInput) -> Option<char> {
        match input.keycode.unwrap() {
            KeyCode::A => return Some('A'),
            KeyCode::B => return Some('B'),
            KeyCode::C => return Some('C'),
            KeyCode::D => return Some('D'),
            KeyCode::E => return Some('E'),
            KeyCode::F => return Some('F'),
            KeyCode::G => return Some('G'),
            KeyCode::H => return Some('H'),
            KeyCode::I => return Some('I'),
            KeyCode::J => return Some('J'),
            KeyCode::K => return Some('K'),
            KeyCode::L => return Some('L'),
            KeyCode::M => return Some('M'),
            KeyCode::N => return Some('N'),
            KeyCode::O => return Some('O'),
            KeyCode::P => return Some('P'),
            KeyCode::Q => return Some('Q'),
            KeyCode::R => return Some('R'),
            KeyCode::S => return Some('S'),
            KeyCode::T => return Some('T'),
            KeyCode::U => return Some('U'),
            KeyCode::V => return Some('V'),
            KeyCode::W => return Some('W'),
            KeyCode::X => return Some('X'),
            KeyCode::Y => return Some('Y'),
            KeyCode::Z => return Some('Z'),
            _ => {}
        }
        None
    }

    pub fn detect_typing(&mut self, ctx: &mut Context, input: KeyInput) {
        if self.curr_letter.1 != WORDLE_COLS && Wordle::get_value_of_typed_key(input).is_some() {
            let value_typed: char = Wordle::get_value_of_typed_key(input).unwrap();
            for curr_tile in &mut self.used_letters.clone() {
                let value_of_current_letter = curr_tile.get_value_char();
                if value_typed == value_of_current_letter.unwrap() {
                    (*curr_tile).update(ctx, self, value_of_current_letter);
                }
            }
        }

        if input.keycode.unwrap() == KeyCode::Back {
            self.delete_button.clone().update(ctx, self, None);
        }

        if input.keycode.unwrap() == KeyCode::Return {
            self.enter_button.clone().update(ctx, self, None);
        }
    }
}

fn letter_clicked(ctx: &mut Context, wordle: &mut Wordle, value: Option<char>) {
    if wordle.curr_letter.1 == WORDLE_COLS {
        return;
    }
    wordle.game_board[wordle.curr_letter.0][wordle.curr_letter.1].set_letter (ctx, value.unwrap());
    wordle.curr_letter.1 += 1;
}

fn enter_clicked(ctx: &mut Context, wordle: &mut Wordle, _value: Option<char>) {
    if wordle.curr_letter.1 != WORDLE_COLS {
        if !wordle.invalid_word.is_on_screen() {
            wordle.invalid_word.put_on_screen();
        }
        return;
    }
    let mut curr_word: [char; WORDLE_COLS] = [' '; WORDLE_COLS];

    for i in 0..WORDLE_COLS {
        curr_word[i] = wordle.game_board[wordle.curr_letter.0][i].get_value().unwrap();
    }

    if !Wordle::is_valid_word(curr_word) {
        if !wordle.invalid_word.is_on_screen() {
            wordle.invalid_word.put_on_screen();
        }
        return; 
    }

    let cond = scan_entered_word(wordle, ctx, curr_word);

    if cond == GameState::Won || cond == GameState::Lost {
        if cond == GameState::Won {
            wordle.game_state = GameState::Won;
            wordle.you_won_box.put_on_screen();
            wordle.stats.update_stats(Some(wordle.curr_letter.0 as u32 + 1));
        }
        else if cond == GameState::Lost {
            wordle.game_state = GameState::Lost;
            wordle.shown_answer_box.put_on_screen();
            wordle.stats.update_stats(None);
        }
        wordle.replay_button.put_on_screen();
        wordle.see_stats_button.put_on_screen();
        return;
    }
    wordle.curr_letter.1 = 0;
    wordle.curr_letter.0 += 1;
}

fn delete_clicked(_ctx: &mut Context, wordle: &mut Wordle, _value: Option<char>) {
    if wordle.game_state == GameState::Lost || wordle.game_state == GameState::Won {
        return;
    }
    if wordle.curr_letter.1 != 0 {
        wordle.curr_letter.1 -= 1;
        wordle.game_board[wordle.curr_letter.0][wordle.curr_letter.1].clear_letter ();
    }
}

fn scan_entered_word(wordle: &mut Wordle, ctx: &mut Context, word: [char; WORDLE_COLS]) -> GameState {
    let mut temp: String = word.iter().collect();
    temp = temp.to_lowercase();
    let mut ans: String = wordle.answer.iter().collect();
    ans = ans.to_lowercase();

    let mut indedxes_of_taken_letters_of_answer: [bool; WORDLE_COLS] = [false; WORDLE_COLS];
    let mut indedxes_of_taken_letters_of_guess: [bool; WORDLE_COLS] = [false; WORDLE_COLS];

    // searching for the greens in both
    for i in 0..WORDLE_COLS {
        if word[i] == wordle.answer[i] {
            wordle.game_board[wordle.curr_letter.0][i].set_state(ctx, utility::State::CorrectInWord);
            wordle.used_letters[wordle.get_position(word[i])].set_state(ctx, utility::State::CorrectInWord);
            indedxes_of_taken_letters_of_guess[i] = true;
            indedxes_of_taken_letters_of_answer[i] = true;
        }
    }

    // searching for the grays in guess
    for i in 0..WORDLE_COLS {
        let mut curr_letter_in_guess_is_gray: bool = true;
        for j in 0..WORDLE_COLS {
            if word[i] == wordle.answer[j] {
                curr_letter_in_guess_is_gray = false;
            }
        }
        if curr_letter_in_guess_is_gray == true {
            wordle.game_board[wordle.curr_letter.0][i].set_state(ctx, utility::State::NotInWord);
            wordle.used_letters[wordle.get_position(word[i])].set_state(ctx, utility::State::NotInWord);
            indedxes_of_taken_letters_of_guess[i] = true;
        }
    }

    // searching for the yellows in both
    for i in 0..WORDLE_COLS {
        for j in 0..WORDLE_COLS {
            if word[i] == wordle.answer[j] {
                if indedxes_of_taken_letters_of_guess[i] == false && indedxes_of_taken_letters_of_answer[j] == false {
                    wordle.game_board[wordle.curr_letter.0][i].set_state(ctx, utility::State::IncorrectInWord);
                    wordle.used_letters[wordle.get_position(word[i])].set_state(ctx, utility::State::IncorrectInWord);
                    indedxes_of_taken_letters_of_guess[i] = true;
                    indedxes_of_taken_letters_of_answer[j] = true;
                }
            }
        }
    }

    // searching for the missed (repeating) letters
    for i in 0..WORDLE_COLS {
        if indedxes_of_taken_letters_of_guess[i] == false {
            wordle.game_board[wordle.curr_letter.0][i].set_state(ctx, utility::State::NotInWord);
            wordle.used_letters[wordle.get_position(word[i])].set_state(ctx, utility::State::NotInWord);
            indedxes_of_taken_letters_of_guess[i] = true;
        }
    }

    if ans == temp {
        return GameState::Won;
    }
    else if wordle.curr_letter.0 != WORDLE_ROWS - 1 {
        return GameState::NotOver;
    }
    else {
        return GameState::Lost;
    }
}
