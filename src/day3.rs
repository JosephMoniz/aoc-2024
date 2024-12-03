struct Literal(&'static [u8]);

struct Number();

struct AndThen<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    first: P,
    second: Q,
    _phantom: std::marker::PhantomData<(R, S)>,
}

struct SkipUntil<P, R>
where
    P: Parser<R>,
{
    parser: P,
    _phantom: std::marker::PhantomData<R>,
}

struct DiscardLeft<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    first: P,
    second: Q,
    _phantom: std::marker::PhantomData<(R, S)>,
}

struct DiscardRight<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    first: P,
    second: Q,
    _phantom: std::marker::PhantomData<(R, S)>,
}

struct Or<P, Q, R>
where
    P: Parser<R>,
    Q: Parser<R>,
{
    first: P,
    second: Q,
    _phantom: std::marker::PhantomData<R>,
}

struct Repeat<P, R>
where
    P: Parser<R>,
{
    parser: P,
    _phantom: std::marker::PhantomData<R>,
}

struct Map<P, F, R, S>
where
    P: Parser<R>,
    F: Fn(R) -> S,
{
    parser: P,
    f: F,
    _phantom: std::marker::PhantomData<(R, S)>,
}

fn literal(value: &'static [u8]) -> Literal {
    Literal(value)
}

fn number() -> Number {
    Number()
}

fn and_then<P, Q, R, S>(first: P, second: Q) -> AndThen<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    AndThen {
        first,
        second,
        _phantom: std::marker::PhantomData,
    }
}

fn skip_until<P, R>(parser: P) -> SkipUntil<P, R>
where
    P: Parser<R>,
{
    SkipUntil {
        parser,
        _phantom: std::marker::PhantomData,
    }
}

fn discard_left<P, Q, R, S>(first: P, second: Q) -> DiscardLeft<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    DiscardLeft {
        first,
        second,
        _phantom: std::marker::PhantomData,
    }
}

fn discard_right<P, Q, R, S>(first: P, second: Q) -> DiscardRight<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    DiscardRight {
        first,
        second,
        _phantom: std::marker::PhantomData,
    }
}

fn or<P, Q, R>(first: P, second: Q) -> Or<P, Q, R>
where
    P: Parser<R>,
    Q: Parser<R>,
{
    Or {
        first,
        second,
        _phantom: std::marker::PhantomData,
    }
}

fn repeat<P, R>(parser: P) -> Repeat<P, R>
where
    P: Parser<R>,
{
    Repeat {
        parser,
        _phantom: std::marker::PhantomData,
    }
}

fn map<P, F, R, S>(parser: P, f: F) -> Map<P, F, R, S>
where
    P: Parser<R>,
    F: Fn(R) -> S,
{
    Map {
        parser,
        f,
        _phantom: std::marker::PhantomData,
    }
}

trait Parser<R>: Sized {
    fn parse<'a>(&self, input: &'a [u8]) -> Option<(R, &'a [u8])>;

    fn and_then<P, S>(self, other: P) -> AndThen<Self, P, R, S>
    where
        P: Parser<S>,
    {
        and_then(self, other)
    }

    fn discard_left<P, S>(self, other: P) -> DiscardLeft<Self, P, R, S>
    where
        P: Parser<S>,
    {
        discard_left(self, other)
    }

    fn discard_right<S, T>(self, other: S) -> DiscardRight<Self, S, R, T>
    where
        S: Parser<T>,
    {
        discard_right(self, other)
    }

    fn or<P>(self, other: P) -> Or<Self, P, R>
    where
        P: Parser<R>,
    {
        or(self, other)
    }

    fn map<S, F>(self, f: F) -> Map<Self, F, R, S>
    where
        F: Fn(R) -> S,
    {
        map(self, f)
    }
}

impl Parser<()> for Literal {
    fn parse<'a>(&self, input: &'a [u8]) -> Option<((), &'a [u8])> {
        if input.starts_with(self.0) {
            return Some(((), &input[self.0.len()..]));
        }
        None
    }
}

impl Parser<usize> for Number {
    fn parse<'a>(&self, input: &'a [u8]) -> Option<(usize, &'a [u8])> {
        let mut cursor = input;
        let mut number = 0;
        while cursor.len() > 0 {
            let byte = cursor[0];
            if byte < b'0' || byte > b'9' {
                break;
            }
            number = number * 10 + (byte - b'0') as usize;
            cursor = &cursor[1..];
        }
        if cursor.len() == input.len() {
            None
        } else {
            Some((number, cursor))
        }
    }
}

impl<P, Q, R, S> Parser<(R, S)> for AndThen<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    fn parse<'a>(&self, input: &'a [u8]) -> Option<((R, S), &'a [u8])> {
        let (first, post_first) = self.first.parse(input)?;
        let (second, post_second) = self.second.parse(post_first)?;
        Some(((first, second), post_second))
    }
}

impl<P, R> Parser<R> for SkipUntil<P, R>
where
    P: Parser<R>,
{
    fn parse<'a>(&self, input: &'a [u8]) -> Option<(R, &'a [u8])> {
        let mut cursor = input;
        loop {
            if cursor.len() == 0 {
                return None;
            }
            let maybe_matched = self.parser.parse(cursor);
            if maybe_matched.is_some() {
                return maybe_matched;
            }
            cursor = &cursor[1..];
        }
    }
}

impl<P, Q, R, S> Parser<S> for DiscardLeft<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    fn parse<'a>(&self, input: &'a [u8]) -> Option<(S, &'a [u8])> {
        let (_, post_first) = self.first.parse(input)?;
        self.second.parse(post_first)
    }
}

impl<P, Q, R, S> Parser<R> for DiscardRight<P, Q, R, S>
where
    P: Parser<R>,
    Q: Parser<S>,
{
    fn parse<'a>(&self, input: &'a [u8]) -> Option<(R, &'a [u8])> {
        let (first, post_first) = self.first.parse(input)?;
        let (_, post_second) = self.second.parse(post_first)?;
        Some((first, post_second))
    }
}

impl<P, Q, R> Parser<R> for Or<P, Q, R>
where
    P: Parser<R>,
    Q: Parser<R>,
{
    fn parse<'a>(&self, input: &'a [u8]) -> Option<(R, &'a [u8])> {
        self.first.parse(input).or_else(|| self.second.parse(input))
    }
}

impl<P, F, R, S> Parser<S> for Map<P, F, R, S>
where
    P: Parser<R>,
    F: Fn(R) -> S,
{
    fn parse<'a>(&self, input: &'a [u8]) -> Option<(S, &'a [u8])> {
        let (result, post_result) = self.parser.parse(input)?;
        Some(((self.f)(result), post_result))
    }
}

impl<P, R> Parser<Vec<R>> for Repeat<P, R>
where
    P: Parser<R>,
{
    fn parse<'a>(&self, input: &'a [u8]) -> Option<(Vec<R>, &'a [u8])> {
        let mut cursor = input;
        let mut results = Vec::new();
        while let Some((result, post_result)) = self.parser.parse(cursor) {
            results.push(result);
            cursor = post_result;
        }
        Some((results, cursor))
    }
}

enum Operation {
    Mul(usize, usize),
    Do(),
    Dont(),
}

pub fn a(input: &[u8]) -> usize {
    let mul_ops_parser = repeat(skip_until(
        literal(b"mul")
            .and_then(literal(b"("))
            .discard_left(number())
            .and_then(
                literal(b",")
                    .discard_left(number())
                    .discard_right(literal(b")")),
            ),
    ));
    let (mul_ops, _) = mul_ops_parser.parse(input).unwrap();
    mul_ops.iter().map(|(a, b)| a * b).sum()
}

pub fn b(input: &[u8]) -> usize {
    let mul_op_parser = literal(b"mul")
        .and_then(literal(b"("))
        .discard_left(number())
        .and_then(
            literal(b",")
                .discard_left(number())
                .discard_right(literal(b")")),
        )
        .map(|(a, b)| Operation::Mul(a, b));
    let do_op_parser = literal(b"do()").map(|_| Operation::Do());
    let dont_op_parser = literal(b"don't()").map(|_| Operation::Dont());
    let parser = repeat(skip_until(
        mul_op_parser.or(do_op_parser).or(dont_op_parser),
    ));

    let (ops, _) = parser.parse(input).unwrap();

    let mut result = 0;
    let mut enabled = true;
    for op in ops {
        match op {
            Operation::Mul(a, b) => {
                if enabled {
                    result += a * b;
                }
            }
            Operation::Do() => {
                enabled = true;
            }
            Operation::Dont() => {
                enabled = false;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3a() {
        // given ...
        let input_str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 161);
    }

    #[test]
    fn test_day3b() {
        // given ...
        let input_str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 48);
    }
}
