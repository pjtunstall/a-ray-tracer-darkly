use std::io::{self, Write};

use terminal_size::{Width, terminal_size};

pub fn show(current: usize, total: usize, label: &str) {
    let percent = current * 100 / total;

    let terminal_width = match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 80,
    };

    let necessary_deduction = 5 + 4 + 2 + 2 + label.len() + 1; // 5 spaces indent + 4 chars for "100%" + 2 spaces + 2 brackets + label + spaces after label.
    let fixed_deduction = necessary_deduction + 24;

    let bar_width = terminal_width.saturating_sub(fixed_deduction);

    let filled_len = current * bar_width / total;

    let bar = if current == total {
        "=".repeat(bar_width)
    } else if filled_len > 0 {
        let mut bar = "=".repeat(filled_len - 1);
        bar.push('>');
        bar.push_str(&" ".repeat(bar_width - filled_len));
        bar
    } else {
        let mut bar = String::new();
        bar.push('>');
        bar.push_str(&" ".repeat(bar_width - 1));
        bar
    };

    print!("\r     \x1b[1m{}\x1b[0m [{}] {:>3}%", label, bar, percent);
    io::stdout().flush().unwrap();
}
