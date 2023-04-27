use std::io::{self, Write};
use std::time::Instant;

pub trait ProgressBar {
    fn update(&self, current_step: usize);
}

pub struct SpinnerProgressBar {
    message: String,
    start_time: Instant,
}

impl SpinnerProgressBar {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            start_time: Instant::now(),
        }
    }
}

impl ProgressBar for SpinnerProgressBar {
    fn update(&self, _current_step: usize) {
        // Define the spinner animation
        let spinner_frames = vec!["|", "/", "-", "\\"];
        let current_frame = spinner_frames[self.start_time.elapsed().as_millis() as usize % spinner_frames.len()];

        // Build the spinner string
        let spinner_string = format!("{} {}", self.message, current_frame);

        // Print the spinner string and update the cursor position
        print!("\r{}", spinner_string);
        io::stdout().flush().unwrap();
    }
}

pub struct PieChartProgressBar {
    message: String,
    steps: usize,
    bar_width: usize,
}

impl PieChartProgressBar {
    pub fn new(message: &str, steps: usize, bar_width: usize) -> Self {
        Self {
            message: message.to_string(),
            steps,
            bar_width,
        }
    }
}

impl ProgressBar for PieChartProgressBar {
    fn update(&self, current_step: usize) {
        // Calculate the progress percentage
        let progress_percent = ((current_step + 1) as f64 / self.steps as f64 * 100.0).round() as usize;

        // Calculate the completed pie chart segments and remaining segments
        let segments = ((current_step + 1) as f64 / self.steps as f64 * self.bar_width as f64) as usize;
        let remaining_segments = self.bar_width - segments;

        // Build the pie chart string
        let pie_chart_string = format!(
            "[{}] {}%",
            format!("{}{}", "█".repeat(segments), "░".repeat(remaining_segments)),
            progress_percent
        );
        // Build the full progress bar string
        let full_progress_string = format!("{} {}", self.message, pie_chart_string);

        // Print the progress bar string and update the cursor position
        print!("\r{}", full_progress_string);
        io::stdout().flush().unwrap();
    }
}
