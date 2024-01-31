use toml_edit::{Document, value};

const INPUT: &str = "[package]\r
name = \"toml_edit_repro\"
version = \"0.1.0\"
edition = \"2021\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
toml_edit = \"0.21.0\"
";

fn main() {
    let mut doc = INPUT.parse::<Document>().expect("invalid doc");
    assert_eq!(doc.to_string(), INPUT);
    println!("All good");
}


