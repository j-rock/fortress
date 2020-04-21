use crate::{
    app::StatusOr,
    bake::{
        InputDirectories,
        InputOutput,
        OutputDirectories,
    },
    file::Config,
    render::{
        AllPackedSpriteSheets,
        BakedSpriteSheetConfig,
        SpriteSheetConfig,
    },
};

fn save_sprite_sheets(input: &InputDirectories, output: &OutputDirectories) -> StatusOr<()> {
    let sprite_sheet_config = SpriteSheetConfig::from_path(&input.config.join("sprite_sheet.conf"))?;
    let all_packed_sprite_sheets = AllPackedSpriteSheets::read_from_files(&sprite_sheet_config, &input.images)?;

    for (named_sprite_sheet, (image, _style)) in all_packed_sprite_sheets.images.iter() {
        let image_filepath = output.images.join(format!("{}.png", named_sprite_sheet.to_directory_basename()));
        image.save_to_file(image_filepath)?;
    }

    let baked_config_filepath = output.config.join("sprite_sheet.conf");
    let baked_config = BakedSpriteSheetConfig::new(sprite_sheet_config, all_packed_sprite_sheets.frames);
    let baked_config_string = ron::ser::to_string(&baked_config)
        .map_err(|e| format!("{:?}", e))?;
    std::fs::write(baked_config_filepath, baked_config_string)
        .map_err(|e| format!("{:?}", e))?;

    Ok(())
}

pub fn run(input_output: InputOutput) -> StatusOr<()> {
    save_sprite_sheets(&input_output.input, &input_output.output)?;
    Ok(())
}
