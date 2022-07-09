use std::collections::HashMap;

pub fn begin(part: u8, input: String) -> u32{
    let (template, rules)  = handle_input(input);
    println!("{}", template);
    match part {
        1 => return part_1::begin(&template, &rules),
        2 => return part_2::begin(&template, &rules),
        _ => panic!("Invalid part entered.")
    }
    0
}

fn handle_input(input: String) -> (String, HashMap<String, char>) {
    let mut rules = HashMap::new();
    let mut template = String::new();
    for (i, line) in input.lines().enumerate() {
        if i > 1 {
            let mut rule = line.split("-> ");
            rules.insert(String::from(rule.next().unwrap().trim()), rule.next().unwrap().chars().next().unwrap());
        }else if i == 0 {
            template = String::from(line);
        }
    }
    (template, rules)
}

pub fn get_next_template(template: &str, rules: &HashMap<String, char>) -> String {
    
    let mut curr_temp = String::from(template);
    let mut next_temp = String::new();
    
    let mut prev = None;
    for ch in curr_temp.chars() {
        if prev == None {
            prev = Some(ch);
        }else{
            let mut key = String::new();
            key.push(prev.unwrap());
            key.push(ch); 
            next_temp.push(prev.unwrap());
            //println!("key: {}", key);
            //println!("value: {}", rules.get(&key).unwrap());
            next_temp.push(*rules.get(&key).unwrap());
            //println!("{}", next_temp);
            prev = Some(ch);
        }
    }
    next_temp.push(curr_temp.chars().last().unwrap());
    next_temp

}

mod part_1 {

    use std::collections::HashMap;
    use super::get_next_template;

    pub fn begin(template: &str, rules: &HashMap<String, char>) -> u32 {
        let mut template = String::from(template);
        for iter in 1..11 {
            template = get_next_template(&template, &rules);
            println!("Step {} Complete", iter);
        }
        let mut counts = HashMap::new();
        for ch in template.chars() {
            let count = counts.entry(ch).or_insert(0);
            *count += 1;
        }
        let mut largest = 0;
        let mut smallest: u32 = 10000000;
        for ch in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']{
            let val = counts.get(&ch);
            match val {
                Some(x) => {
                    if (x < &smallest) {smallest = *x};
                    if (x > &largest) {largest = *x};
                },
                _ => ()
            }
        }
        
        largest - smallest
    }

}

mod part_2 {

    use std::collections::HashMap;
    use super::get_next_template;
    
    pub fn begin(template: &str, rules: &HashMap<String, char>) -> u32 {
        let mut template = String::from(template);
        for iter in 1..41 {
            template = get_next_template(&template, &rules);
            println!("Step {} Complete", iter);
        }
        let mut counts = HashMap::new();
        for ch in template.chars() {
            let count = counts.entry(ch).or_insert(0);
            *count += 1;
        }
        let mut largest =  0;
        let mut smallest = std::u128::MAX;
        for ch in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']{
            let val = counts.get(&ch);
            match val {
                Some(x) => {
                    if (x < &smallest) {smallest = *x};
                    if (x > &largest) {largest = *x};
                },
                _ => ()
            }
        }
        (largest - smallest) as u32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn handle_sample_input() {
        let input = String::from(
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C");

    let (template, rules) = super::handle_input(input);
    assert_eq!(template, String::from("NNCB"));

    }

    #[test]
    fn generate_second_template() {
        let input = String::from(
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C");
    let (template, rules) = super::handle_input(input);

    part_1::begin(&template, &rules);
    }
}



