fn min(a: i32, b: i32) -> i32 {
    if a < b{
        println!("A est inférieure ou égale a b :{}", a);
        a
    }
    else {
        println!("Bon bah B est inférieur ... : {}", b);
        b
    }
}

fn main() {
    min(31, 21);
    min(56, 2);
    min(-3, 56);
}