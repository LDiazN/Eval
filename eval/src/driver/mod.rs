// Driver code for our interpreter
use crate::eval;
use std::io;
use std::io::Write;

enum ProgramAction {
    Exit,
    Eval(eval::Order, Vec<String>),
    Show(eval::Order, Vec<String>)
}

pub enum ProgramError {
    NotEnoughActionArgs,
    InvalidActionArgument,
    InvalidAction,
}

pub struct Program {
    exit : bool
}

impl Program {

    /// Create a new program instance ready to run
    pub fn new() -> Program {
        Program{
            exit : false
        }
    }

    /// Run one iteration of the program
    pub fn run(&mut self) {
        let mut line = String::new();

        print!(">> "); // print prompt
        // so the print! doesn't mess up the execution order with read_line
        io::stdout().flush().expect("Couldn't flush stdout"); 

        // Read a single line
        match io::stdin().read_line(&mut line) {
            Err(_) => panic!("Error leyendo input D:"),
            Ok(_)  => {}
        }

        // Do something depending on the action type
        match Program::parse(line) {
            Err(e) => e.print(),
            Ok(ProgramAction::Exit) => self.exit = true,
            Ok(ProgramAction::Eval(order,exp)) => {
                match eval::eval(order, exp) {
                    Err(e) => e.print(),
                    Ok(i)  => println!("{}",i)
                };

            },
            Ok(ProgramAction::Show(order,expr)) => {
                let expr = match eval::ExprTree::parse(order,expr) {
                    Err(e)   => { e.print(); return; },
                    Ok(expr) => expr
                };

                expr.print();
            }
        }
    }

    /// Checks if the program should close
    /// ## Return
    /// if this program should not be running
    pub fn exit(&self) -> bool {
        self.exit
    }

    /// parse a line into an action, or an error if it was not possible
    fn parse(line : String) -> Result<ProgramAction, ProgramError> {

        let line = line.to_lowercase();

        let mut input = line.split_whitespace();
        let action  =  match input.next() {
                            None    => return Err(ProgramError::NotEnoughActionArgs),
                            Some(s) => s
                        };

        if action == "salir" {
            Ok(ProgramAction::Exit)
        }
        else if action == "mostrar" || action == "eval" {

            let act = if action == "mostrar" { ProgramAction::Show } else { ProgramAction::Eval };

            if let Some(order) = Program::parse_order(input.next()) {
                Ok(
                    act(    order, 
                            input.map(
                                |s| s.to_string()
                            ).collect()
                    )
                )
            }
            else {
                return Err(ProgramError::InvalidActionArgument);
            }
        }
        else {
            Err(ProgramError::InvalidAction)
        }
    }

    /// Print usage
    pub fn usage() {
        println!("📖 Uso: <accion> [argumentos]");
        println!("Posibles acciones: ");
        println!("  * mostrar <orden> <expresion>");
        println!("  * eval <orden> <expresion>");
        println!("  * salir");
    }

    // parse a possible option into an order type
    fn parse_order(arg : Option<&str>) -> Option<eval::Order>{
        let order = match arg {
            None => return None,
            Some(s) => s
        };

        if order == "pre" {
            Some(eval::Order::Pre)
        }
        else if order == "post" {
            Some(eval::Order::Post)
        }
        else { // scream
            None
        }
    }  
}

impl ProgramError {

    /// Print errors
    fn print(&self) {
        match self {
            ProgramError::InvalidActionArgument => {
                eprintln!("🚨 Esos argumentos no son validos para esta accion");
            },
            ProgramError::NotEnoughActionArgs => {
                eprintln!("🚨 Faltan argumentos para esta accion")
            },
            ProgramError::InvalidAction => {
                eprintln!("🚨 Esa no es una acción válida.");
                Program::usage();
            }
        }
    }
}