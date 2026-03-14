use symlink_rs;
use std::fs::metadata;
use symlink_rs::{symlink_file, symlink_dir};
use std::path::Path;

pub async fn create_syslink<P: AsRef<Path>>(targets: &[(P, P)]) -> std::io::Result<()> {

    for (target, link) in targets {
        let target_path = target.as_ref();
        let link_path = link.as_ref();
        let metadata = metadata(target_path)?;

        if metadata.is_dir() {
            symlink_dir(target_path, link_path)?;
        } else {
            symlink_file(target_path, link_path)?;
        }
    }

    Ok(())

}


// FOR FRONTEND







/*
files_to_link.push(("config.toml", "cfg_link.toml"));
files_to_link.push(("data_dir", "link_to_data"));

create_symlinks(&files_to_link)?;
let mut files_to_link = Vec::new();



let file_paths = vec!["src/file1.txt", "src/file2.txt"];
let link_paths = vec!["links/l1.txt", "links/l2.txt"];

let targets: Vec<_> = file_paths.into_iter().zip(link_paths.into_iter()).collect();

 */