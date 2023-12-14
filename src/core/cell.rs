#[derive(Clone, Debug)]
pub struct Cell {
    pub collapsed: bool,
    pub sockets: Vec<usize>,
}

impl Cell {
    pub fn from_value(value: usize) -> Cell {
        Cell {
            collapsed: false,
            sockets: (0..value).collect(),
        }
    }

    pub fn from_list(value: Vec<usize>) -> Cell {
        // 空
        // println!("value: {:?}", value);
        Cell {
            collapsed: false,
            sockets: value,
        }
    }
}
