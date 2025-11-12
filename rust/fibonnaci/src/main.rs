fn main() {
    println!("\nFIBONNACI \n");

    println!("Fibonnaci N: {}\n",fibonnaci(10));
    fibonnaci_sequence(10);

}

fn fibonnaci(n: i32) -> i32 {
    if n <= 0 {
        println!("N can not be 0");
        return 0;
    }
    else if n == 1 {
        return 0;
    }
    else if n == 2 {
        return 1;
    }
    else {
        return fibonnaci(n-1) + fibonnaci(n-2);
    }
}


fn fibonnaci_sequence(top: u32) {
    let mut a = 1;
    let mut b = 1;

    for _ in 1..top {
        println!("{a}");

        let c = a + b;
        a = b;
        b = c;
    }
}