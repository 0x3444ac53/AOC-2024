use itertools::Itertools;

advent_of_code::solution!(3);

#[derive(Debug, Clone)]
struct Operation {
    name: &'static str,
    operands: Vec<i32>,
    result: i32,
}

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
    fn map<F, NewOutput>(self, map_fn: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        BoxedParser::new(map(self, map_fn))
    }
    fn pred<F>(self, pred_fn: F) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        BoxedParser::new(pred(self, pred_fn))
    }
    fn and_then<F, NextParser, NewOutput>(self, f: F) -> BoxedParser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        BoxedParser::new(and_then(self, f))
    }
}

fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }
        Err(input)
    }
}

fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| match parser.parse(input) {
        Ok((next_input, result)) => f(result).parse(next_input),
        Err(err) => Err(err),
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}

fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn one_or_two_or_three<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();
        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }
        if let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }
        if let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }
        Ok((input, result))
    }
}

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        _ => Err(input),
    }
}

fn either<'a, P1, P2, A>(parser1: P1, parser2: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, A>,
{
    move |input| match parser1.parse(input) {
        ok @ Ok(_) => ok,
        Err(_) => parser2.parse(input),
    }
}

fn any_number<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_numeric())
}

fn whitespace<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

fn operands<'a>() -> impl Parser<'a, Vec<i32>> {
    zero_or_more(left(
        one_or_two_or_three(any_number()).map(|w| (w.iter().join("")).parse().unwrap()),
        pair(
            zero_or_more(pair(zero_or_more(whitespace()), match_literal(","))),
            zero_or_more(whitespace()),
        ),
    ))
    // pair(
    //     right(
    //         pair(match_literal("("), zero_or_more(whitespace())),
    //         one_or_two_or_three(any_number()).map(|ns| (ns.iter().join("")).parse().unwrap()),
    //     ),
    //     left(
    //         right(
    //             pair(
    //                 zero_or_more(whitespace()),
    //                 pair(match_literal(","), zero_or_more(whitespace())),
    //             ),
    //             one_or_two_or_three(any_number()).map(|ns| (ns.iter().join("")).parse().unwrap()),
    //         ),
    //         match_literal(")"),
    //     ),
    // )
}

fn function<'a>(expected: &'static str) -> impl Parser<'a, Operation> {
    left(
        right(
            pair(match_literal(expected), match_literal("(")),
            operands().map(move |w| Operation {
                name: expected,
                operands: w.clone(),
                result: match expected {
                    "mul" => w.into_iter().reduce(|acc, x| x * acc).unwrap_or(0),
                    &_ => 0,
                },
            }),
        ),
        match_literal(")"),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cur_input = input;
    let mut ops = vec![];
    loop {
        match function("mul").parse(cur_input) {
            Ok((rest, parsed)) => {
                ops.push(parsed);
                cur_input = rest;
            }
            Err(_) => {
                if cur_input.is_empty() {
                    break;
                }
                cur_input = &cur_input[1..];
            }
        }
    }
    Some(
        ops.into_iter()
            .filter(|o| o.operands.len() == 2)
            .fold(0, |acc, o| acc + o.result as u32),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cur_input = input;
    let mut enabled = true;
    let mut ops = vec![];
    loop {
        match either(function("mul"), either(function("do"), function("don't"))).parse(cur_input) {
            Ok((rest, parsed)) => {
                match parsed.name {
                    "do" => enabled = true,
                    "don't" => enabled = false,
                    "mul" => {
                        if enabled {
                            ops.push(parsed)
                        }
                    }
                    &_ => (),
                }
                cur_input = rest;
            }
            Err(_) => {
                if cur_input.is_empty() {
                    break;
                }
                cur_input = &cur_input[1..];
            }
        }
    }
    Some(
        ops.into_iter()
            .filter(|o| o.operands.len() == 2)
            .fold(0, |acc, o| acc + o.result as u32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
