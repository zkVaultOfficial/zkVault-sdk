use dep::std;

fn main(x: Field, y: pub Field) {
    let hash = std::hash::pedersen([x]);
    assert(hash[0] == y);
}