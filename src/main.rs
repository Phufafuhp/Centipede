use std::{
	fmt::format, io::{Write, stdout}, time::{Duration, Instant},
};

use crossterm::{
	cursor::{Hide, MoveTo, Show},
	event::{self, Event, KeyCode, KeyEventKind},
	execute, queue,
	style::{Color, Print, ResetColor, SetForegroundColor},
	terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

const WIDTH: usize = 50;
const HEIGHT: usize = 30;

const PLAYER_SPRITE: &str = "^";

enum Status {
	//Paused
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
	score: u32,
	status: Status,
}

impl Game {
	fn new() -> Self {
		Game {
			player: Player {
				pos: Position {
					x: 25,
					y: 15,
				},
				lives: 3,
			},
			score: 0,
			status: Status::Playing,
		}
	}

	fn handle_input(&mut self, input: Input) {
		match input {
			Input::Up => {
				if self.player.pos.y > 0 {
					self.player.pos.y -= 1;
				}
			}
			Input::Down => {
				if self.player.pos.y < HEIGHT as u32 - 1 {
					self.player.pos.y += 1;
				}
			}
			Input::Left => {
				if self.player.pos.x > 0 {
					self.player.pos.x -= 1;
				}
			}
			Input::Right => {
				if self.player.pos.x < WIDTH as u32 - 1{
					self.player.pos.x += 1;
				}
			}
			Input::Shoot =>{
				return; //more to do here >:(
			}
		}
	}

	fn update(&mut self) {
		//shtuff to put here
		return;
	}

	fn render(&self) -> Vec<String> {
		let mut grid = [[' ' ; WIDTH] ; HEIGHT];

		let put_char = |grid: &mut [[char; WIDTH]; HEIGHT], x: u32, y: u32, ch: char| {
			grid[y as usize][x as usize] = ch;
		};

		let put_str = |grid: &mut [[char; WIDTH]; HEIGHT], x: u32, y: u32, s: &str| {
			for (i, ch) in s.chars().enumerate() {
				put_char(grid, x + i as u32, y, ch);
			}
		};

		put_str(&mut grid, self.player.pos.x, self.player.pos.y, PLAYER_SPRITE);
		grid.into_iter()
		.map(|line| line.into_iter().collect())
		.collect()
	}
}

fn draw(stdout: &mut impl Write, game: &Game) -> std::io::Result<()> {
    queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

	let header = format!(" CENTIPEDE\tScore: {:>4}\tLives: {}", game.score, game.player.lives);

	queue!(stdout, SetForegroundColor(Color::Cyan), Print(header), ResetColor, Print("\t\n"))?;
	for line in game.render() {
		queue!(stdout, Print(line), Print("\r\n"))?;
	}

	queue!(stdout, MoveTo(0, (HEIGHT + 3) as u16))?;
    stdout.flush()?;
	Ok(())
}

fn main() -> std::io::Result<()> {
	let mut stdout = stdout();
	terminal::enable_raw_mode()?;
	execute!(stdout, Hide, EnterAlternateScreen)?;

	



	let result = run(&mut stdout);

	execute!(stdout, Show, LeaveAlternateScreen)?;
	terminal::disable_raw_mode()?;
	result
}


fn run(stdout: &mut impl Write) -> std::io::Result<()> {
	let mut game = Game::new();
	let frame = Duration::from_millis(25);
	let mut last = Instant::now();

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
					KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
						game.handle_input(Input::Up);
					}
					KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
						game.handle_input(Input::Down);
					}
					KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => {
						game.handle_input(Input::Left);
					}
					KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => {
						game.handle_input(Input::Right);
					}
					KeyCode::Char(' ') => {
						game.handle_input(Input::Shoot);
					}
					_ => {}
				}
			}
		}

		if last.elapsed() >= frame {
			game.update();
			draw(stdout, &game)?;
			last = Instant::now();

        std::thread::sleep(Duration::from_millis(5));
		}
	}
}