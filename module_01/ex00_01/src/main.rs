use ex00_01::min;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_min(){
        assert_eq!(min(&12,&25), &12);
        assert_eq!(min(&1,&12), &1);
        assert_eq!(min(&17,&3), &3);
        assert_eq!(min(&100,&25), &25);
        assert_eq!(min(&0,&0), &0);
    }
}

fn main() {
    println!("Min : {:?}", min(&12, &34));
}
