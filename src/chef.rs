use crate::vm;

pub struct ChefProgram {
    file_path: String,
    vm: vm::ChefVM
}

fn build() {
    
}

impl ChefProgram {
    pub fn new(path: &String) -> ChefProgram {
        ChefProgram {
            file_path: String::from(path),
            vm: vm::ChefVM {}
        }
    }

    pub fn run(&mut self) {

    }
}