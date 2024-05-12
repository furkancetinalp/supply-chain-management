mod planning;
mod manufacturing;
mod idgenerator;
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
