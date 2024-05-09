// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.



pub fn foo_if_fizz(fizzish: &str) -> &str 
{
    if fizzish == "fizz" {
        "foo"
    } 
    else 
    {
        if fizzish=="fuzz"
        {
            "bar"
        }
        else
        {
            "baz"
        }
    }
}
// pub fn bar_for_fuzz(temp:&str) -> &str
// {
//     if temp=="fuzz"
//     {
//         "bar"
//     }
//     else
//     {
//         1
//     }
// }
// pub fn default_to_baz(hey:&str) -> &str
// {
//     if hey=="literally anything"
//     {
//         "baz"
//     }
//     else
//     {
//         1
//     }
// }

// No test changes needed!
#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
