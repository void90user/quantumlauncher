use serde::{Deserialize, Serialize};


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
    #[serde(rename = "fabric-loader")]
    fabric_loader: String,
}

fn main(name: String, summary: String, minecraft_version: String, loader_version: String, paths: Vec<&str>, sha1: Vec<&str>, sha512: Vec<&str>, links: Vec<&str>, file_size: Vec<u64>) -> Result<String, Box<dyn std::error::Error>> {

    let name = name;
    let summary = summary;
    let minecraft_version = minecraft_version;
    let loader_version = loader_version;
    let paths = paths;
    let sha1 = sha1;
    let sha512 = sha512;
    let links = links;
    let file_sizes = file_size;


    let files: Vec<ModrinthFileEntry> = paths
        .iter()
        .zip(&sha1)
        .zip(&sha512)
        .zip(&links)
        .zip(&file_sizes)
        .map(|((((&path, &sha1), &sha512), &download), &file_size)| ModrinthFileEntry {
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
        versionId: "1".to_string(),
        name,
        summary,
        files,
        dependencies: ModrinthDependencies {
            minecraft: minecraft_version,
            fabric_loader: loader_version,
        },
    };

    let json_data = serde_json::to_string_pretty(&manifest)?;

    Ok(json_data)
}

// fs::write("modrinth.index.json", json_data)?;
