use std::env;
use std::fs;

use grid_games::game_of_life::GameOfLife;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::{io, thread, time::Duration};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

const FRAME_TIME: u64 = 100;
const FRAME_COUNT: u64 = 100;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments, you need to pass a filename");
    }

    let basefile = &args[1];
    println!("Game of Life - Example {}", basefile);

    let contents = fs::read_to_string(basefile).expect("Should have been able to read the file");

    let mut board: GameOfLife = GameOfLife::from_str(&contents).unwrap();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    for _ in 0..=FRAME_COUNT {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().borders(Borders::ALL).title("Game of Life");
            board.expand((size.width - 2).into(), (size.height - 2).into());

            let as_str = board.to_str();
            let text: Vec<Spans<'_>> = as_str
                .lines()
                .map(|f| Spans::from(Span::styled(f, Style::default().fg(Color::Green))))
                .collect();

            let paragraph = Paragraph::new(text)
                .block(block)
                .wrap(tui::widgets::Wrap { trim: true });

            f.render_widget(paragraph, size);
        })?;
        board.iterate();
        thread::sleep(Duration::from_millis(FRAME_TIME));
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
