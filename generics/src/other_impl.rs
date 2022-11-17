fn overview_02(item1: &impl Overview, item2: &impl Overview) 
{
    
}

fn overview_03<T: Overview>(item1: &T, item2: &T) 
{
    
}

fn overview_04(item1: &impl Overview + AnotherTrait)
{
    
}

fn overview_05<T: Overview + AnotherTrait>(item1: &T, item2: &T) 
{
    
}
