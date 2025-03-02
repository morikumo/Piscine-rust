use ftkit;
use std::i32;
use std::cmp::Ordering::*;

// vu que on a pas le droit au opérateur de comparaison on va utiliser :
// cmp library
// Ex: if (num.cmp(&devine), Ordering::Less)
// Ex: if (num.cmp(&devine), Ordering::Greater)
// Ex: if (num.cmp(&devine), Ordering::Equal)

fn main() {
    let devine:i32 = ftkit::random_number(1..=20);
    
    println!("VOYONS VOIR !");
    loop {
        let num = ftkit::read_number();
        
        match num.cmp(&devine) {
            Greater => println!("Notre nombre {}. Est plus grand que celui rechercher", num),
            Less => println!("Notre nombre : {}. Est plus petit que celui rechercher", num),
            Equal => break,
        }    
    }
    println!("Le bon nombre est : {}", devine)
}
