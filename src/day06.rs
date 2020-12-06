pub fn day06() {
    let contents = std::fs::read_to_string("in/day06")
        .expect("File error");

    let lines = contents.lines();

    let mut any_yes = std::collections::HashSet::new();
    let mut any_yes_sets = Vec::new();

    let mut set_initialized = false;

    for line in lines {
        if line == "" {
            any_yes_sets.push(any_yes);

            any_yes = std::collections::HashSet::new();

            set_initialized = false;

            continue;
        }
        let mut yes_answers = std::collections::HashSet::new();
        for c in line.chars() {
            yes_answers.insert(c);
        }
        if !set_initialized {
            any_yes = yes_answers.iter().copied().collect();
            set_initialized = true;
        }
        else {
            any_yes = any_yes.union(&yes_answers).cloned().collect();
        }
    }
    any_yes_sets.push(any_yes);

    let mut sum = 0;

    for set in any_yes_sets {
        sum += set.len();
    }
    println!("Part 1: {}", sum);

}