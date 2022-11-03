use std::error::Error;

fn is_num_args_valid(argc: usize) -> bool {
    match argc {
        2 | 3 | 4 => true,
        1 | _ => false,
    }
}
pub fn are_cargo_args_valid(argc: usize, argv: Vec<String>) -> bool {
    if !is_num_args_valid(argc) {
        return false;
    } else {
        true
    }
}

pub fn cargo_init(argv: Vec<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
