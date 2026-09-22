/// Calculate a factorial recursively
fn factorial(x) {
    if x <= 1 {
        return 1;
    }
    return x * factorial(x - 1);
}

// Call the function and print the result
let number = 5;
print(`The factorial of ${number} is ${factorial(number)}`);

factorial(number)
// ```

// ### Loading and Running the Rhai File in Rust
// You can execute the file above from a host Rust application using the [Rhai Engine](https://rhai.rs/book/engine/hello-world.html):

// ```rust
// use rhai::{Engine, EvalAltResult};

// fn main() -> Result<(), Box<EvalAltResult>> {
//     // Create the Rhai scripting engine
//     let engine = Engine::new();

//     // Evaluate the .rhai file directly
//     let result: i64 = engine.eval_file("factorial.rhai".into())?;
    
//     println!("Result returned from script: {result}");
//     Ok(())
// }