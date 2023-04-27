## Usage

Add to dependencies
```
[dependencies]
progressbars = "0.1.0"
```
There are 2 types of progressbars available: PieChartProgressBar, SpinnerProgressBar.

__PieChartProgressBar__
```
use std::thread;
use std::time::Duration;
use progressbars::ProgressBar;
use progressbars::PieChartProgressBar;

fn main() {
    let percentage_progress_bar = PieChartProgressBar::new("Percentage:", 10, 20);
    for i in 0..10 {
        percentage_progress_bar.update(i);
        thread::sleep(Duration::from_millis(100));
    }
    println!();
}
```
Looks like:
```
Percentage: [████████░░░░░░░░░░░░] 40%
```
__SpinnerProgressBar__
```
use std::thread;
use std::time::Duration;
use progressbars::ProgressBar;
use progressbars::SpinnerProgressBar;

fn main() {
    let spinner_bar = SpinnerProgressBar::new("Percentage:");
    for i in 0..10 {
        spinner_bar.update(i);
        thread::sleep(Duration::from_millis(100));
    }
    println!();
}
```
Looks like:
```
Percentage: |
```
