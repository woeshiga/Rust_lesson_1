use std::io;
fn main() {
    // Глава 3
    loop {
        println!("Введите номер задачи, которую, хотите проверить:\n
        1) Конвертация температур между значениями по Фаренгейту к Цельсию.\n
        2) - Генерирование n-го числа Фиббоначи\n
        0) - Выход");
        let n: u8 = loop {
            let mut n = String::new();
    
            io::stdin()
            .read_line(&mut n)
            .expect("Input error!");
    
            match n.trim().parse() {
                Ok(num) => {
                    break num;
                },
                Err(msg) => {
                    println!("Ошибка: {msg}");
                }
            }
        };
        if n == 1 {
            far_cel();
        } else if n == 2 {
            fibbonachi();
        }
        else if n == 0 {
            break;
        } else {
            println!("Нет такой задачи!");
        }
    }

    
}

fn fibbonachi() {
    println!("Генерирование n-го числа Фиббоначи");
    println!("Введите n.");
    let n: i128 = loop {
        let mut n = String::new();
        io::stdin()
        .read_line(&mut n)
        .expect("Input error!");

        match n.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(msg) => {
                println!("Введите число {n}! {msg}");
            }
        }
    };
    let mut _num_1 = 0;
    let mut _num_2 = 1;  
    let mut num_3 = 1;
    for _ in 0..n - 2 {
       (_num_1, _num_2) = (_num_2, num_3);
       num_3 = _num_1 + _num_2;

    }
    println!("{num_3}");

}

fn far_cel() {
    println!("Конвертация температур между значениями по Фаренгейту к Цельсию.");
    let mut far = String::new();
    println!("Введите температуру по Фарренгейту.");
    let far: f32 = loop {
        io::stdin()
        .read_line(&mut far)
        .expect("Input error!");

        match far.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                continue;
            }
        }
    };

    let res = (far - 32.0) * 5.0 / 9.0;

    println!("Температура {far} градусов по Фаренгейту равна {res} градусов по Цельсию.");
}
