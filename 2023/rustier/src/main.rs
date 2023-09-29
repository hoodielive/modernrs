fn paragraph(
    input: &str
) -> IResult<&str, Vec<&str> {
    separated_list0(
        tuple((newline, not(newline))),
        not_line_ending,
    )(input)
}

fn hard_break_paragraphs(
    input: &str,
) -> IResult<&str, Vec<String>> {
    let (input, paragraphs) = separated_list0 (
        tuple(newline, newline),
        paragraph,
    )(input.trim())?;
    let output = paragraphs
        .into_iter()
        .map(|lines| lines.join(""))
        .collect();
    Ok((input, output))
}

fn main() {
    println!("Hello, world!");
}
