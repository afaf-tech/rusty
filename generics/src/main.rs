struct PointXY<T> {
    ex: T,
    ye: T,
}

impl<U> PointXY<U> {
    fn ex(&self) -> &U {
        &self.ex
    }
}

impl PointXY<f64> {
    fn ye (&self) -> &f64 {
        &self.ye
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = get_largest(number_list);


    println!("The largest number is {}", largest);

    let char_list = vec!['c', 'f', 't', 'y'];

    let largest = get_largest(char_list);


    println!("The largest number is {}", largest);

    struct Point<T,U> {
        x: T,
        y: U,
    }
    // build impl to train mixup type
    impl <T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V,W>) -> Point<T,W> {
            Point{
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1= Point{ x: 5, y: 10};
    let p2= Point{ x: 5, y: 10.0};
    let p2 = p1.mixup(p2);

    let pxy = PointXY {ex: 5, ye: 10};
    pxy.ex(); // cant access y() due to not f64
    let pxy2 = PointXY {ex: 5.0, ye: 1.0};
    pxy2.ye();

}

// genric function
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T{
    let mut largest = number_list[0];

    for number in number_list{
        if number > largest{
            largest = number;
        }
    }

    largest
}

fn get_largest_char(number_list: Vec<char>) -> char {
    let mut largest = number_list[0];

    for number in number_list{
        if number > largest{
            largest = number;
        }
    }

    largest
}
