mod driver;
mod eval;

fn main() {
    println!("Hello, world!");
    let mut program = driver::Program::new();

    println!("✨ ¡Bienvenido al evaluador de expresiones! ✨");
    println!("¿Qué puedo hacer por tí?");

    while !program.exit() {
        program.run();
    }
}
