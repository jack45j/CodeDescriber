use std::io::Write;

// @一些helper function@
pub fn display_progress(current_idx: usize, total_files: usize) {
    let progress = (current_idx + 1) as f32 / total_files as f32 * 100.0;
    print!("\rProcessing: [{:<50}] {:.2}%", "#".repeat((progress as usize) / 2), progress);
    std::io::stdout().flush().unwrap();
}