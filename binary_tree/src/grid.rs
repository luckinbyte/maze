
//pub mod grid{
    pub struct Guid{
        row :u32,
        col :u32,
        cells :Vec<Cell>,
    }
    pub fn new(row:u32, col:u32) -> Guid{
        let mut cells = Vec::with_capacity((row*col).try_into().unwrap());
        for i in 1..row{
            for j in 1..col{
                cells.push(Cell::new(i,j))
            }
        }
        Guid{row, col, cells}
    }

    // impl Guid{
    //     pub fn new(row:u32, col:u32) -> Guid{
    //         let mut cells = Vec::with_capacity((row*col).try_into().unwrap());
    //         for i in 1..row{
    //             for j in 1..col{
    //                 cells.push(Cell::new(i,j))
    //             }
    //         }
    //         Guid{row, col, cells}
    //     }
    // }

    struct Cell{
        row :u32,
        col :u32,
        north :bool,
        south :bool,
        east :bool,
        west :bool,
    }
    
    impl Cell{
        fn new(row:u32, col:u32) -> Cell{
            Cell{row, col, north:false, south:false, east:false, west:false}
        }
    }
//}


