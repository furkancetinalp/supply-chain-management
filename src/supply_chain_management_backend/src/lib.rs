mod planning;
mod manufacturing;
mod idgenerator;
mod entities;
#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
