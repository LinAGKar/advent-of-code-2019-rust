use std::collections::HashMap;
use std::io::Read;

struct Reaction<'a> {
    count: i64,
    sources: Vec<(i64, &'a str)>,
}

fn calc_ore_needed(reactions: &HashMap<&str, Reaction>, fuel_count: i64) -> i64 {
    let mut ore_needed = 0;
    let mut needed = vec![(fuel_count, "FUEL")];
    let mut surplus = HashMap::new();

    while let Some((needed_count, needed_resource)) = needed.pop() {
        let resource_surplus = surplus.remove(needed_resource).unwrap_or_default();
        let needed_count = needed_count - resource_surplus;
        let reaction = &reactions[needed_resource];

        let mut needed_reactions = needed_count / reaction.count;
        if needed_reactions < 0 {
            needed_reactions = 0;
        } else if needed_reactions * reaction.count < needed_count {
            needed_reactions += 1;
        }

        let resulting_count = needed_reactions * reaction.count;
        if resulting_count > needed_count {
            surplus.insert(needed_resource, resulting_count - needed_count);
        }

        if needed_reactions > 0 {
            for (source_count, source_resource) in &reaction.sources {
                let source_count = source_count * needed_reactions;
                if source_resource == &"ORE" {
                    ore_needed += source_count;
                } else {
                    needed.push((source_count, source_resource));
                }
            }
        }
    }

    ore_needed
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    fn parse_substance(string: &str) -> (i64, &str) {
        let mut split = string.trim().split(" ");
        let count = split.next().unwrap().trim().parse().unwrap();
        (count, split.next().unwrap().trim())
    }

    let reactions = input.lines().map(|x| {
        let mut sides = x.split("=>");
        let sources = sides.next().unwrap().split(", ").map(|y| parse_substance(y)).collect();
        let (count, target) = parse_substance(sides.next().unwrap());
        (target, Reaction { count: count, sources: sources })
    }).collect();

    const ORE_AVAILABLE: i64 = 1000000000000;

    let mut lower_bound = 0;
    let mut upper_bound = 16;

    while calc_ore_needed(&reactions, upper_bound) <= ORE_AVAILABLE {
        upper_bound *= 16;
    }

    while upper_bound > lower_bound + 1 {
        let fuel_to_test = lower_bound + (upper_bound - lower_bound) / 2;
        let ore_needed = calc_ore_needed(&reactions, fuel_to_test);
        if ore_needed > ORE_AVAILABLE {
            upper_bound = fuel_to_test;
        } else {
            lower_bound = fuel_to_test;
        }
    }

    println!("{}", lower_bound);
}
