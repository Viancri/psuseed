#[derive(Debug, RustcDecodable)]
pub struct Config {
    /// default psu-clementine.ddns.net
    pub host: Option<String>,
    /// default 11030
    pub patch_port: Option<u16>,
    /// default 12030
    pub login_port: Option<u16>,
    /// default 1280
    pub width: Option<u32>,
    /// default 720
    pub height: Option<u32>,
    pub borderless: Option<bool>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: None,
            patch_port: None,
            login_port: None,
            width: None,
            height: None,
            borderless: None
        }
    }
}
