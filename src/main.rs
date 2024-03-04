use art_color_mixer::PrimaryColor;
use art_color_mixer::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
}