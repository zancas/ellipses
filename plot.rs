#! /usr/bin/env rust-script

fn main() {
    let remove_image = std::process::Command::new("rm")
        .args(&["-rf", "images/0.1.png"])
        .status()
        .expect("Failed to remove the image");

    if remove_image.success() {
        println!("Image removed successfully.");
    } else {
        println!("Failed to remove the image.");
    }

    // Step 2: Run the Cargo build
    let cargo_run = std::process::Command::new("cargo")
        .arg("run")
        .status()
        .expect("Failed to run cargo");

    if cargo_run.success() {
        println!("Cargo run completed successfully.");
    } else {
        println!("Cargo run failed.");
        return;
    }

    // Step 3: Open the image using Brave browser
    let open_image = std::process::Command::new("brave")
        .arg("images/0.1.png")
        .status()
        .expect("Failed to open the image with Brave");

    if open_image.success() {
        println!("Image opened in Brave successfully.");
    } else {
        println!("Failed to open the image in Brave.");
    }
}
