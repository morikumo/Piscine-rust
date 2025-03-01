fn main() {
    let x: u8 = 255;
    let y: u8 = 1;
    
    // Cette opération entraîne un dépassement d'entier en mode "dev"
    let result = x + y;
    
    println!("{}u8 + {}u8 == {}", x, y, result);
}
