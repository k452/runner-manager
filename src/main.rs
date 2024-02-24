pub mod run_container;

fn main() {
    let result = run_container::run();

    let converted: String = String::from_utf8(result.to_vec()).unwrap();
    println!("{}", converted)
}
