use std::fs;

pub fn delete_files(urls: Vec<String>, use_srv_dir: bool) {
    let home = if use_srv_dir {
        match std::env::var("HOME_DIR") {
            Ok(h) => format!("{}srv/", h),
            Err(_) => {
                eprintln!("Failed to remove files: {:?}", &urls);
                return;
            }
        }
    } else {
        "".to_string()
    };

    for u in urls {
        let full_path = format!("{}{}", home, u);
        match fs::remove_file(full_path) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Failed to remove file: {}", u);
                ()
            }
        };
    }
}
