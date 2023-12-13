pub struct Cell {
    pub collapsed: bool,
    pub sockets: Vec<i32>,
}

impl Cell {
    fn from_value(value: i32) -> Cell {
        Cell {
            collapsed: false,
            sockets: (0..value).collect(),
        }
    }

    fn from_list(value: Vec<i32>) -> Cell {
        Cell {
            collapsed: false,
            sockets: value,
        }
    }
}
