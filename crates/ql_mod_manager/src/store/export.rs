use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tokio::task::id;
use ql_core::InstanceSelection;
use crate::store::{ModId, ModIndex};

#[derive(Serialize)]
pub struct ModrinthFileEntry {
    path: String,
    hashes: Hashes,
    downloads: Vec<String>,
    fileSize: u64,
}

#[derive(Serialize)]
pub struct Hashes {
    sha1: String,
    sha512: String,
}

#[derive(Serialize)]
pub struct ModrinthModpackManifest {
    formatVersion: u32,
    game: String,
    versionId: String,
    name: String,
    summary: String,
    files: Vec<ModrinthFileEntry>,
    dependencies: ModrinthDependencies,
}

#[derive(Serialize)]
pub struct ModrinthDependencies {
    minecraft: String,
    loader_Id: String,
}

async fn export_modpack(mod_ids: HashSet<ModId>, instance: InstanceSelection) {
    let index = ModIndex::load(&instance).await.unwrap();

    let mut urls: Vec<String> = Vec::new();
    let mut filenames: Vec<String> = Vec::new();

    for id in &mod_ids {
        let is_modrinth = matches!(id, ModId::Modrinth(_));
        if !is_modrinth {
            continue;
        }

        let Some(config) = index.mods.get(id) else {
            continue;
        };

        let Some(primary_file) = config.files
            .iter()
            .find(|file| file.primary)
            .or_else(|| config.files.first())
        else {
            continue;
        };

        urls.push(primary_file.url.clone());
        filenames.push(primary_file.filename.clone());
    }

    let paths: Vec<String> = filenames
        .iter()
        .map(|name| format!("mods/{}", name))
        .collect();

    let loader_raw =  instance.get_loader().await.unwrap().to_string();
    let loader = match loader_raw.as_str() {
         "Fabric" => {}
         "Forge" => {}
         "NeoForge" => {}
         "Quilt" => {}

        /*
        "LiteLoader".to_string() =>  {}
        "Rift".to_string() => {}
        "NilLoader".to_string() => {}
        "Orntihe" => {}
        "Babric" => {}
        "Legacy Fabric" => {}
         */

        _ => panic!()
    };

}


fn create_modrinth_index_json(modpack_name: String,modpack_version: String, modpack_summary: String,loader_type: String, minecraft_version: String, loader_version: String, paths: Vec<String>, sha1: Vec<&str>, sha512: Vec<&str>, links: Vec<String>, file_size: Vec<u64>) -> Result<String, Box<dyn std::error::Error>> {

    let name = modpack_name;
    let modpack_version = modpack_version;
    let summary = modpack_summary;
    let minecraft_version = minecraft_version;
    let loader_version = loader_version;
    let paths = paths;
    let sha1 = sha1;
    let sha512 = sha512;
    let links = links;
    let file_sizes = file_size;
    let loader_type = loader_type;
    let loader = loader_type + loader_version.as_str();


    let files: Vec<ModrinthFileEntry> = paths
        .iter()
        .zip(&sha1)
        .zip(&sha512)
        .zip(&links)
        .zip(&file_sizes)
        .map(|((((path, &sha1), &sha512), download), &file_size)| ModrinthFileEntry {
            path: path.to_string(),
            hashes: Hashes {
                sha1: sha1.to_string(),
                sha512: sha512.to_string(),
            },
            downloads: vec![download.to_string()],
            fileSize: file_size,
        })
        .collect();


    let manifest = ModrinthModpackManifest {
        formatVersion: 1,
        game: "minecraft".to_string(),
        versionId: modpack_version,
        name,
        summary,
        files,
        dependencies: ModrinthDependencies {
            minecraft: minecraft_version,
            loader_Id: loader,
        },
    };

    let json_data = serde_json::to_string_pretty(&manifest)?;

    Ok(json_data)
}

// fs::write("modrinth.index.json", json_data)?;


/*

fn create_curseforge_mainfest(mod_id: Vec<&str>, fileID: Vec<&str>,author: String, modpack_version: String, name: String, recommended_ram: u32, lodaer_id: String, minecraft_version: String)  {

    let minecraft_version = minecraft_version;
    let author = author;
    let modpack_version = modpack_version;
    let name = name;
    let recommended_ram = recommended_ram;
    let loader_id = lodaer_id;
    let projectID = mod_id;
    let fileID = fileID;
}

 */