pub struct AveragedCollection {
    list: Vec<i32>, // bot of the field remain private
    average: f64,
}

impl AveragedCollection {

    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove (&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value)=> {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64{
        self.average
    }

    fn update_average(&mut self){
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Hello, world!");

    let mut collection1 = AveragedCollection{
        list: Vec::new(),
        average: 0.0,
    };

    collection1.add(32);
    collection1.add(32);
    collection1.add(3322);
    collection1.add(3542);
    collection1.add(32576);

    println!("average: {}", collection1.average());
}
