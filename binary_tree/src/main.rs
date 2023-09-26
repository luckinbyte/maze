mod grid;

fn main() {
    let row = 6;
    let col = 6;
    let mut g = grid::Guid::new(row,col);
    g.binary_tree_rand();
    g.draw();
}


