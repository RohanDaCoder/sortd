use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    folder: PathBuf,

    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    let folder = args.folder;
    println!("🔍 Scanning folder: {}", folder.display());

    let files_in_dir = match fs::read_dir(&folder) {
        Ok(dir) => dir,
        Err(e) => {
            println!("❌ Error: Could not read folder '{}'.", folder.display());
            println!("Reason: {}", e);
            return;
        }
    };

    for entry in files_in_dir {
        if let Ok(valid_entry) = entry {
            let path = valid_entry.path();
            if path.is_file() {
                let file_name = &path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                let ext = &path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("no_extension")
                    .to_lowercase();

                // Edge Cases
                if file_name.starts_with(".") {
                    continue;
                }
                if matches!(ext.as_str(), "ini" | "pfx" | "lnk" | "tmp" | "crdownload") {
                    println!("Ignored: {:?}", path.file_name().unwrap());
                    continue;
                }

                // Variables
                let category = match ext.as_str() {
                    "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" => "Images",
                    "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" => "Video",
                    "mp3" | "wav" | "flac" | "aac" | "ogg" => "Music",
                    "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" => "Documents",
                    "zip" | "rar" | "7z" | "tar" | "gz" | "xz" => "Compressed",
                    "exe" | "msi" | "deb" | "rpm" => "Programs",
                    _ => "Other",
                };
                println!("File name: {} \nCategory: {}", path.display(), category);

                // Create the folder
                let target_dir = folder.join(category);
                let target_path = &target_dir.join(path.file_name().unwrap());

                if !target_dir.exists() {
                    // Category folder does not exist, so create it
                    if let Err(e) = fs::create_dir_all(&target_path) {
                        eprintln!("Failed to create folder '{}': {}", &category, e);
                        continue;
                    };
                }

                if !args.dry_run {
                    // Not a dry run, so move files
                    match fs::rename(&path, &target_path) {
                        Ok(_) => {
                            println!("✅ Moved: {:?} → {}/", &path.file_name().unwrap(), category);
                        }
                        Err(e) => {
                            eprintln!(
                                "❌ Failed to move '{:?}': {}",
                                &path.file_name().unwrap(),
                                e
                            )
                        }
                    }
                } else {
                    println!("Dry run. Files won't get moved.")
                }
            }
        }
    }
}
