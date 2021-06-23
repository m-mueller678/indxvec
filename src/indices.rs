use crate::Indices;

impl Indices for &[usize] {

    /// Constructs inverted index, eg. from sort index to data ranks
    /// This is a symmetric operation, i.e. any even number of applications 
    /// leads back to the original index form.
    fn invindex(self) -> Vec<usize> {
        let n = self.len();
        let mut index:Vec<usize> = vec![0;n];
        for i in 0..n { index[self[i]] = i };    
        index
    }

    /// Collects values from v in the order given by self index. 
    /// When ascending is false, creates descending order.
    /// Used by msort for ascending or descending sort.
    /// Good for efficient sorting of any vectors.    
    fn unindex<T: Copy>(self, v:&[T], ascending: bool) -> Vec<T> {
        if ascending { self.iter().map(|&i| v[i]).collect() }
        else { self.iter().rev().map(|&i| v[i]).collect()   } 
    }
    
    /// Pearson's correlation coefficient of two $[usize] slices.
    /// When the inputs are ranks, then this gives Spearman's correlation 
    /// of the original data. However, in general, any other ordinal measures
    /// can be deployed (not just the ranks). 
    fn ucorrelation(self, v: &[usize]) -> f64 {
        let (mut sy, mut sxy, mut sx2, mut sy2) = (0_f64, 0_f64, 0_f64, 0_f64);
        let sx: f64 = self
            .iter()
            .zip(v)
            .map(|(&ux, &uy)| {
                let x = ux as f64;
                let y = uy as f64;
                sy += y;
                sxy += x * y;
                sx2 += x * x;
                sy2 += y * y;
                x
            })
            .sum();
        let nf = self.len() as f64;
        (sxy - sx / nf * sy) / ((sx2 - sx / nf * sx) * (sy2 - sy / nf * sy)).sqrt()
    }
}
