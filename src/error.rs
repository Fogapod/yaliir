pub fn error(line: i32, message: String) {
    report(line, String::from(""), message);
}

fn report(line: i32, place: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, place, message);

    //had_error = true;
}
