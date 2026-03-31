#[derive(Debug, Clone, Copy)]
pub enum Hemisphere {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Hemisphere {
    pub fn find_for_region(name: &str) -> Option<Self> {
        match name {
            "Western_Sahara" => Some(Hemisphere::NorthWest),
            "Djibouti" => Some(Hemisphere::NorthEast),
            "Qatar" => Some(Hemisphere::NorthEast),
            "Uruguay" => Some(Hemisphere::SouthWest),
            "Zimbabwe" => Some(Hemisphere::SouthEast),
            "Canada" => Some(Hemisphere::NorthWest),
            "Oman" => Some(Hemisphere::NorthEast),
            "Tanzania" => Some(Hemisphere::SouthEast),
            "Egypt" => Some(Hemisphere::NorthEast),
            "Ireland" => Some(Hemisphere::NorthWest),
            _ => None,
        }
    }

    pub fn signs(&self) -> (f64, f64) {
        match self {
            Hemisphere::NorthEast => (1.0, 1.0),
            Hemisphere::NorthWest => (-1.0, 1.0),
            Hemisphere::SouthEast => (1.0, -1.0),
            Hemisphere::SouthWest => (-1.0, -1.0),
        }
    }
}
