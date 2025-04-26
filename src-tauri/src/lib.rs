use regex::Regex;

mod utils;
use utils::{get_mod_manifest, get_mod_latest_version};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[warn(dead_code)]
struct Mod {
    id: String,
    name: String,
    version: String, 
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn check_mods(directory: String) -> std::result::Result<String, String> {
    // 读取目录下的所有文件夹下面的 manifest.json 文件
    let mut mods:Vec<Mod> = Vec::new();
    for entry in std::fs::read_dir(directory).unwrap() {
        let result = get_mod_manifest(entry.unwrap().path().to_str().unwrap()).await.unwrap();
        // println!("Found mod: {}", result);
        // 使用正则表达式匹配 mod 的id、名称和版本号
        let name_re = Regex::new(r#""Name":\s*"(.*?)"\s*,"#).unwrap();
        let id_re = Regex::new(r#""[N|n]exus:(.*?)""#).unwrap();
        let version_re = Regex::new(r#""Version": "(.*?)""#).unwrap();

        let name = name_re.captures(&result).map(|c| c.get(1).unwrap().as_str());
        let id = id_re.captures(&result).map(|c| c.get(1).unwrap().as_str());
        let version = version_re.captures(&result).map(|c| c.get(1).unwrap().as_str());

        println!("Found mod: {} {} {}", name.unwrap_or(""), id.unwrap_or(""), version.unwrap_or(""));
        if name.is_some() && id.is_some() && version.is_some() {
            mods.push(Mod {
                id: id.unwrap().to_string(),
                name: name.unwrap().to_string(),
                version: version.unwrap().to_string(),
            });
        }
    }

    for mod_ in mods.iter() {
        let (fid, version) = get_mod_latest_version(&mod_.id).await.unwrap();
        println!("name: {} current version {} latest version {} fid {}", mod_.name, mod_.version, version, fid);
    }
    println!("mods: {:?}", mods.len());

    Ok("mods checked".into())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, check_mods])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
