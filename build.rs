use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;


fn main() {
    let repo_url = "https://github.com/raspberrypi/utils.git";
    let repo_dir = "utils";
    let assets_dir = "assets";

    // Clone the repository
    if !Path::new(repo_dir).exists() {
        println!("Cloning the utils repository...");
        Command::new("git")
            .args(&["clone", repo_url])
            .status()
            .expect("Failed to clone the repository");
    }

    // Build pinctrl from the utils repo
    println!("Building pinctrl...");

    match Command::new("apt-get").args(&["install", "-y", "cmake", "device-tree-compiler", "libfdt-dev"]).status()
    {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Failed to install dependencies. Please make sure you have the following packages installed:");
            eprintln!("cmake, device-tree-compiler, libfdt-dev");
        }
    }

    Command::new("cmake")
        .arg("CMakeLists.txt")
        .current_dir(repo_dir)
        .status()
        .expect("Failed to build pinctrl");

    Command::new("make")
        .arg("pinctrl")
        .current_dir(repo_dir)
        .status()
        .expect("Failed to build pinctrl");

    // Create the assets directory if it doesn't exist
    fs::create_dir_all(assets_dir).unwrap();

    // Move the compiled pinctrl binary to the assets directory
    println!("Moving pinctrl to the assets directory...");
    fs::rename(
        format!("{}/pinctrl/pinctrl", repo_dir),
        format!("{}/pinctrl", assets_dir),
    ).unwrap();
    println!("pinctrl is now available in the assets directory.");

    //remove the utils directory
    fs::remove_dir_all(repo_dir).unwrap();

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
