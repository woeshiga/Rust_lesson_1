trait Sum {
    fn summa(&self) -> String;
}

struct Vector {
    numbers: Vec<i32>
}

impl Sum for Vector {
    fn summa(&self) -> String {
        format!("Сумма: {}", self.numbers.iter().sum::<i32>())
    }
}

impl Vector {
    fn new(numbers: Vec<i32>) -> Self {
        Self { numbers }
    }
}

fn main() {
    let vector = Vector::new(vec![1, 2, 3, 4, 5]);
    println!("Сумма: {}", vector.summa());
}
