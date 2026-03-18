use std::fs;
use std::path::Path;
use serde_json::Value::String;
use crate::LAUNCHER_DIR;

async fn create_hard_links(links: Vec<(&str, &str)>) -> std::io::Result<()> {
    for (from, to) in links {
        fs::hard_link(from, to)?;
    }
    Ok(())
}


pub async fn file_linking() {

    let file_type;
    //TODO: ADD THE FILE TYPE LIKE: resourcepack, config ect.
    let file_names;
    //TODO: ADD THE FILE NAMES

    let global_source_path = LAUNCHER_DIR.join("global_resources");
    let sources: Vec<&str> = vec![] ;
    let destinations: Vec<&str> = vec![];

    sources.join("as");
    destinations.join("as");

    let paths: Vec<(&str, &str)> = sources.into_iter().zip(destinations.into_iter()).collect();

    create_hard_links(paths).await.expect("Unable to create links");

}