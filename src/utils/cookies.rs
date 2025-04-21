use tower_cookies::{Cookie, Cookies};

pub async fn set_cookie(
    name: String,
    value: String,
    http_only: bool,
    path: String,
    cookies: &Cookies,
) {
    let mut cookie = Cookie::new(name, value);
    cookie.set_http_only(http_only);
    cookie.set_secure(true);
    cookie.set_path(path);
    cookies.add(cookie)
}

pub async fn read_cookies(cookies: Cookies, name: &str) -> Result<String, &'static str> {
    cookies
        .get(name)
        .map(|c| c.value().to_string())
        .ok_or("Cookie not found")
}
