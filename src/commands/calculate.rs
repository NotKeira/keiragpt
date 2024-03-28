use eval::eval;

use crate::helpers::strip::Parameters;

pub fn execute(parameters: Parameters) -> String {
    let trigger: &str = "e";
    if parameters.flags.contains(&trigger) {
        let mut express: String = parameters.args.join(" ");
        let mut calculation: f64 = 0.0;
        match eval(&express) {
            Ok(result) => calculation = result.as_f64().unwrap(),
            Err(e) => {
                eprintln!("Error evaluating expression: {}", e);
                return "Errored".to_string();
            }
        };
        express.push_str(" = ");
        express.push_str(&calculation.to_string());
        return express;
    } else {
        return "Error: Invalid flag. Use -e to evaluate an expression.".to_string();
    }
}