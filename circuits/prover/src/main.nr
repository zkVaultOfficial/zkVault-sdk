use dep::std;

fn main(secret: Field, public_hash: pub Field) {
    let computed_hash = std::hash::poseidon::bn254::hash_1(secret);
    assert(computed_hash == public_hash);
}