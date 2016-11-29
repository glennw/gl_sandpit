use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub fn load_text_file(path: &PathBuf) -> String {
    let f = File::open(path);
    match f {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            s
        }
        Err(..) => {
            panic!(format!("Unable to open file {}", path.display()));
        }
    }
}
