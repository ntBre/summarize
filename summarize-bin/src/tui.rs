use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use summarize::Summary;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
    Frame, Terminal,
};

enum State {
    Harm,
    Fund,
    Corr,
}

impl State {
    fn next(&self) -> State {
        use State::*;
        match self {
            Harm => Fund,
            Fund => Corr,
            Corr => Harm,
        }
    }

    fn prev(&self) -> State {
        use State::*;
        match self {
            Harm => Corr,
            Fund => Harm,
            Corr => Fund,
        }
    }

    fn title(&self) -> &'static str {
        use State::*;
        match self {
            Harm => "Harmonic Frequencies",
            Fund => "Fundamental Frequencies",
            Corr => "Corrected Frequencies",
        }
    }
}

struct App {
    summaries: Vec<Summary>,
    state: State,
}

impl App {
    fn new(summaries: Vec<Summary>) -> Self {
        Self {
            summaries,
            state: State::Harm,
        }
    }

    fn data(&self, height: u16) -> Vec<(String, u64)> {
        let mut data = Vec::new();
        let mut max = 0.0;
        let pairs = match self.state {
            State::Harm => self.summaries[0]
                .harm
                .iter()
                .zip(self.summaries[1].harm.iter()),
            State::Fund => self.summaries[0]
                .fund
                .iter()
                .zip(self.summaries[1].fund.iter()),
            State::Corr => self.summaries[0]
                .corr
                .iter()
                .zip(self.summaries[1].corr.iter()),
        };
        for (a, b) in pairs {
            // absolute value for now to simplify height calculations
            let diff = (a - b).abs();
            if diff > max {
                max = diff;
            }
            data.push((format!("{diff:6.1}"), diff));
        }
        let mut ret = Vec::new();
        for (s, d) in data.into_iter() {
            ret.push((s, (d / max * height as f64) as u64));
        }
        ret
    }
}

pub fn run_tui(summaries: Vec<Summary>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new(summaries);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}")
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') => app.state = app.state.next(),
                    KeyCode::Char('k') => app.state = app.state.prev(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    const MARGIN_WIDTH: u16 = 2;
    const BAR_GAP: u16 = 1;
    let r = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(MARGIN_WIDTH)
        .constraints(
            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
        )
        .split(f.size());
    let data = &app.data(r.height);
    let nbars = data.len() as u16;
    let v: Vec<_> = data.iter().map(|(a, b)| (a.as_ref(), *b)).collect();
    let barchart = BarChart::default()
        .block(
            Block::default()
                .title(app.state.title())
                .borders(Borders::ALL),
        )
        .data(&v)
        .bar_gap(BAR_GAP)
        .bar_width((r.width - nbars * BAR_GAP - MARGIN_WIDTH) / nbars)
        .bar_style(Style::default().fg(Color::Yellow))
        // can't figure out how to disable the value labels, so make them the
        // same color as the background
        .value_style(Style::default().fg(Color::Yellow).bg(Color::Yellow));
    f.render_widget(barchart, chunks[0]);
}
