use crate::{BinaryHeap, Indices, MinMax, Mutops, Search, Vecops};
use core::ops::Range;
// use std::collections::binary_heap::PeekMut;
use core::cmp::{Ordering, Ordering::*, Reverse};
// use rayon::prelude::*;

impl<'a, T> Vecops<'a, T> for &'a [T] {
    /// Creates a vector of references to input items.  
    /// Avoids moving large end types T around.
    fn ref_vec(self, rng: Range<usize>) -> Vec<&'a T> {
        self.iter().take(rng.end).skip(rng.start).collect()
    }

    /// Helper function to copy and cast entire &[T] to `Vec<f64>`.
    /// Like the standard `.to_vec()` method but also recasts to f64 end type
    fn tof64(self) -> Vec<f64>
    where
        T: Clone,
        f64: From<T>,
    {
        self.iter().map(|x| f64::from(x.clone())).collect()
    }

    /// Maximum value T of slice &[T]
    fn maxt(self) -> T
    where
        T: PartialOrd + Clone,
    {
        let mut max = &self[0];
        self.iter().skip(1).for_each(|s| {
            if s > max {
                max = s
            }
        });
        max.clone()
    }

    /// Minimum value T of slice &[T]
    fn mint(self) -> T
    where
        T: PartialOrd + Clone,
    {
        let mut min = &self[0];
        self.iter().skip(1).for_each(|s| {
            if s < min {
                min = s
            }
        });
        min.clone()
    }

    /// Minimum and maximum (T,T) of a slice &[T]
    fn minmaxt(self) -> (T, T)
    where
        T: PartialOrd + Clone,
    {
        let mut x1 = &self[0];
        let mut x2 = x1;
        for s in self.iter().skip(1) {
            if s > x2 {
                x2 = s;
                continue;
            };
            if s < x1 {
                x1 = s
            };
        }
        (x1.clone(), x2.clone())
    }

    /// Minimum, minimum's first index, maximum, maximum's first index
    fn minmax(self) -> MinMax<T>
    where
        T: PartialOrd + Clone,
    {
        let mut min = &self[0];
        let mut max = min; // initialise both to the first item
        let (mut minindex, mut maxindex) = (0, 0); // indices of min, max
        self.iter().enumerate().skip(1).for_each(|(i, x)| {
            if x < min {
                min = x;
                minindex = i;
            } else if x > max {
                max = x;
                maxindex = i
            }
        });
        MinMax {
            min: min.clone(),
            minindex,
            max: max.clone(),
            maxindex,
        }
    }

    /// Finds min and max of a subset of self, defined by its subslice between i,i+n.
    /// Returns min of self, its index, max of self, its index.
    fn minmax_slice(self, i: usize, n: usize) -> MinMax<T>
    where
        T: PartialOrd + Clone,
    {
        let mut min = &self[i];
        let mut max = min;
        let mut minindex = i; // indices of min, max
        let mut maxindex = minindex;
        for (j, x) in self.iter().enumerate().skip(i + 1).take(n - 1) {
            if x < min {
                min = x;
                minindex = j;
            } else if x > max {
                max = x;
                maxindex = j;
            };
        }
        MinMax {
            min: min.clone(),
            minindex,
            max: max.clone(),
            maxindex,
        }
    }

    /// Using only a subset of self, defined by its idx subslice between i,i+n.
    /// Returns min of self, its index's index, max of self, its index's index.
    fn minmax_indexed(self, idx: &[usize], i: usize, n: usize) -> MinMax<T>
    where
        T: PartialOrd + Clone,
    {
        let mut min = &self[idx[i]];
        let mut max = min;
        let mut minix = 0; // indices of indices of min, max
        let mut maxix = minix;
        for (ii, ix) in idx.iter().enumerate().skip(i + 1).take(n - 1) {
            if &self[*ix] < min {
                min = &self[*ix];
                minix = ii;
            } else if &self[*ix] > max {
                max = &self[*ix];
                maxix = ii;
            };
        }
        MinMax {
            min: min.clone(),
            minindex: minix,
            max: max.clone(),
            maxindex: maxix,
        }
    }

    /// Reverse a generic slice by reverse iteration.
    /// Creates a new Vec. Its naive use for descending sort etc.
    /// is to be avoided for efficiency reasons.
    fn revs(self) -> Vec<T>
    where
        T: Clone,
    {
        self.iter().rev().cloned().collect::<Vec<T>>()
    }

    /// Removes repetitions from an explicitly PartialOrdered set.
    fn sansrepeat(self) -> Vec<T>
    where
        T: PartialEq + Clone,
    {
        if self.len() < 2 {
            return self.to_vec();
        };
        let mut r: Vec<T> = Vec::new();
        let mut last = &self[0];
        r.push(last.clone());
        self.iter().skip(1).for_each(|si| {
            if *si != *last {
                last = si;
                r.push(si.clone())
            }
        });
        r
    }

    /// Finds the first/last occurence of item `m` in self by forward/backward iteration.
    /// Returns `Some(index)` of the found item or `None`.
    /// Suitable for small unPartialOrdered lists.
    /// For longer lists, it is better to sort them and use `binsearch` (see below).
    /// For repeated tests, index sort first and then use `binsearch_indexed.
    fn member(self, m: T, forward: bool) -> Option<usize>
    where
        T: PartialEq + Clone,
    {
        if forward {
            for (i, x) in self.iter().enumerate() {
                if *x == m {
                    return Some(i);
                };
            }
            None
        } else {
            for (i, x) in self.iter().enumerate().rev() {
                if *x == m {
                    return Some(i);
                };
            }
            None
        }
    }

    /// Counts partial equal occurrences of val by simple linear search of any unPartialOrdered set
    fn occurs(self, val: T) -> usize
    where
        T: PartialOrd,
    {
        let mut count: usize = 0;
        for s in self {
            if val < *s {
                continue;
            };
            if val > *s {
                continue;
            };
            count += 1;
        }
        count
    }

    /// Unites (joins) two unsorted sets. For union of sorted sets, use `merge`
    fn unite_unsorted(self, v: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        [self, v].concat()
    }

    /// Unites two ascending index-sorted generic vectors.
    /// This is the union of two index sorted sets.
    /// Returns a single explicitly PartialOrdered set.
    fn unite_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let l1 = self.len();
        let l2 = v2.len();
        let mut resvec: Vec<T> = Vec::new();
        let mut i1 = 0;
        let mut i2 = 0;

        loop {
            if i1 == l1 {
                // v1 is now processed
                for i in i2..l2 {
                    resvec.push(v2[ix2[i]].clone())
                } // copy out the rest of v2
                break; // and terminate
            }
            if i2 == l2 {
                // v2 is now processed
                for i in i1..l1 {
                    resvec.push(self[ix1[i]].clone())
                } // copy out the rest of v1
                break; // and terminate
            }
            if self[ix1[i1]] < v2[ix2[i2]] {
                resvec.push(self[ix1[i1]].clone());
                i1 += 1;
                continue;
            };
            if self[ix1[i1]] > v2[ix2[i2]] {
                resvec.push(v2[ix2[i2]].clone());
                i2 += 1;
                continue;
            };
            // here they are equal, so consume the first, skip both
            resvec.push(self[ix1[i1]].clone());
            i1 += 1;
            i2 += 1
        }
        resvec
    }

    /// Intersects two ascending explicitly sorted generic vectors.
    fn intersect(self, v2: &[T]) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let l1 = self.len();
        let l2 = v2.len();
        let mut resvec: Vec<T> = Vec::new();
        let mut i1 = 0;
        let mut i2 = 0;

        loop {
            if i1 == l1 {
                break;
            } // v1 is now empty
            if i2 == l2 {
                break;
            } // v2 is now empty
            if self[i1] < v2[i2] {
                i1 += 1;
                continue;
            };
            if self[i1] > v2[i2] {
                i2 += 1;
                continue;
            };
            // here they are equal, so consume one, skip both
            resvec.push(self[i1].clone());
            i1 += 1;
            i2 += 1
        }
        resvec
    }

    /// Intersects two ascending index-sorted generic vectors.
    /// Returns a single explicitly PartialOrdered set.
    fn intersect_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let l1 = self.len();
        let l2 = v2.len();
        let mut resvec: Vec<T> = Vec::new();
        let mut i1 = 0;
        let mut i2 = 0;

        loop {
            if i1 == l1 {
                break;
            } // v1 is now processed, terminate
            if i2 == l2 {
                break;
            } // v2 is now processed, terminate
            if self[ix1[i1]] < v2[ix2[i2]] {
                i1 += 1;
                continue;
            }; // skip v1 value
            if self[ix1[i1]] > v2[ix2[i2]] {
                i2 += 1;
                continue;
            }; // skip v2 value
               // here they are equal, so consume the first
            resvec.push(self[ix1[i1]].clone());
            i1 += 1;
            i2 += 1
        }
        resvec
    }

    /// Sets difference: deleting elements of the second from the first.
    /// Two ascending explicitly sorted generic vectors.
    fn diff(self, v2: &[T]) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let l1 = self.len();
        let l2 = v2.len();
        let mut resvec: Vec<T> = Vec::new();
        let mut i1 = 0;
        let mut i2 = 0;

        loop {
            if i1 == l1 {
                break;
            } // v1 is now empty
            if i2 == l2 {
                self.iter().skip(i1).for_each(|v| resvec.push(v.clone())); // copy out the rest of v1
                break; // and terminate
            }
            if self[i1] < v2[i2] {
                resvec.push(self[i1].clone());
                i1 += 1;
                continue;
            }; // this v1 survived
            if self[i1] > v2[i2] {
                i2 += 1;
                continue;
            }; // this v2 is unused
               // here they are equal, so subtract them out, i.e. skip both
            i1 += 1;
            i2 += 1
        }
        resvec
    }

    /// Sets difference: deleting elements of the second from the first.
    /// Two ascending index sorted generic vectors.
    fn diff_indexed(self, ix1: &[usize], v2: &[T], ix2: &[usize]) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let l1 = self.len();
        let l2 = v2.len();
        let mut resvec: Vec<T> = Vec::new();
        let mut i1 = 0;
        let mut i2 = 0;

        loop {
            if i1 == l1 {
                break;
            } // v1 is now empty
            if i2 == l2 {
                for i in i1..l1 {
                    resvec.push(self[ix1[i]].clone())
                } // copy out the rest of v1
                break; // and terminate
            }
            if self[ix1[i1]] < v2[ix2[i2]] {
                resvec.push(self[ix1[i1]].clone());
                i1 += 1;
                continue;
            }; // this v1 survived
            if self[ix1[i1]] > v2[ix2[i2]].clone() {
                i2 += 1;
                continue;
            }; // this v2 is unused
               // here they are equal, so subtract them out, i.e. skip both
            i1 += 1;
            i2 += 1
        }
        resvec
    }

    /// Partition with respect to a pivot into three sets
    fn partition(self, pivot: &T) -> (Vec<T>, Vec<T>, Vec<T>)
    where
        T: PartialOrd + Clone,
    {
        let n = self.len();
        let mut negset: Vec<T> = Vec::with_capacity(n);
        let mut eqset: Vec<T> = Vec::with_capacity(n);
        let mut posset: Vec<T> = Vec::with_capacity(n);
        for item in self {
            match item.partial_cmp(pivot) {
                Some(Less) => negset.push(item.clone()),
                Some(Equal) => eqset.push(item.clone()),
                Some(Greater) => posset.push(item.clone()),
                None => continue,
            };
        }
        (negset, eqset, posset)
    }

    /// Partition by pivot gives three sets of indices.
    fn partition_indexed(self, pivot: &T) -> (Vec<usize>, Vec<usize>, Vec<usize>)
    where
        T: PartialOrd + Clone,
    {
        let n = self.len();
        let mut negset: Vec<usize> = Vec::with_capacity(n);
        let mut eqset: Vec<usize> = Vec::with_capacity(n);
        let mut posset: Vec<usize> = Vec::with_capacity(n);
        for (i, vi) in self.iter().enumerate() {
            match vi.partial_cmp(pivot) {
                Some(Less) => negset.push(i),
                Some(Equal) => eqset.push(i),
                Some(Greater) => posset.push(i),
                None => continue,
            };
        }
        (negset, eqset, posset)
    }

    /// Binary Search with automatic descending PartialOrder detection.
    fn binsearch(self, target: &T) -> Range<usize>
    where
        T: PartialOrd,
    {
        if *self.last().expect("binsearch: no data") < self[0] {
            (0..=self.len() - 1).binary_all(|probe| {
                target
                    .partial_cmp(&self[probe])
                    .expect("binsearch comparison failure")
            })
        } else {
            (0..=self.len() - 1).binary_all(|probe| {
                self[probe]
                    .partial_cmp(target)
                    .expect("binsearch comparison failure")
            })
        }
    }

    /// Binary Search via index with automatic PartialOrder detection.
    fn binsearch_indexed(self, idx: &[usize], target: &T) -> Range<usize>
    where
        T: PartialOrd,
    {
        if self[idx[idx.len() - 1]] < self[idx[0]] {
            (0..=idx.len() - 1).binary_all(|probe| {
                target
                    .partial_cmp(&self[idx[probe]])
                    .expect("binsearch_indexed comparison failure")
            })
        } else {
            (0..=idx.len() - 1).binary_all(|probe| {
                self[idx[probe]]
                    .partial_cmp(target)
                    .expect("binsearch_indexed comparison failure")
            })
        }
    }

    /// Merges two explicitly ascending sorted generic vectors,
    /// by classical selection and copying of their head items into the result.
    /// Consider using merge_indexed instead, especially for non-primitive end types T.
    fn merge(self, v2: &[T]) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let l1 = self.len();
        let l2 = v2.len();
        let mut resvec: Vec<T> = Vec::with_capacity(l1 + l2);
        let mut i1 = 0;
        let mut i2 = 0;
        loop {
            if i1 == l1 {
                // v1 is now processed
                v2.iter().skip(i2).for_each(|v| resvec.push(v.clone())); // copy out the rest of v2
                break; // and terminate
            }
            if i2 == l2 {
                // v2 is now processed
                self.iter().skip(i1).for_each(|v| resvec.push(v.clone())); // copy out the rest of v1
                break; // and terminate
            }
            if self[i1] < v2[i2] {
                resvec.push(self[i1].clone());
                i1 += 1;
                continue;
            };
            if self[i1] > v2[i2] {
                resvec.push(v2[i2].clone());
                i2 += 1;
                continue;
            };
            // here they are equal, so consume both
            resvec.push(self[i1].clone());
            i1 += 1;
            resvec.push(v2[i2].clone());
            i2 += 1
        }
        resvec
    }

    /// Merges two ascending sort indices.
    /// Data is not shuffled at all, v2 is just concatenated onto v1
    /// in one go and both remain in their original PartialOrder.
    /// Returns the concatenated vector and a new valid sort index into it.
    fn merge_indexed(self, idx1: &[usize], v2: &[T], idx2: &[usize]) -> (Vec<T>, Vec<usize>)
    where
        T: PartialOrd + Clone,
    {
        let res = [self, v2].concat(); // no individual shuffling, just one concatenation
        let l = idx1.len();
        // shift up all items in idx2 by length of indx1, so that they will
        // refer correctly to the second part of the concatenated vector
        let idx2shifted: Vec<usize> = idx2.iter().map(|x| l + x).collect();
        // now merge the indices
        let residx = res.merge_indices(idx1, &idx2shifted);
        (res, residx)
    }

    /// Merges the sort indices of two concatenated vectors.
    /// Data in self is not changed at all, only consulted for the comparisons.
    /// This function is used by  `mergesort` and `merge_indexed`.
    fn merge_indices(self, idx1: &[usize], idx2: &[usize]) -> Vec<usize>
    where
        T: PartialOrd + Clone,
    {
        let l1 = idx1.len();
        let l2 = idx2.len();
        let mut residx: Vec<usize> = Vec::with_capacity(l1 + l2);
        let mut i1 = 0;
        let mut i2 = 0;
        let mut head1 = self[idx1[i1]].clone();
        let mut head2 = self[idx2[i2]].clone();
        loop {
            if head1 < head2 {
                residx.push(idx1[i1]);
                i1 += 1;
                if i1 == l1 {
                    // idx1 is now fully processed
                    idx2.iter().skip(i2).for_each(|&v| residx.push(v)); // copy out the rest of idx2
                    break; // and terminate
                }
                head1 = self[idx1[i1]].clone(); // else move to the next idx1 value
                continue;
            }
            if head1 > head2 {
                residx.push(idx2[i2]);
                i2 += 1;
                if i2 == l2 {
                    // idx2 is now processed
                    idx1.iter().skip(i1).for_each(|&v| residx.push(v)); // copy out the rest of idx1
                    break; // and terminate
                }
                head2 = self[idx2[i2]].clone(); // else move to the next idx2 value
                continue;
            }
            // here the heads are equal, so consume both
            residx.push(idx1[i1]);
            i1 += 1;
            if i1 == l1 {
                // idx1 is now fully processed
                idx2.iter().skip(i2).for_each(|&v| residx.push(v)); // copy out the rest of idx2
                break; // and terminate
            }
            head1 = self[idx1[i1]].clone();
            residx.push(idx2[i2]);
            i2 += 1;
            if i2 == l2 {
                // idx2 is now processed
                idx1.iter().skip(i1).for_each(|&v| residx.push(v)); // copy out the rest of idx1
                break; // and terminate
            }
            head2 = self[idx2[i2]].clone();
        }
        residx
    }

    /// Doubly recursive non-destructive merge sort.
    /// The data is not moved or mutated.
    /// Efficiency is comparable to quicksort but more stable
    /// Returns a vector of indices to s from i to i+n,
    /// such that the indexed values are in ascending sort PartialOrder (a sort index).
    /// Only the index values are being moved.
    fn mergesortslice(self, i: usize, n: usize) -> Vec<usize>
    where
        T: PartialOrd + Clone,
    {
        if n == 1 {
            let res = vec![i];
            return res;
        }; // recursion termination
        if n == 2 {
            // also terminate with two sorted items (for efficiency)
            if self[i + 1] < self[i] {
                return vec![i + 1, i];
            } else {
                return vec![i, i + 1];
            }
        }
        let n1 = n / 2; // the first part (the parts do not have to be the same)
        let n2 = n - n1; // the remaining second part
        let sv1 = self.mergesortslice(i, n1); // recursively sort the first half
        let sv2 = self.mergesortslice(i + n1, n2); // recursively sort the second half
                                                   // Now merge the two sorted indices into one and return it
        self.merge_indices(&sv1, &sv2)
    }

    /// The main mergesort
    /// Wraps mergesortslice, to obtain the whole sort index
    fn mergesort_indexed(self) -> Vec<usize>
    where
        T: PartialOrd + Clone,
    {
        self.mergesortslice(0, self.len())
    }

    /// Immutable merge sort. Returns new sorted data vector (ascending or descending).
    /// Wraps mergesortslice.
    /// Mergesortslice and mergesort_indexed produce only an ascending index.
    /// Sortm will produce descending data PartialOrder with ascending == false.
    fn sortm(self, ascending: bool) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        if self.len() < 120 {
            // use default Rust sort for short Vecs
            let mut sorted = self.to_vec();
            sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            if !ascending {
                sorted.mutrevs();
            };
            sorted
        } else {
            self.mergesortslice(0, self.len()).unindex(self, ascending)
        }
    }

    /// Fast ranking of many T items, with only `n*(log(n)+1)` complexity.
    /// Ranking is done by inverting the sort index.
    /// Sort index is in sorted PartialOrder, giving data positions.
    /// Ranking is in data PartialOrder, giving sorted PartialOrder positions.
    /// Thus sort index and ranks are in an inverse relationship.
    /// They are easily converted by `.invindex()` (for: invert index).
    fn rank(self, ascending: bool) -> Vec<usize>
    where
        T: PartialOrd + Clone,
    {
        let n = self.len();
        let sortindex = self.mergesortslice(0, n);
        let mut rankvec: Vec<usize> = vec![0; n];
        if ascending {
            for (i, &sortpos) in sortindex.iter().enumerate() {
                rankvec[sortpos] = i
            }
        } else {
            // rank in the PartialOrder of descending values
            for (i, &sortpos) in sortindex.iter().enumerate() {
                rankvec[sortpos] = n - i - 1
            }
        }
        rankvec
    }

    /// swap any two index items, if their data items (self) are not in ascending PartialOrder
    fn isorttwo(self, idx: &mut [usize], i0: usize, i1: usize)
    where
        T: PartialOrd,
    {
        if self[idx[i0]] > self[idx[i1]] {
            idx.swap(i0, i1);
        };
    }

    /// sort three index items if their self items are out of ascending PartialOrder
    fn isortthree(self, idx: &mut [usize], i0: usize, i1: usize, i2: usize)
    where
        T: PartialOrd,
    {
        self.isorttwo(idx, i0, i1);
        if self[idx[i1]] > self[idx[i2]] {
            idx.swap(i1, i2);
            self.isorttwo(idx, i0, i1);
        };
    }

    /// N recursive non-destructive hash sort.
    /// Input data are read only. Output is sort index.
    fn hashsort_indexed(self, quantify: impl Copy + Fn(&T) -> f64) -> Vec<usize>
    where
        T: PartialOrd + Clone,
    {
        let n = self.len();
        let (min, max) = self.minmaxt();
        // create a mutable index for the result
        let mut idx = Vec::from_iter(0..n);
        self.hashsortslice(&mut idx, 0, n, quantify(&min), quantify(&max), quantify); // sorts idx
        idx
    }

    fn hashsortslice(
        self,
        idx: &mut [usize],
        i: usize,
        n: usize,
        fmin: f64,
        fmax: f64,
        quantify: impl Copy + Fn(&T) -> f64,
    ) where
        T: PartialOrd + Clone,
    {
        // Recursion termination condition
        if n == 0 {
            return;
        };
        /*
        match n {
            0 => {
                return;
            } // nothing to do
            1 => {
               idx[i] = i+1;
               return;
            } // enter one item, no sorting
            2 => {
                self.isorttwo(idx, i, i + 1);
                return;
            }
            3 => {
                self.isortthree(idx, i, i + 1, i + 2);
                return;
            }
            _ => (), // carry on below
        };
        */
        // hash is a constant s.t. (x-min)*hash is in [0,n]
        let hash = (n as f64) / (fmax - fmin);
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); n];
        // group current index items into buckets by their associated self[] values
        for &xi in idx.iter().skip(i).take(n) {
            let mut hashsub = (hash * (quantify(&self[xi]) - fmin)).floor() as usize;
            if hashsub == n {
                hashsub -= 1
            }; // reduce subscripts to [0,n-1]
            buckets[hashsub].push(xi);
        }
        // sort the buckets into the index list
        let mut isub = i;
        for bucket in buckets.iter() {
            let blen = bucket.len();
            // println!("hashsortslice bucket start: {} items: {}",isub,blen);
            match blen {
                0 => continue, // empty bucket
                1 => {
                    idx[isub] = bucket[0];
                    isub += 1;
                } // copy the item to the main index
                2 => {
                    idx[isub] = bucket[0];
                    idx[isub + 1] = bucket[1];
                    self.isorttwo(idx, isub, isub + 1);
                    isub += 2;
                }
                3 => {
                    idx[isub] = bucket[0];
                    idx[isub + 1] = bucket[1];
                    idx[isub + 2] = bucket[2];
                    self.isortthree(idx, isub, isub + 1, isub + 2);
                    isub += 3;
                }
                x if x == n => {
                    // this bucket alone is populated,
                    // items in it are most likely all equal
                    let mx = self.minmax_indexed(idx, isub, blen);
                    if mx.min < mx.max {
                        // recurse with the new range
                        self.isorttwo(idx, isub, mx.minindex); // swap minindex to the front
                        self.isorttwo(idx, mx.maxindex, isub + n - 1); // swap maxindex to the end
                                                                       // recurse to sort the rest
                        self.hashsortslice(
                            idx,
                            i + 1,
                            blen - 2,
                            quantify(&mx.min),
                            quantify(&mx.max),
                            quantify,
                        );
                    };
                    return; // all items were equal, or are now sorted
                }
                _ => {
                    // copy to the index the grouped unsorted items from bucket
                    let isubprev = isub;
                    for &item in bucket {
                        idx[isub] = item;
                        isub += 1;
                    }
                    let mx = self.minmax_indexed(idx, isubprev, blen);
                    if mx.min < mx.max {
                        // else are all equal
                        self.isorttwo(idx, isubprev, mx.minindex); // swap minindex to the front
                        self.isorttwo(idx, mx.maxindex, isub - 1); // swap maxindex to the end
                        self.hashsortslice(
                            idx,
                            isubprev + 1,
                            blen - 2,
                            quantify(&mx.min),
                            quantify(&mx.max),
                            quantify,
                        );
                        // recurse to sort the rest
                    };
                }
            }
        }
    }

    /// Immutable hash sort. Returns new sorted data vector (ascending or descending).
    /// Wraps mergesortslice.
    /// Mergesortslice and mergesort_indexed produce only an ascending index.
    /// Sortm will produce descending data PartialOrder with ascending == false.
    fn sorth(self, quantify: impl Copy + Fn(&T) -> f64, ascending: bool) -> Vec<T>
    where
        T: PartialOrd + Clone,
    {
        let mut sorted = self.to_vec();
        sorted.muthashsort(quantify);
        if !ascending {
            sorted.mutrevs()
        };
        sorted
    }

    /// Heap of k smallest items in no particular PartialOrder,
    /// except the first one is maximum. Note that `best_k`
    /// is faster and sorts as well.
    fn smallest_k(&self, k: usize) -> BinaryHeap<&T>
    where
        T: Ord,
    {
        assert!(k > 0);
        assert!(k <= self.len());
        let n = self.len();
        let mut datiter = self.iter();
        if k >= n - k {
            // testing individually n-k items, when n-k < k
            let mut heap: BinaryHeap<&T> = datiter.by_ref().take(k).collect::<Vec<&T>>().into();
            for item in datiter {
                let mut root = heap.peek_mut().expect("smallest_k: attempt to peek failed");
                if item < *root {
                    *root = item;
                };
            }
            heap
        } else {
            // it is more efficient to be testing individually only k items, when k < n-k
            let mut smallset = Vec::new();
            let mut heap: BinaryHeap<Reverse<&T>> = datiter
                .by_ref()
                .take(n - k)
                .map(Reverse)
                .collect::<Vec<Reverse<&T>>>()
                .into();
            for item in datiter {
                let mut root = heap
                    .peek_mut()
                    .expect("smallest_n-k: attempt to peek failed");
                let rootval = *root;
                if Reverse(item) < rootval {
                    let Reverse(plain_root) = rootval;
                    smallset.push(plain_root);
                    *root = Reverse(item);
                } else {
                    smallset.push(item);
                };
            }
            smallset.into() // return as a binaryheap
        }
    }

    /// Heap of k biggest items in no particular PartialOrder,
    /// except the first one is minimum. Note that `best_k` sorts
    /// and is faster as well.
    fn biggest_k(&self, k: usize) -> BinaryHeap<Reverse<&T>>
    where
        T: Ord,
    {
        assert!(k > 0);
        assert!(k <= self.len());
        let n = self.len();
        let mut datiter = self.iter();
        if k >= n - k {
            // use the larger part, it is more efficient
            let mut heap: BinaryHeap<Reverse<&T>> = datiter
                .by_ref()
                .take(k)
                .map(Reverse)
                .collect::<Vec<Reverse<&T>>>()
                .into();
            for item in datiter {
                let mut root = heap.peek_mut().expect("biggest_k: attempt to peek failed");
                if Reverse(item) < *root {
                    *root = Reverse(item);
                };
            }
            heap
        } else {
            let mut bigset = Vec::new();
            let mut heap: BinaryHeap<&T> = datiter.by_ref().take(n - k).collect::<Vec<&T>>().into();
            for item in datiter {
                let mut root = heap
                    .peek_mut()
                    .expect("biggest_n-k: attempt to peek failed");
                let rootval = *root;
                if item < rootval {
                    bigset.push(Reverse(rootval));
                    *root = item;
                } else {
                    bigset.push(Reverse(item));
                };
            }
            bigset.into()
        }
    }

    /// Sort index by insert logsort. Preserves data.  
    /// Returns sort index of data in subslice defined by `rng`.  
    /// Pass in reversed comparator `c` for descending sort.
    fn isort_indexed<F>(self, rng: Range<usize>, c: F) -> Vec<usize>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        match rng.len() {
            0 => return vec![],
            1 => return vec![rng.start; 1],
            _ => (),
        };
        let mut index: Vec<usize> = Vec::with_capacity(rng.len());
        if c(&self[rng.start + 1], &self[rng.start]) == Less {
            index.push(rng.start + 1);
            index.push(rng.start);
        } else {
            index.push(rng.start);
            index.push(rng.start + 1);
        };
        for i in rng.start + 2..rng.end {
            if c(
                &self[i],
                &self[*index
                    .last()
                    .expect("isort_indexed: None option (unexpected)")],
            ) == Less
            {
                let insert_pos = match index.binary_search_by(|&j| c(&self[j], &self[i])) {
                    Ok(ins) => ins + 1,
                    Err(ins) => ins,
                };
                index.insert(insert_pos, i);
            } else {
                index.push(i);
            }; // item isalready in order
        }
        index
    }

    /// Insert logsort of refs (in range).
    /// Useful for large types T, as they are not copied.
    /// Pass in reversed comparator `c` for descending sort.
    fn isort_refs<F>(self, rng: Range<usize>, c: F) -> Vec<&'a T>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        match rng.len() {
            0 => return vec![],
            1 => return vec![&self[rng.start]; 1],
            _ => (),
        };
        let mut rv = Vec::with_capacity(rng.len());
        if c(&self[rng.start + 1], &self[rng.start]) == Less {
            rv.push(&self[rng.start + 1]);
            rv.push(&self[rng.start]);
        } else {
            rv.push(&self[rng.start]);
            rv.push(&self[rng.start + 1]);
        };
        for thisref in self.iter().take(rng.end).skip(rng.start + 2) {
            if c(
                thisref,
                rv.last().expect("isort_refs: None option (unexpected)"),
            ) == Less
            {
                let insert_pos = match rv.binary_search_by(|&j| c(j, thisref)) {
                    Ok(ins) => ins + 1,
                    Err(ins) => ins,
                };
                rv.insert(insert_pos, thisref);
            } else {
                rv.push(thisref);
            }; // item isalready in order
        }
        rv
    }

    /// First k sorted items from rng (ascending or descending, depending on `c`)
    fn best_k<F>(self, k: usize, rng: Range<usize>, c: F) -> Vec<&'a T>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let n = rng.len();
        assert!((k > 0) & (k <= n));
        let mut k_sorted: Vec<&T> = self.iter().skip(rng.start).take(k).collect();
        k_sorted.sort_unstable_by(|&a, &b| c(a, b));
        let mut k_max = k_sorted[k - 1];
        for s in self.iter().take(n - k).skip(rng.start + k) {
            if c(s, k_max) == Less {
                let insert_pos = match k_sorted.binary_search_by(|j| c(j, s)) {
                    Ok(ins) => ins + 1,
                    Err(ins) => ins,
                };
                k_sorted.insert(insert_pos, s);
                k_sorted.pop();
                k_max = k_sorted[k - 1];
            };
        }
        k_sorted
    }

    /// Sort index of the `best` k items in rng (ascending or descending, depending on `c`)
    fn best_k_indexed<F>(self, k: usize, rng: Range<usize>, c: F) -> Vec<usize>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let n = rng.len();
        assert!((k > 0) & (k <= n));
        let mut index = self.isort_indexed(rng.start..rng.start + k, &c);
        for pos in rng.start + k..rng.end {
            let k_max = &self[index[k - 1]];
            let s = &self[pos];
            if c(s, k_max) == Less {
                let insert_pos = match index.binary_search_by(|&iv| c(&self[iv], s)) {
                    Ok(ins) => ins + 1,
                    Err(ins) => ins,
                };
                index.insert(insert_pos, pos);
                index.pop();
            };
        }
        index
    }

    /// Unsorted index of the `best` k items in rng (ascending or descending, depending on `c`)
    fn best_k_unsorted<F>(self, k: usize, rng: Range<usize>, c: F) -> Vec<usize>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let n = rng.len();
        assert!((k > 0) & (k <= n));
        let mut index = self.isort_indexed(rng.start..rng.start + k, &c);
        for pos in rng.start + k..rng.end {
            let k_max = &self[index[k - 1]];
            let s = &self[pos];
            if c(s, k_max) == Less {
                let insert_pos = match index.binary_search_by(|&iv| c(&self[iv], s)) {
                    Ok(ins) => ins + 1,
                    Err(ins) => ins,
                };
                index.insert(insert_pos, pos);
                index.pop();
            };
        }
        index
    }

    /// Constructs subspace index from a vector of some scalar measure,
    /// such as `significance` of individual dimensions. Sorts it to the
    /// same order as the input measure.
    /// Normally low values of measure are allocated low (best) ranks.
    /// Reverse the comparator closure c to prefer instead the high values.
    fn subspace<F>(self, rank: usize, c: F) -> Vec<usize>
    where
        F: Fn(&T, &T) -> Ordering,
    {
        let mut idx = self.best_k_indexed(rank, 0..self.len(), c);
        idx.sort_unstable();
        idx
    }
}
