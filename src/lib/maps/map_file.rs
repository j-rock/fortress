#[derive(Clone, Deserialize)]
pub struct GridCell {
    // Axial coordinates
    q: i64,
    r: i64
}

#[derive(Clone, Deserialize)]
pub struct MapFile {
    cells: Vec<GridCell>
}