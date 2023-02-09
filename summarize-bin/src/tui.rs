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
    style::{Color, Style, Modifier},
    widgets::{BarChart, Block, Borders, Row, Table, TableState},
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

macro_rules! field {
    ($summary:expr, $state:expr) => {
        match $state {
            State::Harm => &$summary.harm,
            State::Fund => &$summary.fund,
            State::Corr => &$summary.corr,
        }
    };
}

struct App {
    summaries: Vec<Summary>,
    state: State,
    table_state: TableState,
}

impl App {
    fn new(summaries: Vec<Summary>) -> Self {
        Self {
            summaries,
            state: State::Harm,
            table_state: TableState::default(),
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

    fn rows(&self) -> usize {
        let [a, b] = &self.summaries[..] else { unimplemented!() };
        a.harm.len().min(b.harm.len())
    }

    fn table<'a, 'b>(&'a self) -> Table<'b> {
        let mut rows = Vec::new();
        let [a, b] = &self.summaries[..] else { unimplemented!() };
        let end = self.rows();
        let label = match self.state {
            State::Harm => "ω",
            State::Fund => "ν",
            State::Corr => "ν",
        };
        for i in 0..end {
            rows.push(
                Row::new(vec![
                    format!("{:>5}", format!("{label}{}", i + 1)),
                    format!("{:8.1}", field!(a, self.state)[i]),
                    format!("{:8.1}", field!(b, self.state)[i]),
                ])
                .style(Style::default().fg(Color::Black)),
            );
        }
        Table::new(rows)
    }

    fn next_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.rows() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn prev_row(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
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
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('n') => app.state = app.state.next(),
                    KeyCode::Char('p') => app.state = app.state.prev(),
                    KeyCode::Char('j') => app.next_row(),
                    KeyCode::Char('k') => app.prev_row(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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

    let mode = format!("{:>5}", "Mode");
    let mol1 = format!("{:>8}", "Mol. 1");
    let mol2 = format!("{:>8}", "Mol. 2");
    let table = app.table();
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let table = table
        .header(
            Row::new(vec![mode.as_str(), mol1.as_str(), mol2.as_str()])
                .style(Style::default().fg(Color::Yellow))
                .height(1),
        )
        .block(
            Block::default()
                .title(app.state.title())
                .borders(Borders::ALL),
        )
        .widths(&[
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
        ])
        .column_spacing(1).highlight_style(selected_style);
    f.render_stateful_widget(table, chunks[1], &mut app.table_state);
}
