use app::App;

mod app;
mod words;

fn main() {
    let language = match words::load_language() {
        Ok(language) => language,
        Err(_) => {
            eprintln!("Failed to load language. Please save a language.json file in the wordtui config directory.");
            return;
        }
    };
    let mut terminal = ratatui::init();
    terminal.clear().expect("Failed to clear terminal");
    let app_result = App::new(language).run(&mut terminal);
    ratatui::restore();
    let language = match app_result {
        Ok(language) => language,
        Err(_) => {
            eprintln!("Wordtui failed to run.");
            return;
        }
    };

    match words::save_language(language) {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to save language. Progress will not be saved."),
    }
}
