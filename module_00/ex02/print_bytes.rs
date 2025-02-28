fn print_bytes(s: &str){
    println!("Entrée : \n {}", s);
    
    println!("");
    println!("Sortie : ");
    for i in s.bytes() {
        println!("{}", i);
    }
}

fn main() {
    print_bytes("Hola que tal\n");

    print_bytes("Déjà Vu\n");
}