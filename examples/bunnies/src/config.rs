pub const N_BUNNIES_PER_TICK: usize = 100;
pub const START_GRAVITY: f64 = 0.75;

pub fn get_media_href(path: &str) -> String {
    format!("/public/media/{}", path)
}
