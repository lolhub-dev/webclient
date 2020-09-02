use regex::Regex;

const STATIC_PATH: &str = "static";
const MOCK_PATH: &str = "static/mocks";
const IMAGES_PATH: &str = "static/images";

pub fn check_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    email_regex.is_match(email)
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

pub fn mock_path(mock: &str) -> String {
    format!("{}/{}", MOCK_PATH, mock)
}
