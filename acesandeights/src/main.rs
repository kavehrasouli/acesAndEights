// generate all possible states (num of aces in each player's hand)
fn generate_states() -> Vec<[i32; 3]> { 
    let ad: (i32, i32, i32) = (2, 1, 0);     // Aces distribution
    let mut ps: Vec<[i32; 3]> = Vec::new(); // possible states

    for i in [ad.0, ad.1, ad.2] {  // loop over possible Aces dist
        let mut p: [i32; 3] = [0, 0, 0]; 
        p[0] = i;

        for j in [ad.0, ad.1, ad.2] { // loop over Aces distribution
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


fn main() {
    let mut vec: Vec<[i32; 3]> = Vec::new();
    vec = generate_states();
    println!("{:?}", vec);
}
