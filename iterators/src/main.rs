pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3,4];
    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), Some(4));
    assert_eq!(v1_iter.next(), None);
}

fn main() {
    let v1 = vec![1, 2, 3, 4];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("{}", value);
    }

    // methods that produce other iterators
    let i1 = vec![1,2,3];
    let i2: Vec<_> = i1.iter().map(|x| x + 1).collect();

    assert_eq!(i2, vec![2,3,4]);
}


// Closure that captures Their environment
#[derive(PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size(){
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
            Shoe {
                size: 11,
                style: String::from("sports shoe"),
            },
            Shoe {
                size: 10,
                style: String::from("office shoe"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);
        let expected_shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
    
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
      
            Shoe {
                size: 10,
                style: String::from("office shoe"),
            },
        ];
        assert_eq!(in_my_size, expected_shoes);
    }
}
