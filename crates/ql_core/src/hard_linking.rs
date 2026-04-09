use std::path::Path;

use crate::{IoError, file_utils::create_symlink_async};

async fn create_hard_links(links: Vec<(&Path, &Path)>) -> Result<(), IoError> {
    for (from, to) in links {
        create_symlink_async(from, to).await?;
    }
    Ok(())
}

pub async fn file_linking() {
    /*
    TODO: This might help you do the frontend

    let file_type;
    //TODO: ADD THE FILE TYPE LIKE: resourcepack, config, shaderpack .
    let file_names;
    //TODO: ADD THE FILE NAMES

    let selected_global_source_path = LAUNCHER_DIR.join("global_resources").join(file_type);
    let instance_path:Path = "current instance path";
    // TODO: add the selected instance here ^^^^^^^
    let instance_resource_path;

    if file_type == "config" { instance_resource_path = instance_path.join(".minecraft") }
    else { instance_resource_path = instance_path.join(".minecraft").join(file_type) }

     */

    let sources: Vec<&Path> = vec![];
    let destinations: Vec<&Path> = vec![];
    // TODO: THESE NEED TO BE ABSOLUTE PATHS LIKE: sources -> ("SOURCE/FILE/PATH.zip", "SOURCE/FILE/PATH2.zip") dest -> ("DEST/FILE/PATH.zip", "DEST/FILE/PATH2.zip")

    let links: Vec<(&Path, &Path)> = sources
        .iter()
        .zip(destinations.iter())
        .map(|(&src, &dst)| (src, dst))
        .collect();

    create_hard_links(links)
        .await
        .expect("Unable to create links");
}
