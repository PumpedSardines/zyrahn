pub trait ClLn {
    fn cl_ln(&self) -> (usize, usize, usize, usize) {
        (
            self.ln_start(),
            self.cl_start(),
            self.ln_end(),
            self.cl_end(),
        )
    }
    fn cl_start(&self) -> usize;
    fn cl_end(&self) -> usize;
    fn ln_start(&self) -> usize;
    fn ln_end(&self) -> usize;

    fn combine<T: Sized>(&self, other: T) -> (usize, usize, usize, usize)
    where
        T: ClLn,
    {
        (
            usize::min(other.ln_start(), self.ln_start()),
            usize::min(other.cl_start(), self.cl_start()),
            usize::max(other.ln_end(), self.ln_end()),
            usize::max(other.cl_end(), self.cl_end()),
        )
    }
}

impl ClLn for (usize, usize, usize, usize) {
    fn cl_start(&self) -> usize {
        self.1
    }

    fn cl_end(&self) -> usize {
        self.3
    }

    fn ln_start(&self) -> usize {
        self.0
    }

    fn ln_end(&self) -> usize {
        self.2
    }
}

pub fn combine<T: Sized>(values: &[T]) -> (usize, usize, usize, usize)
where
    T: ClLn,
{
    let min_cl = values.iter().map(|t| t.cl_start()).min().unwrap();
    let max_cl = values.iter().map(|t| t.cl_end()).max().unwrap();
    let min_ln = values.iter().map(|t| t.ln_start()).min().unwrap();
    let max_ln = values.iter().map(|t| t.ln_end()).max().unwrap();

    (min_ln, min_cl, max_ln, max_cl)
}
