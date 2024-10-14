use image::GenericImageView;
use std::path::Path;
use std::result::Result;
use viuer::{print_from_file, Config};

fn main() {
    // Load the image from a file using the `image` crate
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let img_path = base_path.join("resource").join("test.jpg");

    let mut conf = Config::default();
    conf.use_kitty = false;
    conf.use_iterm = true;
    print_from_file(img_path, &conf).expect("Image printing failed.");
}
