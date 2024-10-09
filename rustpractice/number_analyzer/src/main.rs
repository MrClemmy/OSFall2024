const NUM_LIST: [i32; 10] = [23, 92, 34, 11, 65, 83, 32, 47, 30, 44];

fn is_even(n: i32) -> bool{
    if n % 2 == 0{
        return true;
    }else{
        return false;
    }
}

fn main() {
    for &num in NUM_LIST.iter(){
        println!("{}", is_even(num));
        
        if num % 15 == 0{
            println!("FizzBuzz");
        }else if num % 3 == 0{
            println!("Fizz");
        }else if num % 5 == 0{
            println!("Buzz");
        }else{
            println!("NO FB");
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < 10{
        sum += NUM_LIST[i];
        i += 1;
    }
    println!("The sum of the array is: {}", sum);

    let mut maxnum = NUM_LIST[0];
    for &num in NUM_LIST.iter() {
        if num > maxnum {
            maxnum = num;
        }
    }
    println!("The largest number is: {}", maxnum);
}
