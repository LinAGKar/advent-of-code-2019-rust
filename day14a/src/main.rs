use std::collections::HashMap;
use std::io::Read;

struct Reaction<'a> {
    count: i32,
    sources: Vec<(i32, &'a str)>,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    fn parse_substance(string: &str) -> (i32, &str) {
        let mut split = string.trim().split(" ");
        let count = split.next().unwrap().trim().parse().unwrap();
        (count, split.next().unwrap().trim())
    }

    let reactions: HashMap<_, _> = input.lines().map(|x| {
        let mut sides = x.split("=>");
        let sources = sides.next().unwrap().split(", ").map(|y| parse_substance(y)).collect();
        let (count, target) = parse_substance(sides.next().unwrap());
        (target, Reaction { count: count, sources: sources })
    }).collect();

    let mut ore_needed = 0;
    let mut needed = vec![(1, "FUEL")];
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

    println!("{}", ore_needed);
}
