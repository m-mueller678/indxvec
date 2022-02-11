#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{merge::*, Indices, Printing, GR, UN};
use ran::*;

#[test]
fn indxvec() {
    let mut seeds = set_xoshiro(123456789);
    let v1 = ranvf64_xoshiro(&mut seeds,19);
    println!("\nv1: {}", v1.gr());  
    let v2 = ranvf64_xoshiro(&mut seeds,19);
    println!("v2: {}", v2.gr());
    println!("Minmax:       {}", minmax(&v1));
    println!("minmaxt:      {GR}{:?}{UN}", minmaxt(&v1));
    let (lset,gset) = partition_indexed(&v1, 0.5);
    println!( "Partition indices around 0.5:\n{}\n{}", lset.gr(),gset.gr() );
    println!("Ranks to f64: {}", rank(&v1, true).gr());
    println!("Sorted:       {}", sortm(&v1, true).gr()); // sorted data but index lost
    println!("Sorted:       {}", hashsort(&v1,0.0,1.0).unindex(&v1, true).gr()); // new hashsort
    println!("Sorted:       {}", rank(&v1, false).invindex().unindex(&v1, false).gr() );
    println!("Ranks:        {}", rank(&v1, true).gr()); // how to get ranks
    println!("Ranks:        {}", rank(&v1, true).complindex().complindex().gr() ); // symmetry
    println!("Ranks:        {}", sortidx(&v1).invindex().gr()); // simplest ranks from sortindex
    println!("Ranks rev:    {}", rank(&v1, true).revindex().gr()); // revindex() reverses any index
    println!("Ranks rev:    {}", sortidx(&v1).complindex().invindex().gr()); // via sortidx()  and complindex()
    println!("Ranks rev:    {}", sortidx(&v1).invindex().revindex().gr()); // via revindex()
    println!("Ranks desc:   {}", rank(&v1, false).gr()); // descending ranks, not the same as ranks reversed!!
    println!("Ranks desc:   {}", rank(&v1, true).complindex().gr()); // descending ranks, not the same as ranks reversed!!
    println!("Ranks desc:   {}", sortidx(&v1).invindex().complindex().gr()); // descending ranks, not the same as ranks reversed!!
    println!("Sort index:   {}", sortidx(&v1).gr()); // sortindex, can be unindexed at anytime
    println!("Sort index:   {}", hashsort(&v1,0.0,1.0).gr()); 
    println!("Sortix rev:   {}", sortidx(&v1).revindex().gr());
    println!("Sortix rev:   {}", rank(&v1, false).invindex().gr()); // descending sort index from desc ranks
    println!("Ranks to idx: {}", rank(&v1, true).invindex().gr()); // ascending sort index from ascending ranks
    println!("Ranks to idx: {}", rank(&v1, false).complindex().invindex().gr()); // from ascending ranks
    println!("Idx to ranks: {}", sortidx(&v1).invindex().gr());
    println!("Sorted rev:   {}", sortm(&v1, false).gr()); // descending sort, index lost
    println!("Sorted rev:   {}", revs(&sortm(&v1, true)).gr()); // the above simply reversed
    println!("Sorted rev:   {}", sortidx(&v1).unindex(&v1, false).gr()); // more efficient reversal
    println!("Sorted rev:   {}", sortidx(&v1).revindex().unindex(&v1, true).gr()); // by reversing the sort index
    println!("Sorted rev:   {}", sortidx(&v1).invindex().complindex().invindex().unindex(&v1, true).gr());
    println!("Sorted rev:   {}", rank(&v1, true).complindex().invindex().unindex(&v1, true).gr()); // complindex reverses ranks
    println!("Spearman corr v1,v2: {}",rank(&v1, true).ucorrelation(&rank(&v2, true)).gr()); //  1 for any Vec
    //println!("Spearman corr against reversed: {}",
    //    rank(&v1, true).ucorrelation(&rank(&v1, false)).gr()); // -1 for any Vec
    let (vm, vi) = merge_indexed(&v1, &hashsort(&v1,0.0,1.0),
        &v2, &hashsort(&v2,0.0,1.0)); // merge two vecs using their sort indices
    let sorted = vi.unindex(&vm, true);
    println!("v1 and v2 sorted, merged and unindexed:\n{}", sorted.gr());
    let sorteddesc = vi.unindex(&vm, false);
    println!("The above reversed:\n{}", sorteddesc.gr());
    println!("Binsearch for 0.4, found before: {}",binsearch(&sorted,0.4).gr()); // binsearch
    println!("Binsearchdesc for 0.4, found before: {}",binsearchdesc(&sorteddesc,0.4).gr()); // binsearch
    println!("Memsearchdesc for 0.4, found at: {}",
        memsearchdesc(&revs(&sorted),0.4).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Memsearch_indexed for 0.4, found at: {}",
        memsearch_indexed(&vm, &vi,0.4).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Memsearch_indexed for 0.4, found at: {}",
        memsearchdesc_indexed(&vm, &vi.revindex(),0.4).map_or_else(|| "None".gr(), |x| x.gr()));
    println!("Occurrences count of 0.75: {}",occurs(&sorted, 0.75).gr());
    println!("Occurrences count of 0.75: {}",occurs_multiple(&sorted,&sorteddesc,0.75).gr());
    println!("Intersect_indexed: {}",intersect_indexed(&vm, &vi, &v1, &sortidx(&v1)).gr());
    println!("Diff_indexed: {}",diff_indexed(&vm, &vi, &v1, &sortidx(&v1)).gr());
    println!("Sansrepeat:   {}\n", sansrepeat(&sorted).gr());
}
