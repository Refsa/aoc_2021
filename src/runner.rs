
pub trait Runner {
    fn run_p1(&self, input: &Vec<String>) -> usize;
    fn run_p2(&self, input: &Vec<String>) -> usize;
}