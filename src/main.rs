use std::env;
use walkdir::WalkDir;

fn main() {
    // Get the command line argument for apath
    let args: Vec<String> = env::args().collect();
    let apath = if args.len() > 1 {
        args[1].clone()
    } else {
        panic!("Please provide a path as a command line argument.")
    };

    for e in WalkDir::new(apath.clone())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();
            let ext_split: Vec<&str> = fname.split(".").collect();
            let ext = ext_split.last().unwrap().to_string();
            if ext == "jpeg".to_string() {
                let new_fname = fname.replace(".jpeg", ".jpg");
                println!("Renaming file: {}\n to\n {}", fname, new_fname);
                // Uncomment the line below to actually rename the file
                // std::fs::rename(&fname, &new_fname).unwrap();
            }
        }
    }
}