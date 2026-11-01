use std::fs;

pub fn check_and_save_death(x: f64, y: f64, z: f64) {
    if let Ok(mut path) = std::env::current_dir() {
        path.push("config");
        let _ = fs::create_dir_all(&path);
        path.push("death_marker.json");
        
        let data = format!(
            "{{\"x\":{:.2}, \"y\":{:.2}, \"z\":{:.2}}}",
            x, y, z
        );
        
        if fs::write(&path, data).is_ok() {
            eprintln!("[death_marker :3 ] Saved death position: {:.1}, {:.1}, {:.1}", x, y, z);
        }
    }
}

// oh, look a comment -\
//              /|\     |
//                \____/