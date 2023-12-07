use common::*;

const IDENTS: &[(&str, u32)] = &[
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];

fn main() -> Result<()> {
    let input = setup()?;

    let res1 = input
        .lines()
        .map(|line| {
            let mut digits = line
                .chars()
                .filter(|char| char.is_numeric())
                .map(|char| char.to_string().parse::<u32>().unwrap());

            let first = digits.next().unwrap_or_else(|| {
                debug!("Didn't file value");
                return 0;
            });
            let last = digits.rev().next().unwrap_or_else(|| first);
            debug!(?first, ?last);

            return first * 10 + last;
        })
        .sum::<u32>();

    info!(?res1);

    let res2 = input
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            let mut digits = Vec::new();

            while !line.is_empty() {
                let mut found = false;
                for &(ident, value) in IDENTS {
                    debug!(?line, ?ident, ?digits, "Checking");
                    if let Some(_) = line.strip_prefix(ident) {
                        let l = ident.chars().count();
                        line.drain(0..l);
                        digits.push(value);
                        found = true;
                    }
                }
                if !found {
                    line.drain(..1);
                }
            }

            let mut digits = digits.into_iter();
            let first = digits.next().expect("Getting first value");
            let last = digits.rev().next().unwrap_or_else(|| first);
            debug!(?first, ?last);

            return first * 10 + last;
        })
        .sum::<u32>();

    info!(?res2);

    Ok(())
}
