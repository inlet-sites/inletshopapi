use std::fs;

pub fn delete_files(urls: Vec<String>) {
    let home = match std::env::var("HOME_DIR") {
        Ok(h) => h,
        Err(_) => {
            eprintln!("Failed to remove files: {:?}", &urls);
            return;
        }
    };

    for u in urls {
        let full_path = format!("{}srv/{}", home, u);
        match fs::remove_file(full_path) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Failed to remove file: {}", u);
                ()
            }
        };
    }
}
