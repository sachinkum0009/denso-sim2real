use denso_sim2real::Sim2Real;

#[tokio::main]
async fn main() {
    let sim2real = Sim2Real::new();
    sim2real.do_something();
    println!("Hello, world!");
}
