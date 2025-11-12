fn main() {
    println!("factorial {}", factorial(10));
    println!("fibonnaci {}", fibonnaci(10));
    println!("is_prime {}", is_prime(97));
    println!("gcd {}", gcd(48,18));
    println!("lcm {}", lcm(54, 24));
    println!("average {}", average(&[10.0,12.33]));
    println!("square root {}", square_root(16));
    println!("lcm_gcd {}", lcm_gcd(5,17));
}


fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }   
}


fn fibonnaci(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonnaci(n-1) + fibonnaci(n-2);
    }
}


fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    if n <= 1 || n % 2 == 0 {
        return false;
    } 
    
    let limit = square_root(n as i32) as u32;
    
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}


fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp; 
    }
    a
}


fn lcm( a: u32, b: u32) -> u32 {
    
    let mut greater = if a > b  { a } else { b };
    
    loop {
        if greater % a == 0 && greater % b == 0 {
            return greater;
        }
        greater += 1; 
    }
}

fn lcm_gcd(a: u32, b: u32) -> u32 {
    a * b / gcd(a, b)
}


fn average(n: &[f64]) -> f64 {
    if n.is_empty() {
        return 0.0;
    }
    let sum: f64 = n.iter().sum();
    sum / (n.len() as f64)
}


fn square_root(n: i32) -> f64 {
    if n <= 0 {
        return 0.0;
    }

    let n_f64 = n as f64;
    let mut x = n_f64;

    for _ in 0..=20 {
        x = 0.5 * (x + n_f64 / x);
    };

    x
}


