
pub trait Runner {
    fn parse(&mut self, input: &Vec<String>);
    fn run_p1(&self) -> usize;
    fn run_p2(&self) -> usize;
}