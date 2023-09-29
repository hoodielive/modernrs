


fn hard_break_paragraphs(input: &str) -> Vec<String>
{
    let mut paragraphs: Vec<String> = vec![];
    let mut index = 0;
    for line in input.trim().lines() 
    {
        if line.is_empty()
        {
            index += 1;
        }
        else if let Some(s) = paragraphs.get_mut(index)
        {
           s.push_str(" ") ;
           s.push_str(line);
        }
        else
        {
            paragraphs.push(line.to_string());
        }
    }

    paragraphs
    
}

fn main()
{
    
}
