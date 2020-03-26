use crate::{
    app::StatusOr,
    file::Config,
    render::{
        AllPackedSpriteSheets,
        BakedSpriteSheetConfig,
        SpriteSheetConfig,
    },
};
use std::path::PathBuf;

struct InputDirectories {
    config: PathBuf,
    images: PathBuf,
}

impl InputDirectories {
    pub fn new(project_root: PathBuf) -> StatusOr<Self> {
        let resource_base = project_root
            .join("res")
            .canonicalize()
            .map_err(|e| format! ("Couldn't canonicalize resource base: {}", e))?;

        Ok(InputDirectories {
            config: resource_base.join("config"),
            images: resource_base.join("images"),
        })
    }
}

struct OutputDirectories {
    config: PathBuf,
    images: PathBuf,
}

impl OutputDirectories {
    pub fn new() -> StatusOr<Self> {
        let output_dir_string = std::env::var("OUT_DIR")
            .map_err(|e| format!("{:?}", e))?;
        let output_dir = PathBuf::from(output_dir_string);

        let config = output_dir.join("config");
        let images = output_dir.join("images");

        for directory in [&config, &images].iter() {
            std::fs::create_dir_all(directory.clone())
                .map_err(|e| format!("{:?}", e))?;
        }

        Ok(OutputDirectories {
            config,
            images,
        })
    }
}

fn save_sprite_sheets(input: &InputDirectories, output: &OutputDirectories) -> StatusOr<()> {
    let sprite_sheet_config = SpriteSheetConfig::from_path(&input.config.join("sprite_sheet.conf"))?;
    let all_packed_sprite_sheets = AllPackedSpriteSheets::read_from_files(&sprite_sheet_config, &input.images)?;

    for (named_sprite_sheet, (image, _style)) in all_packed_sprite_sheets.images.iter() {
        let image_filepath = output.images.join(format!("{}.png", named_sprite_sheet.to_directory_basename()));
        image.save_to_file(image_filepath)?;
    }

    let baked_config_filepath = output.config.join("sprite_sheet.conf");
    let baked_config = BakedSpriteSheetConfig::new(sprite_sheet_config, all_packed_sprite_sheets.frames);
    let baked_config_json = serde_json::to_string(&baked_config)
        .map_err(|e| format!("{:?}", e))?;
    std::fs::write(baked_config_filepath, baked_config_json)
        .map_err(|e| format!("{:?}", e))?;

    Ok(())
}

pub fn run(project_root: PathBuf) -> StatusOr<()> {
    let input = InputDirectories::new(project_root)?;
    let output = OutputDirectories::new()?;
    save_sprite_sheets(&input, &output)?;
    Ok(())
}
