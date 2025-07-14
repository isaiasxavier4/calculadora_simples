use std::env;
fn main() {
    
    let args: Vec<String> = env::args().collect();

    println!("Argumentos Recebidos: {:?}", args);

    if args.len() != 4 {
        println!("Falta mais expressões para o cáclulo!");
        std::process::exit(1);
    }

    let num1: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Erro: O primeiro argumento '{}' não é um argumento válido!", args[1]);
            std::process::exit(1);
        }
    };

    let num2: f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Erro: O segunto argumento '{}' não é o argumento válido!", args[3]);
            std::process::exit(1);
        }
    };

    let operador = args[2].chars().next().unwrap_or_else(||{
        eprintln!("Error: Operador não encontrado!");
        std::process::exit(1);
    });

    let resultado = match operador {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        'x' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                eprintln!("Erro: Divisão por zero não permitida!");
                std::process::exit(1);
            }
            num1 / num2
        },
        _ => {
            eprintln!("Erro: Operador {} inválido. Use +, -, *, ou /. ", operador);
            std::process::exit(1);
        }
    };
    print!("O resultado de {} {} {} = {}", num1, operador, num2, resultado);

}
