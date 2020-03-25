use crate::app::StatusOr;
use std::path::PathBuf;

#[cfg(feature = "bake")]
use crate::render::NamedSpriteSheet;
#[cfg(feature = "bake")]
use std::collections::HashMap;
#[cfg(feature = "bake")]
use walkdir::WalkDir;

#[cfg(feature = "bake")]
fn resource_base(project_root: PathBuf) -> StatusOr<PathBuf> {
    let mut path = project_root;
    path.push("fortress_bake");
    path.push("res");
    path
        .canonicalize()
        .map_err(|e| format! ("Couldn't canonicalize resource base: {}", e))
}

#[cfg(feature = "bake")]
fn relevant_sprite_sheet_paths(images_dir: &PathBuf) -> HashMap<NamedSpriteSheet, PathBuf> {
    WalkDir::new(images_dir)
        // Just browse directories inside images/.
        .max_depth(1)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.path().file_name()?.to_str()?;
            let quoted_file_name = format!("\"{}\"", file_name);
            let named_sprite_sheet = NamedSpriteSheet::from_str(&quoted_file_name)?;
            Some((named_sprite_sheet, entry.into_path()))
        })
        .collect()
}

#[cfg(feature = "bake")]
fn mark_directory_contents_recursively_for_rerun_if(path: &PathBuf) {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path_string = entry.path().to_str()?;
            if path_string.contains(".png") {
                Some(entry.into_path())
            } else {
                None
            }
        })
        .for_each(|path| {
            println!("cargo:rerun-if-changed={:?}", path);
        });
}

#[cfg(feature = "bake")]
pub fn run(project_root: PathBuf) -> StatusOr<()> {
    let resource_base = resource_base(project_root)?;
    let images_dir = resource_base.join("images");

    let sprite_sheet_paths = relevant_sprite_sheet_paths(&images_dir);
    for path in sprite_sheet_paths.values() {
        mark_directory_contents_recursively_for_rerun_if(path);
    }

    let out_dir = std::env::var("OUT_DIR")
        .map_err(|e| format!("{:?}", e))?;
    println!("OutDir: {}", out_dir);

    Ok(())
}

#[cfg(not(feature = "bake"))]
pub fn run(_project_root: PathBuf) -> StatusOr<()> {
    Ok(())
}
