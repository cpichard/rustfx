
use std::fs::DirEntry;
use std::path::PathBuf;
use std::io::Result;
use std::env;

pub struct Bundle {
    root: String,
    nb_plugins: u32,
    //plugin_names: Vec<str>,
}


pub fn get_bundle_paths() -> Vec<PathBuf> {
    let mut paths : Vec<PathBuf> = Vec::new();
    match env::var_os("OFXPLUGINS") {
        Some(inline_paths) => {
            for path in env::split_paths(&inline_paths) {

                println!("Found '{}'", path.display());
                paths.push(path);
            }   
        }

        None => {
            println!("environment variable OFXPLUGINS not set");
            // TODO Panic ?
        }
    }

    paths
}

/// Returns true if the given dir is an ofx bundle
fn is_ofx_bundle(dir: & Result<DirEntry>) -> bool {
    match *dir {
        Ok(ref entry) => {
            // Should end with bundle and be a directory
            // We should be able to test if the pathbuffer endswith ofx.bundle in standard ascii, not necesseraly in utf8
            if entry.file_name().to_str().unwrap().ends_with(".ofx.bundle") {
                return true;
            } else {
                return false;
            }
        }
        Err(_) => {
            return false;
        }
    }
}

/// TODO
fn bundle_path(dir: Result<DirEntry>) -> String {
     "test".to_string()
}

pub fn init_bundles(bundle_paths: Vec<PathBuf>) -> Vec<Bundle> {

    let mut bundles : Vec<Bundle> = Vec::new();

    for path in bundle_paths {
        match path.as_path().read_dir() {
            Ok(entries) => { 
                for d_entry in entries {
                    // TODO create bundle from here
                    // Like match OfxBundle::new(d_entry) {
                    //}
                    if is_ofx_bundle(&d_entry) {
                       println!("dir '{:?}' has bundle", path.as_path());
                       //println!("entry '{:?}'", entry.path());
                       // TODO read nb_plugins
                       bundles.push(Bundle {root: bundle_path(d_entry), nb_plugins: 1}) 
                    }
                }
            }
            Err(k) => {
                println!("error '{}'", k);
            }
        }
    }
    
    return bundles;
}
