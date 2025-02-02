use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    restore, Terminal,
};
use std::io::{self};
use ui::Ui;

pub mod app;
pub mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    // setup terminal
    enable_raw_mode()?;
    set_panic_hook();
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);

    if let Err(err) = res {
        eprintln!("{err}")
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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ui = Ui::new()?;
    loop {
        terminal.draw(|f| ui.ui(f, app))?;
        if let Event::Key(event) = event::read()? {
            if event.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            ui.handle_input(&event, app)?;
            if event.code == KeyCode::Esc {
                break;
            }
        }
    }
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore(); // ignore any errors as we are already failing
        hook(panic_info);
    }));
}
