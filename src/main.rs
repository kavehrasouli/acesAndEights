use std::collections::HashMap;
use std::collections::HashSet;

// generate all possible states (num of aces in each player's hand)
fn generate_states() -> Vec<[i32; 3]> { 
    let ad: (i32, i32, i32) = (2, 1, 0);     // Aces distribution
    let mut ps: Vec<[i32; 3]> = Vec::new(); // possible states

    for i in [ad.0, ad.1, ad.2] {  // loop over possible Aces dist
        let mut p: [i32; 3] = [0, 0, 0]; 
        p[0] = i;

        for j in [ad.0, ad.1, ad.2] {
            p[1] = j;

            for k in [ad.0, ad.1, ad.2] {

                p[2] = k;
                let s: i32 = p.iter().sum::<i32>();
                if s >= 2 && s <= 4 {
                    ps.push(p)
                }
                
            }
        }
    }
    return ps
}

// return true if player i cannot distinguish state s from state t
fn can_confuse(player: usize, s: [i32; 3], t: [i32; 3]) -> bool {
   for i in 0..3 {
        if i != player && s[i] != t[i] {
            return false;
        }
   }
   return true;
}

// return all states that player considers possible from state s.
fn get_accessible(player: usize, s: [i32; 3], states: &Vec<[i32; 3]>) -> Vec<[i32; 3]> {
    let mut pcps: Vec<[i32; 3]> = Vec::new(); // player i considers possible state
    for x in states {
        if can_confuse(player, s, *x) { // *x bc can_confuse expects [i32; 3] but x is &[i32; 3]
            pcps.push(*x)
        }
    }
    return pcps;
}



enum Formula {
    P(String),                          // a proposition
    NEG(Box<Formula>),                  // negation of a formula
    AND(Box<Formula>, Box<Formula>),    // conjuction
    K(usize, Box<Formula>),             // knowledge,
    Ann(Box<Formula>, Box<Formula>),    // announcement
}


// Take a proposition string and a state 
// and return whether it is true in that state
// prop example: p0_2 ->  player 0 has AA
//               p1_1 ->  player 1 has 8A
fn eval_prop(prop: &str, state: [i32; 3]) -> bool {
    let mut prop_chars = prop.chars();
    let first   = prop_chars.next().unwrap();   // the p
    let player  = prop_chars.next().unwrap().to_digit(10).unwrap() as usize;   // player number
    let third   = prop_chars.next().unwrap();   // the underline
    let num_ace = prop_chars.next().unwrap().to_digit(10).unwrap() as i32;   // number of aces

    return state[player] == num_ace
}

// The "Model". Includes:
// 1- All states -> Vec<[i32; 3]>
// The accessibility relations -> can_confuse -> so won't be included
// 3- The ToM map T -> maps each state to a set of (agent, level) pairs
struct Model {
    states:     Vec<[i32; 3]>,  // all states
    T:          HashMap<[i32; 3], HashSet<(usize, i32)>>,
}


fn involves_other_agent(agent: usize, formula: &Formula) -> bool {
    match formula {
        Formula::P(prop) => {
            return false;
        },
        Formula::NEG(f) => {
            return involves_other_agent(agent, f);
        },
        Formula::AND(f1, f2) => {
            return involves_other_agent(agent, f1) ||
                involves_other_agent(agent, f2);
        }
        Formula::K(j, f) => {
            // 1. if this K operator itself is for a different agent
            //    return true.
            // 2. recursive call: even if this K is for the same agent,
            //    maybe there is another agent's K nested deeper inside f.
            return *j != agent || involves_other_agent(agent, f);
        },
        Formula::Ann(f1, f2) => {
            return involves_other_agent(agent, f1) || involves_other_agent(agent, f2);
        },
    }
}

// announcement happens 
// → for each state, for each (agent, level) pair
//      → call satisfies to check if the announcement is true from that perspective
//      → if false, remove the pair
// → updated T is used for the next announcement
fn satisfies(model: &Model, player: usize, state: [i32; 3], 
            t_level: usize, formula: &Formula) -> bool {
    match formula {
        Formula::P(prop) => {
            return eval_prop(prop, state)
        },
        Formula::NEG(f) => {
            return !satisfies(model, player, state, t_level, f);
        },
        Formula::AND(f1, f2) => {
            return satisfies(model, player, state, t_level, f1) && 
            satisfies(model, player, state, t_level, f2);
        },
        Formula::K(j, f) => {
            // determine relevant level based on perspective switch
            let relevant_level = if *j == player {t_level as i32} else {t_level as i32 -1};
            // 1. all states that j can't distinguish from the current state
            let cd: Vec<[i32; 3]> = get_accessible(*j, state, &model.states);
            // 2. For each of those states, check if (j, relevant_level) is in T[t]
            for t in &cd {
                if model.T.get(t).unwrap().contains(&(*j, relevant_level)) {
                    // 3. If it is, call satisfies on f from j's perspective at relevant_level
                    if !satisfies(model, *j, *t, relevant_level as usize, f) {
                        return false;
                    }
                } 
            }
            return true;
        },
        Formula::Ann(f1, f2) => { // handle announcement
            // the complete new ToM map for all state
            let mut new_T: HashMap<[i32; 3], HashSet<(usize, i32)>> = HashMap::new();

            for (state, pairs) in &model.T {
                // the new set of (agent, level) pairs for one specific state
                let mut new_pairs: HashSet<(usize, i32)> = HashSet::new();
                for (agent, level) in pairs {
                    // decide whether to keep this (agent, level)
                    // check if f1 is true from this perspective
                    // also remember the special case for level 0
                    if *level == 0 && involves_other_agent(*agent, f1) {
                        new_pairs.insert((*agent, *level)); // keep unconditionally
                    } else if satisfies(model, *agent, *state, *level as usize, f1) {
                        new_pairs.insert((*agent, *level)); // keep if f1 is true
                    }
                }
                new_T.insert(*state, new_pairs);
            }
            // now check f2 with the updated model
            let model2 = Model {
                states: model.states.clone(), // keep the same states
                T: new_T,  // just update T
            };
            return satisfies(&model2, player, state, t_level, f2);
        },
    }
}

fn initialize_model() -> Model {
    let all_states: Vec<[i32; 3]> = generate_states();
    let mut T: HashMap<[i32; 3], HashSet<(usize, i32)>> = HashMap::new();
    for state in &all_states {
        let mut pairs: HashSet<(usize, i32)> = HashSet::new();
        for agent in 0..3 {
            for level in 0..=5 {
                pairs.insert((agent, level));
            }
        }
        T.insert(*state, pairs);
    }
    let model = Model {
        states: all_states,
        T: T
    };
    return model;
}

fn get_answer(model: &Model, player: usize, state: [i32; 3], tom_level: usize) -> bool {
    let s1 = format!("p{}_{}", player, 2);
    let s2 = format!("p{}_{}", player, 1);
    let s3 = format!("p{}_{}", player, 0);

    let knows_aa = satisfies(model, player, state, tom_level, &Formula::K(player, Box::new(Formula::P(s1))));
    let knows_8a = satisfies(model, player, state, tom_level, &Formula::K(player, Box::new(Formula::P(s2))));
    let knows_88 = satisfies(model, player, state, tom_level, &Formula::K(player, Box::new(Formula::P(s3))));

    let count = knows_aa as i32 + knows_8a as i32 + knows_88 as i32;
    return count == 1;
}

// Same logic as the Ann arm in satisfies.
// But returns the new model instead of a bool.
fn update_model(model: &Model, announcement: &Formula) -> Model {
    let mut new_T: HashMap<[i32; 3], HashSet<(usize, i32)>> = HashMap::new();
    for (state, pairs) in &model.T {
        let mut new_pairs: HashSet<(usize, i32)> = HashSet::new();
        for (agent, level) in pairs {
            if *level == 0 && involves_other_agent(*agent, announcement) {
                new_pairs.insert((*agent, *level));
            } else if satisfies(model, *agent, *state, *level as usize, announcement) {
                new_pairs.insert((*agent, *level));
            }
        }
        new_T.insert(*state, new_pairs);
    }
    return Model {
        states: model.states.clone(),
        T: new_T,
    };
}

fn game_loop(true_state: [i32; 3], tom_level: usize) {
    let model = initialize_model();
    let mut current_player = 0;

    loop {
        let T_before = model.T.clone();

        for player in 0..3 {
            current_player = player;
            let knows = get_answer(&model, current_player, true_state, tom_level);

            if knows {
                println!("Player {} knows their cards: {:?}", current_player, true_state[current_player]);
                return;
            } else {
                println!("Player {} does not know their cards.", current_player);
                // build the announcement formula: NOT K(player, their cards)
                let s0 = format!("p{}_{}", current_player, 2);
                let s1 = format!("p{}_{}", current_player, 1);
                let s2 = format!("p{}_{}", current_player, 0);
                // When a player says "I don't know my cards", they are publicly announcing:
                // "It is NOT the case that I know I have AA, And I know I have 8A, and I know I have 88".
                // Equivalent to: NEG(K(player, AA)) AND NEG(K(player, 8A)) AND NEG(K(player, 88))
                let announcement = 
                    Formula::AND(
                    Box::new(Formula::NEG(Box::new(Formula::K(current_player, Box::new(Formula::P(s0)))))),
                    Box::new(Formula::AND(
                    Box::new(Formula::NEG(Box::new(Formula::K(current_player, Box::new(Formula::P(s1)))))),
                    Box::new(Formula::NEG(Box::new(Formula::K(current_player, Box::new(Formula::P(s2)))))),
                )),
                );
                // update the model with ANN
                let mut model = update_model(&model, &announcement);
            }
        }
        if model.T == T_before {
            println!("No new info can be gained. The game is over!\n");
            break;
        }
    }
}


fn main() {
    let true_state = [2, 0, 0];
    let tom_level = 2;
    game_loop(true_state, tom_level);
}
