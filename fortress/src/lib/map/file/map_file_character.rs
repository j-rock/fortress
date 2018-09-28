use app::StatusOr;
use std::path::PathBuf;

pub enum MapFileCharacter {
    Wall,
    Buff,
    Spawn,
}

impl MapFileCharacter {
    pub fn parse_byte(path: &PathBuf, line: usize, col: usize, byte: u8) -> StatusOr<Option<MapFileCharacter>> {
        match byte {
            b'X' => Ok(Some(MapFileCharacter::Wall)),
            b'B' => Ok(Some(MapFileCharacter::Buff)),
            b'@' => Ok(Some(MapFileCharacter::Spawn)),
            b' ' => Ok(None),
            _ => Err(format!("Bad character at Line {}, Col {} of {:?}", line, col, path))
        }
    }
}
