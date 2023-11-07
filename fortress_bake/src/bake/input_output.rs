use crate::app::StatusOr;
use std::path::PathBuf;

pub struct InputOutput {
    pub input: InputDirectories,
    pub output: OutputDirectories,
}

impl InputOutput {
    pub fn new(project_root: PathBuf) -> StatusOr<Self> {
        let input = InputDirectories::new(project_root)?;
        let output = OutputDirectories::new()?;

        Ok(InputOutput {
            input,
            output,
        })
    }
}

pub struct InputDirectories {
    pub config: PathBuf,
    pub images: PathBuf,
    pub fonts: PathBuf,
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
            fonts: resource_base.join("fonts"),
        })
    }
}

pub struct OutputDirectories {
    pub config: PathBuf,
    pub images: PathBuf,
    pub fonts: PathBuf,
}

impl OutputDirectories {
    pub fn new() -> StatusOr<Self> {
        let output_dir_string = std::env::var("OUT_DIR")
            .map_err(|e| format!("{:?}", e))?;
        let output_dir = PathBuf::from(output_dir_string);

        let config = output_dir.join("config");
        let images = output_dir.join("images");
        let fonts = output_dir.join("fonts");

        for directory in [&config, &images, &fonts].into_iter() {
            std::fs::create_dir_all(directory)
                .map_err(|e| format!("{:?}", e))?;
        }

        Ok(OutputDirectories {
            config,
            images,
            fonts,
        })
    }
}
