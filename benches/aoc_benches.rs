use anyhow::{anyhow, Result};
use aoc::aoc2022::get_prob_func;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_tools::Itertools;
use std::fs;
use std::fs::File;
use std::io::Read;

struct AocDef {
    year: String,
    day: String,
    input: String,
}

fn get_defs_to_run(dir: &str) -> Result<Vec<AocDef>> {
    let des = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
    let res = des
        .into_iter()
        .filter(|de| !de.file_name().into_string().unwrap().contains("-test.txt"))
        .map(|de| {
            let file_name = de.file_name().into_string().unwrap();
            let (year, day) = file_name
                .strip_suffix(".txt")
                .ok_or_else(|| anyhow!("illegal suffix on file: {}", de.path().display()))?
                .split_once("-")
                .ok_or_else(|| anyhow!("illegal file name: {}", de.path().display()))?;
            let mut f = File::open(de.path())?;
            let mut input = String::new();
            f.read_to_string(&mut input)?;
            anyhow::Ok(AocDef {
                year: year.to_string(),
                day: day.to_string(),
                input,
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sorted_by_key(|d| format!("{}-{}", &d.year, &d.day))
        .collect();
    Ok(res)
}

fn aoc_benches(c: &mut Criterion) {
    let defs = get_defs_to_run("./inputs/").unwrap();
    for AocDef { year, day, input } in defs {
        for prob in ["1", "2"] {
            let mut prob_func = get_prob_func(&format!("{}-{}", &day, prob));
            c.bench_function(&format!("{}-{}-{}", &year, &day, prob), |b| {
                b.iter(|| prob_func(black_box(&input)))
            });
        }
    }
}

criterion_group!(benches, aoc_benches);
criterion_main!(benches);
