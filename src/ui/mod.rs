use crate::app::{self, SelectionMode};
use chrono::{DateTime, Local};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::Marker,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, Paragraph},
    Frame, Terminal,
};

pub fn init(app: &mut app::App) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;
   
    terminal.clear()?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Ratio(3, 5), Constraint::Ratio(2, 5)].as_ref())
            .split(f.size());

        draw_map(f, chunks[0], app);
        draw_info_panel(f, chunks[1], app);
    })?;

    loop {
        match read_input(app) {
            Ok(Some(_)) => terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Ratio(3, 5), Constraint::Ratio(2, 5)].as_ref())
                    .split(f.size());

                draw_map(f, chunks[0], app);
                draw_info_panel(f, chunks[1], app);
            })?,
            Ok(None) => {}
            Err(_) => todo!(),
        }
    }
}

fn read_input(app: &mut app::App) -> crossterm::Result<Option<()>> {
    if poll(Duration::from_millis(50))? {
        let event = read()?;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: _,
            }) => {
                app.toggle_selection_mode();
                return Ok(Some(()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: _,
            }) => {
                app.move_selection(app::SelectionChange::UP);
                return Ok(Some(()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: _,
            }) => {
                app.move_selection(app::SelectionChange::DOWN);
                return Ok(Some(()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: _,
            }) => {
                app.move_selection(app::SelectionChange::LEFT);
                return Ok(Some(()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: _,
            }) => {
                app.move_selection(app::SelectionChange::RIGHT);
                return Ok(Some(()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: mods,
            }) => {
                app.cycle_color(mods == KeyModifiers::CONTROL);
                return Ok(Some(()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: mods,
            }) => {
                app.cycle_foreground(mods == KeyModifiers::CONTROL);
                return Ok(Some(()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('g'),
                modifiers: mods,
            }) => {
                app.cycle_background(mods == KeyModifiers::CONTROL);
                return Ok(Some(()));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: _,
            }) => {
                disable_raw_mode()?;
                std::process::exit(0);
            }
            Event::Resize(_, _) => return Ok(Some(())),
            _ => {}
        }
    }
    Ok(None)
}

pub fn draw_map<B: Backend>(f: &mut Frame<B>, area: Rect, app: &app::App) {
    let sf = app.starfield();
    let sys = app.system_coords();
    let bh = app.black_hole_coords();
    let sel = app.selection(app::SelectionCursor::BLOCK);

    let mut datasets = vec![
        // Starfield
        Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::Gray))
            .data(sf),
        // Systems
        Dataset::default()
            .marker(Marker::Block)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::Indexed(app.color().0)))
            .data(sys.as_slice()),
        // Black holes
        Dataset::default()
            .marker(Marker::Block)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::DarkGray))
            .data(bh.as_slice()),
    ];

    if app.selection_mode() == SelectionMode::MAP {
        datasets.push(
            Dataset::default()
                .marker(if sel.len() > 1 {
                    Marker::Braille
                } else {
                    Marker::Block
                })
                .graph_type(if sel.len() > 1 {
                    GraphType::Line
                } else {
                    GraphType::Scatter
                })
                .style(Style::default().fg(Color::White))
                .data(sel.as_slice()),
        )
    }

    let map = Chart::new(datasets)
        .block(
            Block::default()
                .title("Sector")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(match app.selection_mode() {
                    app::SelectionMode::MAP => Color::Indexed(app.color().1),
                    _ => Color::Reset,
                })),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::White))
                .bounds([0.0, app.sector().columns.into()]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::White))
                .bounds([0.0, app.sector().rows.into()]),
        );

    f.render_widget(map, area);
}

pub fn draw_info_panel<B: Backend>(f: &mut Frame<B>, area: Rect, app: &app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(area);

    let system_block = Block::default()
        .title(format!(
            "System [{:0>2}{:0>2}]",
            app.cursor().0 - 1,
            app.cursor().1 - 1,
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.selection_mode() {
            app::SelectionMode::SYSTEM => Color::Indexed(app.color().1),
            _ => Color::Reset,
        }));

    let layer_block = Block::default()
        .title("Objects")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.selection_mode() {
            app::SelectionMode::OBJECTS => Color::Indexed(app.color().1),
            _ => Color::Reset,
        }));

    if let Some(system) = app.selected_system() {
        let tz = Local::now().timezone();
        let tfmt = "%b %e, %Y - %r";
        let info = Paragraph::new(format!(
            "\
{} {}
  Coordinates: [{:0>2}{:0>2}]
  Created: {}
  Updated: {}
",
            system.name,
            if system.is_hidden { "(hidden)" } else { "" },
            app.cursor().0 - 1,
            app.cursor().1 - 1,
            if let Some(date) = &system.created {
                DateTime::parse_from_rfc3339(date)
                    .unwrap()
                    .with_timezone(&tz)
                    .format(tfmt)
                    .to_string()
            } else {
                "None".into()
            },
            if let Some(date) = &system.created {
                DateTime::parse_from_rfc3339(date)
                    .unwrap()
                    .with_timezone(&tz)
                    .format(tfmt)
                    .to_string()
            } else {
                "None".into()
            },
        ))
        .block(system_block);
        f.render_widget(info, chunks[0]);

        let layers = List::new(
            app.world()
                .child_planets(system)
                .iter()
                .flatten()
                .map(|&p| ListItem::new(p.name.clone()))
                .collect::<Vec<_>>(),
        )
        .block(layer_block)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Indexed(app.color().0)));
        f.render_widget(layers, chunks[1]);
    } else {
        f.render_widget(system_block, chunks[0]);
        f.render_widget(layer_block, chunks[1]);
    }
}
