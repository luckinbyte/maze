mod grid;

fn main() {
    let row = 10;
    let col = 10;
    let mut g = grid::Guid::new(row,col);
    //g.binary_tree_rand();
    g.sidewinder_rand();
    g.draw();
}


