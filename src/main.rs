use std::{sync::RwLock, time::Instant};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::{FxHashMap, FxHashSet};

const LENGTH: usize = 4096;
const HEIGHT: usize = 4096;

fn main() {
    let mut map: Vec<Vec<RwLock<FxHashSet<usize>>>> = Vec::new();
    let mut commands: FxHashMap<(usize, usize), ((usize, usize), usize)> = FxHashMap::default();
    // init map
    for x in 0..LENGTH {
        let mut row = Vec::new();
        for y in 0..HEIGHT {
            row.push(RwLock::new(FxHashSet::default()));
            // make random 1 tile perturbations to x and y without overflowing the vec boundaries
            let x = x.wrapping_add(rand::random::<usize>());
            let y = y.wrapping_add(rand::random::<usize>());
            if x < LENGTH - 1 || y < HEIGHT - 1 {
                commands.insert((x, y), ((x + 1, y + 1), 0));
            }
        }
        map.push(row);
    }

    let timer = Instant::now();
    
    commands.par_iter().for_each(|((from_x, from_y), ((to_x, to_y), id))| {
        let mut from = map[*from_x][*from_y].write().unwrap();
        let mut to = map[*to_x][*to_y].write().unwrap();
        from.remove(id);
        to.insert(*id);
    });

    println!("time: {:?}", timer.elapsed());
}
