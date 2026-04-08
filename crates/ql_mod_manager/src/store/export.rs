use std::collections::HashSet;
use serde::Serialize;
use ql_core::InstanceSelection;
use ql_core::json::VersionDetails;
use crate::store::{ModId, ModIndex};
use std::fs;
use std::path::{PathBuf};
use sha1::{Sha1, Digest};
use sha2::{ Sha512};
use hex;
use std::io::{Result};


#[derive(Serialize)]
pub struct ModrinthFileEntry {
    path: String,
    hashes: Hashes,
    downloads: Vec<String>,
    filesize: u64,
}

#[derive(Serialize)]
pub struct Hashes {
    sha1: String,
    sha512: String,
}

#[derive(Serialize)]
pub struct ModrinthModpackManifest {
    formatversion: u32,
    game: String,
    versionid: String,
    name: String,
    summary: String,
    files: Vec<ModrinthFileEntry>,
    dependencies: ModrinthDependencies,
}

#[derive(Serialize)]
pub struct ModrinthDependencies {
    minecraft: String,
    loader_id: String,
}

pub async fn export_modrinth_modpack(modpack_name: String,modpack_version: String, modpack_summary: String,modpack_file_name: String, mod_ids: HashSet<ModId>, overrides: Vec<String>, instance: InstanceSelection) {
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


    let details = VersionDetails::load(&instance).await.unwrap();
    let minecraft_version = details.get_id();
    let config = ql_core::InstanceConfigJson::read(&instance).await;
    let loader_name = config.unwrap().mod_type.to_modrinth_str();
    let config = ql_core::InstanceConfigJson::read(&instance).await;
    let loader_version = config.unwrap().mod_type_info.unwrap().version;
    let loader = loader_name.to_string() + ":" + loader_version.unwrap().as_str();

    let minecraft_path = instance.get_dot_minecraft_path();

    let full_path: Vec<PathBuf> = paths
        .iter()
        .map(|rel_path| minecraft_path.join(rel_path))
        .collect();

    let file_sizes: Vec<u64> = full_path
        .iter()
        .map(|path| fs::metadata(path).map(|meta| meta.len()).unwrap_or(0))
        .collect();


    let sha1s: Vec<String> = full_path
        .clone()
        .into_iter()
        .map(|path| {
            let data = std::fs::read(path).unwrap();
            let mut hasher = Sha1::new();
            hasher.update(&data);
            let hash = hasher.finalize();
            hex::encode(hash)
        })
        .collect();


    let sha512s: Vec<String> = full_path
        .into_iter()
        .map(|path| {
            let data = std::fs::read(path).unwrap();
            let mut hasher = Sha512::new();
            hasher.update(&data);
            let hash = hasher.finalize();
            hex::encode(hash)
        })
        .collect();

    let json_data = create_modrinth_index_json(modpack_name, modpack_version, modpack_summary, loader, minecraft_version.to_string(), paths, sha1s, sha512s, urls, file_sizes).unwrap();
}



fn create_modrinth_index_json(modpack_name: String,modpack_version: String, modpack_summary: String,loader: String, minecraft_version: String, paths: Vec<String>, sha1: Vec<String>, sha512: Vec<String>, links: Vec<String>, file_size: Vec<u64>) -> Result<String> {

    let name = modpack_name;
    let summary = modpack_summary;
    let sha1: Vec<&str> = sha1.iter().map(|s| s.as_str()).collect();
    let sha512: Vec<&str> = sha512.iter().map(|s| s.as_str()).collect();



    let files: Vec<ModrinthFileEntry> = paths
        .iter()
        .zip(&sha1)
        .zip(&sha512)
        .zip(&links)
        .zip(&file_size)
        .map(|((((path, &sha1), &sha512), download), &file_size)| ModrinthFileEntry {
            path: path.to_string(),
            hashes: Hashes {
                sha1: sha1.to_string(),
                sha512: sha512.to_string(),
            },
            downloads: vec![download.to_string()],
            filesize: file_size,
        })
        .collect();


    let manifest = ModrinthModpackManifest {
        formatversion: 1,
        game: "minecraft".to_string(),
        versionid: modpack_version,
        name,
        summary,
        files,
        dependencies: ModrinthDependencies {
            minecraft: minecraft_version,
            loader_id: loader,
        },
    };

    let json_data = serde_json::to_string_pretty(&manifest)?;

    Ok(json_data)
}

#[derive(Serialize)]
struct CurseForgeModpackManifest {
    minecraft: CurseForgeMinecraftConfig,
    manifest_type: String,
    manifest_version: u32,
    name: String,
    version: String,
    author: String,
    files: Vec<CurseForgeFileEntry>,
    overrides: String,
}

#[derive(Serialize)]
struct CurseForgeMinecraftConfig {
    version: String,
    mod_loaders: Vec<CurseForgeModLoader>,
}

#[derive(Serialize)]
struct CurseForgeModLoader {
    id: String,
    primary: bool,
}

#[derive(Serialize)]
struct CurseForgeFileEntry {
    project_id: u64,
    file_id: u64,
    required: bool,
}
/*
pub async fn export_curseforge_modpack(modpack_name: String,modpack_version: String, modpack_summary: String,modpack_file_name: String, mod_ids: HashSet<ModId>, overrides: Vec<String>, instance: InstanceSelection) {

    let details = VersionDetails::load(&instance).await.unwrap();
    let minecraft_version = details.get_id();
    let config = ql_core::InstanceConfigJson::read(&instance).await;
    let loader_name = config.unwrap().mod_type.to_modrinth_str();
    let config = ql_core::InstanceConfigJson::read(&instance).await;
    let loader_version = config.unwrap().mod_type_info.unwrap().version;
    let loader = loader_name.to_string() + ":" + loader_version.unwrap().as_str();

}

 */

fn write_curseforge_manifest_json(mod_id: Vec<&str>, file_id: Vec<&str>, author: String, modpack_version: String, name: String, loader_id: String, minecraft_version: String, ) -> Result<String> {

    let primary = true;

    let files: Vec<CurseForgeFileEntry> = mod_id
        .into_iter()
        .zip(file_id.into_iter())
        .map(|(proj_str, file_str)| CurseForgeFileEntry {
            project_id: proj_str.parse::<u64>().expect("Invalid project ID"),
            file_id: file_str.parse::<u64>().expect("Invalid file ID"),
            required: true,
        })
        .collect();

    let manifest = CurseForgeModpackManifest {
        minecraft: CurseForgeMinecraftConfig {
            version: minecraft_version,
            mod_loaders: vec![CurseForgeModLoader { id: loader_id, primary }],
        },
        manifest_type: "minecraftModpack".to_string(),
        manifest_version: 1,
        name,
        version: modpack_version,
        author,
        files,
        overrides: "overrides".to_string(),
    };

    let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
    Ok(manifest_json)
}