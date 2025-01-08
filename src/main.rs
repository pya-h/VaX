use VaX::vax::*;

fn main() {
    let i: f32 = 10.2;
    let x: Variable<f32> = i.plus(2.2);
    let z = x.plus(i);
    let q = i.multiply_by(&z);
    let qc = q.clone();
    let w = i.multiply_by(q); // q will be moved
    println!("i={}\tx={}\tz={}\tq={}\tw={}", i, x, z, qc, w);

    // TODO: Variable::new(3).plus(3.2); => Error
    // TODO: 2.minus1.1)

    // TODO: String operations
}
