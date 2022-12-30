use std::process::exit;

pub fn respond(status: &str, content_type: &str, message: &str) -> ! {
    print!("Status: {}\r\n", status);
    print!("Content-Type: {};\r\n", content_type);
    print!("\r\n");
    println!("{}", message);
    exit(0);
}

pub fn plain_text_response(status: &str, message: &str) -> ! {
    respond(status, "text/plain", message);
}

pub fn html_text_response(status: &str, message: &str) -> ! {
    respond(status, "text/html", message);
}

pub fn redirect(to: &str) {
    print!("Status: 302 Found\r\n");
    print!("Location: {}\r\n", to);
    print!("\r\n");
    exit(0);
}

pub fn bad_request(message: &str) -> ! {
    plain_text_response("400 Bad Request", message);
}

pub fn not_found(message: &str) -> ! {
    plain_text_response("404 Not Found", message);
}

pub fn internal_server_error(message: &str) -> ! {
    plain_text_response("500 Internal Server Error", message);
}
