// rustfmt-style_edition: 2027

pub fn parse_conditional<'a, I: 'a>()
-> impl Parser<Input = I, Output = Expr, PartialState = ()> + 'a
where
    I: Stream<Item = char>,
{
}
