pub struct Config {
    pub width: i32,
    pub height: i32,
    pub game_id: &'static str,
    pub author: &'static str,
    pub window_title: &'static str,
}

pub const CONFIG: Config = Config {
    width: 200,
    height: 200,
    game_id: "rustris",
    author: "Abderrahmane Tahri Jouti",
    window_title: "Clonetris, made easy, in Rust!",
};
