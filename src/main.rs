use std::io;
mod app;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| app::App::default().run(terminal))
}
