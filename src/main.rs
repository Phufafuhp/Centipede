use std::{
	io::{stdout, Write},
	time::{Duration, Instant},
};

use crossterm::{
	cursor::{Hide, MoveTo, Show},
	event::{self, Event, KeyCode, KeyEventKind},
	execute, queue,
	style::{Color, Print, ResetColor, SetForegroundColor},
	terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

const WIDTH = 50;
const HEIGHT = 30;

enum Status {
	Playing,
	Won,
	Lost,
}

enum Input {
	Up,
	Down,
	Left,
	Right,
	Shoot,
}

struct Position {
	x: u32,
	y: u32,
}

struct Player {
	pos: Position,
	lives: u32,
}

struct Game {
	player: Player,
	status: Status,
}

impl Game {
	fn new() -> self {
		Game {
			Player {
				Pos {
					25,
					15,
				},
				3,
			},
			Status::Playing,
		}
	}

	fn handle_input(Input) {
		match
	}
}


fn main() -> std::io::Result<()> {
	let mut stdout = stdout();
	terminal::enable_raw_mode()?;
	execute!(stdout, Hide, EnterAlternateScreen)?;

	let mut game = Game::new()
	loop {
		while event::poll(Duration::from_millis(0))?{
			if let Event::Key(key) = event::read()? {
				if key.kind == KeyEventKind::Release {
					continue;
				}
				match key.code {
					KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
						return Ok(());
					}
					Keycode::Char('a') | KeyCode::Char('A') | KeyCode::Left => {
						handle_input()
					}
					_ => {}
				}
			}
		}
	}

	//let result = run(&mut stdout);

	execute!(stdout, Show, LeaveAlternateScreen)?;
	terminal::disable_raw_mode()?;
	//result
}