
use ansiterm::Color;

pub fn log_warning(txt: &str) {
  tracing::warn!("{}: {}", Color::Yellow.paint("warning"), txt);
}

pub fn log_error(txt: &str) {
  tracing::warn!("{}: {}", Color::Red.paint("error"), txt);
}

pub fn log_green(txt: &str) {
  std_out(txt, Color::Green);
}

pub fn log_yellow(txt: &str) {
  std_out(txt, Color::Yellow);
}

pub fn log_green_bold(txt: &str) {
  tracing::info!("{}", Color::Green.bold().paint(txt));
}

pub fn log_yellow_bold(txt: &str) {
  tracing::info!("{}", Color::Yellow.bold().paint(txt));
}

pub fn log_yellow_err(txt: &str) {
  std_err(txt, Color::Yellow);
}

pub fn log_red_err(txt: &str) {
  std_err(txt, Color::Red);
}


fn std_out(text: &str, color: Color) {
  tracing::info!("{}", color.paint(text));
}

fn std_err(txt: &str, color: Color) {
  tracing::error!("{}", color.paint(txt));
}
