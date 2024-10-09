fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 0;
    }else if guess > secret{
        return 1;
    }else{
        return -1;
    }
}

fn main(){
    let mut guess = 39;
    let mut count = 0;
    let secret: i32 = 13;
    while guess != secret{
        let result = check_guess(guess, secret);
        if result == 0{
            count += 1;
            break;
        }
        if result == 1{
            count += 1;
            guess -= 1;
        }
        if result == -1{
            count += 1;
            guess += 1;
        }
    }
    println!("The secret number was {} and you got it in {} guesses!", secret, count);
}