// lib.rs - Canister interface

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}