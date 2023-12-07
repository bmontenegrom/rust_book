impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) ->Option<Self::Item>
}
/*can define with generics 
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }

in this case we need to anotate te type in each implementation of iterator for counter.
When using the next method on Counter, we would have to provide tupe annotations to indicate
which implementarion of Iterator we eant to use
*/