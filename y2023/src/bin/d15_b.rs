use std::collections::HashMap;

use y2023::{
    get_lines,
    util::d15::{simple_hash, D15Error},
};

fn solve(fp: &str) -> Result<u64, D15Error> {
    let Some(line) = get_lines(fp)?.next() else {
        return Err(D15Error::EmptyInput);
    };

    let mut vec_map: HashMap<u8, Vec<(String, u8)>> = HashMap::new();
    for s in line?.split(',') {
        if let Some((label, lens)) = s.split_once('=') {
            let hashed = simple_hash(label);
            let label = label.to_string();
            let lens = lens.parse()?;
            if let Some(v) = vec_map.get_mut(&hashed) {
                let mut found = false;
                let iter = v.iter_mut();
                for item in iter {
                    if item.0 == label {
                        item.1 = lens;
                        found = true;
                        break;
                    }
                }
                if !found {
                    v.push((label, lens));
                }
            } else {
                let v = vec![(label, lens)];
                vec_map.insert(hashed, v);
            }
        } else if let Some((label, _)) = s.split_once('-') {
            let hashed = simple_hash(label);
            let label = label.to_string();
            if let Some(v) = vec_map.get_mut(&hashed) {
                for (i, item) in v.iter_mut().enumerate() {
                    if item.0 == label {
                        v.remove(i);
                        break;
                    }
                }
            }
        } else {
            return Err(D15Error::InvalidOperation(s.to_string()));
        }
    }

    Ok(vec_map
        .iter()
        .map(|(h, v)| {
            v.iter()
                .enumerate()
                .map(|(i, (_, l))| (i as u64 + 1) * (*h as u64 + 1) * (*l as u64))
                .sum::<u64>()
        })
        .sum())
}

fn main() {
    match solve("data/d15/a.txt") {
        Ok(sol) => println!("{}", sol),
        Err(e) => println!("Error: {}", e),
    }
}
