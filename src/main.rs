use std::fs;
use std::env;


fn category(ext: &str) -> &str {
    match ext {
        // Images
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "ico" | "tiff" | "tif"
        | "heic" | "heif" | "raw" | "cr2" | "nef" | "psd" | "ai" | "xcf" | "kra" => "images",

        // Documents
        "txt" | "md" | "pdf" | "doc" | "docx" | "odt" | "rtf" | "tex" | "epub" | "mobi"
        | "azw3" | "djvu" => "docs",

        // Spreadsheets
        "xls" | "xlsx" | "ods" | "csv" | "tsv" => "spreadsheets",

        // Presentations
        "ppt" | "pptx" | "odp" | "key" => "presentations",

        // Code
        "js" | "ts" | "jsx" | "tsx" | "rs" | "py" | "c" | "h" | "cpp" | "hpp" | "cc" | "cs"
        | "java" | "kt" | "go" | "rb" | "php" | "swift" | "lua" | "sh" | "bash" | "zsh"
        | "html" | "css" | "scss" | "json" | "xml" | "yaml" | "yml" | "toml" | "sql" | "vue"
        | "svelte" | "zig" | "nix" | "conf" | "ini" => "code",

        // Audio
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "opus" | "m4a" | "wma" | "aiff" | "mid"
        | "midi" => "audio",

        // Video
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg"
        | "3gp" => "video",

        // Archives
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "zst" | "tgz" | "iso" => "archives",

        // Installers and packages
        "exe" | "msi" | "deb" | "rpm" | "appimage" | "apk" | "dmg" | "pkg" | "flatpak" => "installers",

        // Fonts
        "ttf" | "otf" | "woff" | "woff2" => "fonts",

        // 3D and design
        "blend" | "obj" | "fbx" | "stl" | "gltf" | "glb" | "dwg" | "dxf" => "3d",

        _ => "other",
    }
}

fn main() {
    if let Some(arg) = env::args().skip(1).next() {
        let base = &arg;
        // let entries = fs::read_dir(base);
        if let Ok(entries) = fs::read_dir(base){
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    continue;
            }

                match path.extension(){
                    Some(ext) => {
                        let ext  = &ext.to_str().unwrap().to_lowercase();
                        let label = category(ext);
                        let folder = format!("{}/{}", base, label);
                        let name = path.file_name().unwrap().to_str().unwrap();
                        let new_path = format!("{}/{}", folder, name);
                        fs::create_dir_all(&folder).unwrap();
                        fs::rename(&path, &new_path).unwrap();

                    },
                    None => {
                        let label = "other";
                        let folder = format!("{}/{}", base, label);
                        let name = path.file_name().unwrap().to_str().unwrap();
                        let new_path = format!("{}/{}",folder ,name);
                        fs::create_dir_all(&folder).unwrap();
                        fs::rename(&path, &new_path).unwrap();
                    }
                }
            }
        }

        else {
            println!("No such Directory exists")
        }



    }

    else {
        println!("provie some argument")
    }

}