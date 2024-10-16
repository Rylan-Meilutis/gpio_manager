use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let pyi_parts_dir = Path::new("pyi_stubs");
    let output_pyi = Path::new("gpio_manager.pyi");
    let mut combined = File::create(&output_pyi).expect("Could not create output .pyi file");

    for entry in fs::read_dir(pyi_parts_dir).expect("Failed to read pyi_parts directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("pyi") {
            let contents = fs::read_to_string(path).expect("Failed to read .pyi part");
            writeln!(combined, "{}", contents).expect("Failed to write to combined .pyi");
        }
    }
}
