fn collatz(start: u32){
    println!("Entrée :");
    println!("{}", start);
    println!(" ");
    println!("Sortie :");
    
    let mut go = start;

    while go != 1 {
        if go % 2 == 0{
            go /= 2;
            println!("{:?}",go);
        }
        else {
            go = (3*go) + 1;
            println!("{:?}",go);
        }
    }
}

fn main(){
    println!("Première approche : ");
    collatz(32);
    
    println!("Deuxième approche : ");
    collatz(37);
}