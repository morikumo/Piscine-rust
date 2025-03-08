use ex01_03::largest_group;

#[test]
#[cfg(test)]
fn test_lifetimes() {
    let haystack = [1, 2, 3, 2, 1];
    let result;

    {
        let needle = [2, 3];
        result = largest_group(&haystack, &needle);
    }

    assert_eq!(result, &[2, 3, 2]);
}

fn main() {
    println!("Hello, world!");
    println!("Largest :: {:?}", largest_group(&[1,3,4,3,5,5,4], &[5,3]));
    println!("Go!");

}
