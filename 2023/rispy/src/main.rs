#[derive(Clone)]
enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
}

#[derive(Debug)]
enum RispErr {
    Reason(String),
}

#[derive(Clone)]
struct RispEnv {
    data: HashMap<String, RispExp>,
}

fn tokenize(expr: String) -> Vec<String> {
    expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(tokens: &'a [String]) -> Result<(RispExp, &'a [String]), RispErr> {
    let (token, rest) = tokes.split_first()
        .ok_or(
            RispErr::Reason("Could not get token.".to_string())
        )?;
    match &token[...] {
        "(" => read_seq(rest),
        ")" => Err(RispErr::Reason("Unexpected `)`".to_string())),
        _ =>  Ok((parse_atom(token), rest)),
    }
} 

fn main() {

}
