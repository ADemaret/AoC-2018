use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

const VERBOSE: bool = false;

pub fn main() {
    println!("-- Advent of Code - Day 24 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day24_input_demo1.txt");
    let input = include_str!("../assets/day24_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }
    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
enum Side {
    #[default]
    Imm,
    Inf,
}

#[derive(Default, Debug)]
struct Group {
    id: usize,
    nice_id: usize,
    side: Side,
    units: usize,
    hit: usize,
    immun: Vec<String>,
    weak: Vec<String>,
    attack: usize,
    attacktype: String,
    init: usize,
    attacks: Option<usize>,
}

fn get_answer(input: &str) -> Option<usize> {
    let (imm, inf) = input.split_once("\n\n").unwrap();
    let mut groups = get_groups(imm, Side::Imm);
    groups.append(&mut get_groups(inf, Side::Inf));
    set_ids(&mut groups);

    let mut round = 1;
    loop {
        if VERBOSE {
            println!("--- round {} ---", round);
            for gr in &groups {
                if gr.units > 0 {
                    println!(
                        "{:?} group {} contains {} units, power is {}",
                        gr.side,
                        gr.nice_id,
                        gr.units,
                        gr.units * gr.attack
                    );
                }
            }
            println!("-");
        }

        //
        // target selection
        //
        if VERBOSE { println!("target selection :");}
        let sorted_groups = sort_groups_by_power(&groups);
        if sorted_groups.is_empty() {
            break;
        }
        for att_id in &sorted_groups {
            let mut worst_damage = 0;
            let mut worst_power = 0;
            for def_id in &sorted_groups {
                if att_id != def_id
                    && groups[*att_id].units > 0
                    && groups[*def_id].units > 0
                    && groups[*att_id].side != groups[*def_id].side
                    //&& !attacked.contains(&def_id)
                    && is_not_yet_attacked(&groups,*def_id)
                {
                    let damage = get_damage(&groups, *att_id, *def_id);
                    let def_power = get_power(&groups, *def_id);

                    if VERBOSE { println!(
                        "{:?} {} would deal def gr {}({}) {} damage",
                        groups[*att_id].side,
                        groups[*att_id].nice_id,
                        groups[*def_id].nice_id,
                        groups[*def_id].id,
                        damage
                    );}

                    if damage > worst_damage || (damage == worst_damage && def_power > worst_power)
                    {
                        worst_damage = damage;
                        worst_power = def_power;
                        groups[*att_id].attacks = Some(groups[*def_id].id);
                    }
                }
            }

            if VERBOSE { if let Some(def) = groups[*att_id].attacks {
                println!(
                    " => target will be {:?} {}",
                    groups[def].side, groups[def].nice_id
                );
            }}
        }
        if VERBOSE { println!("-");}

        //
        // attack
        //
        if VERBOSE { println!("attack :");}
        let sorted_groups_by_initiative = sort_groups_by_initiative(&groups);
        for att_id in sorted_groups_by_initiative {
            if let Some(attacks) = groups[att_id].attacks {
                // println!(" units : {}", groups[attacks].units);
                // println!(" hit/units : {}", groups[attacks].hit);
                let strongness = (groups[attacks].units * groups[attacks].hit) as isize;
                // println!(" strongness : {}", strongness);
                let damage = get_damage(&groups, att_id, attacks);
                // println!(" damage : {}",damage);
                // println!(" diff = {}", strongness - damage as isize);
                let not_killed = ((strongness as f32 - damage as f32) / groups[attacks].hit as f32)
                    .ceil() as usize;
                // println!(
                //     " not killed = {} => {}",
                //     (strongness as f32 - damage as f32) / groups[attacks].hit as f32,
                //     not_killed
                // );
                // println!(" killed = {}", groups[attacks].units - not_killed);
                let kills = groups[attacks].units - not_killed;
                if VERBOSE { println!(
                    "{:?} {} attacks group {}, killing {} units",
                    groups[att_id].side, groups[att_id].nice_id, groups[attacks].nice_id, kills
                );}
                if groups[attacks].units > kills {
                    groups[attacks].units -= kills;
                } else {
                    groups[attacks].units = 0;
                }
            }
            groups[att_id].attacks = None;
        }
        if VERBOSE { println!("-");}
        round += 1;
    }
    Some(
        groups
            .iter()
            .filter(|gr| gr.units > 0)
            .map(|gr| gr.units)
            .sum(),
    )
}

fn is_not_yet_attacked(groups: &[Group], id: usize) -> bool {
    for gr in groups {
        if gr.attacks == Some(id) {
            return false;
        }
    }
    true
}

fn get_power(groups: &[Group], id: usize) -> usize {
    groups[id].units * groups[id].attack
}

fn get_damage(groups: &[Group], att_id: usize, def_id: usize) -> usize {
    let mut damage = get_power(groups, att_id);
    if groups[def_id].immun.contains(&groups[att_id].attacktype) {
        damage = 0;
    } else if groups[def_id].weak.contains(&groups[att_id].attacktype) {
        damage *= 2;
    }
    damage
}

fn sort_groups_by_power(groups: &[Group]) -> Vec<usize> {
    let mut sorted_groups = Vec::new();
    let mut v = Vec::new();
    for gr in groups {
        if gr.units > 0 {
            v.push((gr.units * gr.attack, gr.init, gr.id));
        }
    }
    // println!("{:?}", v);
    v.sort_by_key(|gr| (gr.0, gr.1));
    v.reverse();

    let mut imm_army = 0;
    let mut inf_army = 0;
    for gr in groups {
        match gr.side {
            Side::Imm => {
                imm_army += gr.units * gr.attack;
            }
            Side::Inf => {
                inf_army += gr.units * gr.attack;
            }
        }
    }
    if imm_army == 0 || inf_army == 0 {
        if VERBOSE { println!("war is over");}
        return Vec::new();
    }

    if imm_army > inf_army {
        // println!("imm army choose first");
        for vv in &v {
            if groups[vv.2].side == Side::Imm {
                sorted_groups.push(vv.2)
            }
        }
        for vv in &v {
            if groups[vv.2].side == Side::Inf {
                sorted_groups.push(vv.2)
            }
        }
    } else {
        // println!("inf army choose first");
        for vv in &v {
            if groups[vv.2].side == Side::Inf {
                sorted_groups.push(vv.2)
            }
        }
        for vv in &v {
            if groups[vv.2].side == Side::Imm {
                sorted_groups.push(vv.2)
            }
        }
    }
    // println!("{:?}", sorted_groups);
    sorted_groups
}

fn sort_groups_by_initiative(groups: &[Group]) -> Vec<usize> {
    let mut sorted_groups = Vec::new();
    let mut v = Vec::new();
    for gr in groups {
        if gr.units > 0 {
            v.push((gr.init, gr.id));
        }
    }
    // println!("{:?}", v);
    v.sort_by_key(|gr| gr.0);
    v.reverse();
    for vv in v {
        sorted_groups.push(vv.1)
    }
    // println!("{:?}", sorted_groups);
    sorted_groups
}

fn set_ids(groups: &mut [Group]) {
    let mut nice_id = 1;
    let nbr_imm = groups
        .iter()
        .filter(|g| g.side == Side::Imm)
        .collect::<Vec<_>>()
        .len();
    for (id, gr) in groups.iter_mut().enumerate() {
        if gr.side == Side::Inf {
            nice_id = id - nbr_imm + 1;
        }
        gr.id = id;
        gr.nice_id = nice_id;
        nice_id += 1;
    }
}

fn get_groups(lines: &str, side: Side) -> Vec<Group> {
    lines
        .lines()
        .skip(1)
        .map(|line| {
            let chunks = line
                .split([' ', '(', ')', ';', ','])
                .filter(|&x| !x.is_empty())
                .collect::<Vec<_>>();
            // println!("{:?}", chunks);
            Group {
                side,
                units: chunks[0].parse::<usize>().unwrap(),
                hit: chunks[4].parse::<usize>().unwrap(),
                init: chunks[chunks.len() - 1].parse::<usize>().unwrap(),
                immun: get_properties(&chunks, "immune"),
                weak: get_properties(&chunks, "weak"),
                attack: chunks[chunks.len() - 6].parse::<usize>().unwrap(),
                attacktype: chunks[chunks.len() - 5].to_string(),
                ..Default::default()
            }
        })
        .collect::<Vec<_>>()
}

fn get_properties(chunks: &Vec<&str>, keyword: &str) -> Vec<String> {
    let mut gr = Vec::new();
    if chunks.contains(&keyword) {
        let pos = chunks.iter().position(|&c| c == keyword).unwrap();
        for ch in chunks.iter().skip(pos + 2) {
            if (["weak", "immune", "with"]).contains(ch) {
                break;
            } else {
                gr.push(ch.to_string());
            }
        }
    }
    gr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day24_input_demo1.txt")),
            Some(5216)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day24_input.txt")),
            Some(38008)
        );
    }
}
