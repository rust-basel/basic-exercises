/*
Here you can try out to implement your own function. It can also be recursive ;).
The fibonacci sequence adds the last two results together like so:
f(n) = f(n-1)+f(n-2)

The first and second result is given with
f(0) = 0 and f(1) = 1

The second result can be computed like so:
f(2) = f(0) + f(1)
*/

fn fibonacci(input: u32) -> u32{
    todo!()
}

#[cfg(test)]
mod tests{
    use crate::functions::fibonacci;

    #[test]
    fn compute_fibonacci_with_a_function(){
        assert_eq!(fibonacci(7), 8);
    }
}