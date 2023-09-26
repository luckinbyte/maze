use rand::Rng;
use std::collections::HashMap;

pub struct Guid{
    row :u32,
    col :u32,
    cells :Vec<Cell>,
    links :Vec<(u32,u32)>,
}
impl Guid {
    pub fn new(row:u32, col:u32) -> Guid{
        let mut cells = Vec::with_capacity((row*col).try_into().unwrap());
        for i in 0..row{
            for j in 0..col{
                cells.push(Cell::new(i,j))
            }
        }
        let links = Vec::new();
        Guid{row, col, cells, links}
    }

    pub fn binary_tree_rand(&mut self) -> (){
        for i in 0..self.col-1{
            self.link(self.row-1, i, 1);
        }
        for i in 0..self.row-1{
            self.link(i, self.col-1, 4);
        }
        for i in 0..self.row-1{
            for j in 0..self.col-1{
                if !self.is_link(i,j){
                    let direction = rand::thread_rng().gen_range(1..3);
                    if direction==1{
                        self.link(i, j, direction);
                    }else{
                        self.link(i, j, 4);
                    }
                    
                }
            }
        }
    }

    pub fn sidewinder_rand(&mut self) -> (){
        for i in 0..self.col-1{
            self.link(self.row-1, i, 1);
        }
        for i in 0..self.row-1{
            let mut to_north:Vec<(u32,u32)> = Vec::new();
            let mut to_rand_len = 0;
            for j in 0..self.col{
                let direction = rand::thread_rng().gen_range(1..3);
                match direction{
                    1 if (j!=self.col-1) =>{
                        self.link(i, j, 1);
                        to_north.push((i,j));
                        to_rand_len += 1;
                    }
                    _ =>{
                        to_north.push((i,j));
                        to_rand_len += 1;
                        let rand_num = rand::thread_rng().gen_range(0..to_rand_len);
                        let (northi, northj) = to_north[rand_num];
                        self.link(northi, northj, 4);
                        to_north = Vec::new();
                        to_rand_len = 0;
                    }
                }
            }
        }
    }

    fn is_link(&self, row:u32, col:u32) -> bool{
        self.links.contains(&(row,col))
    }

    fn link(&mut self, row:u32, col:u32, direction:u32) {
        self.links.push((row,col));
        match direction{
            1 => {
                self.cells.get_mut((row*self.col+col) as usize).unwrap().link(1);
                self.cells.get_mut((row*self.col+col+1) as usize).unwrap().link(3);
            },
            2 => {
                self.cells.get_mut((row*self.col+col) as usize).unwrap().link(2);
                self.cells.get_mut((row*self.col+col-self.col) as usize).unwrap().link(4);                
            },
            3 => {
                self.cells.get_mut((row*self.col+col) as usize).unwrap().link(3);
                self.cells.get_mut((row*self.col+col-1) as usize).unwrap().link(1);                
            }
            _ =>{
                self.cells.get_mut((row*self.col+col) as usize).unwrap().link(4);
                self.cells.get_mut((row*self.col+col+self.col) as usize).unwrap().link(2);
            }
        }
    }

    pub fn draw(&self) {
        let mut i = self.row;  
        for j in 0..self.col{
            print!("+---");
        }
        print!("+\n");
        while i>0{
            for j in 0..self.col{
                self.cells.get(((i-1)*self.col+j) as usize).unwrap().draw_l();
            }
            print!("|\n");
            for j in 0..self.col{
                self.cells.get(((i-1)*self.col+j) as usize).unwrap().draw_d();
            }
            print!("+\n");
            i = i-1;
        }
    }
}

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

    fn link(&mut self, dir:u32) -> () {
        match dir{
            1 => self.east = true,
            2 => self.south = true,
            3 => self.west = true,
            _ => self.north = true
        };
    }

    fn draw_l(&self) -> () {
        if self.west{
            print!("    ");
        }
        else{
            print!("|   ");
        }   
    }

    fn draw_d(&self) -> () {
        if self.south{
            print!("    ");
        }
        else{
            print!("+---");
        } 
    }  
}


