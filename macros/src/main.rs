macro_rules! add {
    ($x:expr, $y:expr) => {
        $x + $y
    };
    ($x:expr, $y:expr, $z:expr) => {
        $x + $y + $z
    };
}

fn main() {
    let x = 10;
    let y = 20;
    let z = add!(x, y);
    println!("The sum of {} and {} = {}", x, y, z);
    let a = 30;
    let b = 40;
    let c = add!(a, b, z);
    println!("The sum of {}, {} and {} = {}", a, b, z, c);
}

