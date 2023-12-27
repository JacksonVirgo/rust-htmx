use std::process::Command;

fn main() {
    Command::new("npx").args(&["tailwindcss", "-i", "./tailwind.input.css", "-o", "./public/styles.css"]).status().unwrap();
}