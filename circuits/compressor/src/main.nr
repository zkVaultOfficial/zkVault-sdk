use dep::std;

fn main(input: [Field; 4], compressed: pub Field) {
    let sum = input[0] + input[1] + input[2] + input[3];
    assert(sum == compressed);
}