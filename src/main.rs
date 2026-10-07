use crate::{helper::Helper::CLI, random::Random::Rng};

mod helper;
mod random;

fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();



    let mut fmax = clargs.chr_hist[0].1;
    let tot_elem:usize = clargs.chr_hist.iter().map(|x| {
        if x.1 > fmax{
            fmax = x.1;
        }    
        x.1
    }).sum();

    // Make the space char the most freq repeatign char 
    clargs.chr_hist.insert(0, (b' ', (fmax as f64 * (2.0 + (fmax as f64 / tot_elem as f64))) as usize));
    let tot_elem = tot_elem + (fmax as f64 * (1.0 + (fmax as f64 / tot_elem as f64))) as usize;
    let mut rng = Rng::seed_str(&clargs.hash);

    let mut cdf = clargs.chr_hist.clone();
    for i in 1..cdf.len(){
        cdf[i].1 += cdf[i-1].1;
    } 
    if clargs.dbg{
        println!("{clargs:?}");
    }

    println!("+{}+","-".repeat(clargs.dimx+2));
    for _ in 0..clargs.dimy{
        let mut buff = String::new();
        buff += "| ";
        for _ in 0..clargs.dimx{
            let dens_idx =  ((rng.uniform() * (tot_elem as f64)) as usize).clamp(0, tot_elem-1);
            let idx = cdf.partition_point(|&(_,cum)| cum <= dens_idx); // Prefix sum based indexing
            buff.push(clargs.chr_hist[idx].0 as char);
        }
        buff += " |";
        println!("{buff}");
    }
    println!("+{}+","-".repeat(clargs.dimx+2));

}
