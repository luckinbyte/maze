mod grid;

fn main() {
    let mut g = grid::Guid::new(5,5);
    g.link(1, 1, 1);
    g.link(0, 0, 1);
    g.draw();
}

