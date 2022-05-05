pub struct ArtSymbol {
    // width: usize,
    // height: usize,
    data: String,
}

impl ArtSymbol {
    pub fn new(data: &str) -> ArtSymbol {
        // let width: usize = data.lines().max_by(|line| line.len());
        // let height: usize = data.matches('\n').count();
        ArtSymbol {
            // width,
            // height,
            data: String::from(data),
        }
    }
    // pub fn width(&self) -> usize {
    //     width
    // }
    // pub fn height(&self) -> usize {
    //     height
    // }
    pub fn data(&self) -> &str {
        &self.data
    }
}
