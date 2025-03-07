use ex01_01::color_name;

#[cfg(test)]
#[test]
fn test_lifetimes() {
    let name_of_the_best_color;

    {
        let the_best_color = [30, 230, 30];
        name_of_the_best_color = color_name(&the_best_color);
    }

    assert_eq!(name_of_the_best_color, "Greenish");
}

fn main() {
    println!("Hello, world!");
    let pure_black: [u8; 3] = [0, 0, 0];
    println!("Voici les couleurs : {:?}", pure_black);
    println!("Result : {}",color_name(&pure_black));
}
