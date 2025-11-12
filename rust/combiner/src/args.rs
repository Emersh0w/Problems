fn get_nth_arg(n: usize) -> String {
    std::env::args().nth(n).unwrap()
}

#[derive(Debug)]
pub struct Args {
    pub image_one: String,
    pub image_two: String,
    pub image_out: String,
}

impl Args {
    pub fn new() -> Self { 
        Args {
            image_one : get_nth_arg(1),
            image_two : get_nth_arg(2),
            image_out : get_nth_arg(3),
        }
    }
}