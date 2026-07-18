use ort::device::Device;
use ort::session::Session;

pub struct Sim2Real {}

impl Sim2Real {
    pub fn new() -> Self {
        Self {}
    }

    pub fn do_something(&self) {
        println!("hello");
    }

    pub fn my_code(&self) {
        let session = Session::builder().unwrap();
    }
}
