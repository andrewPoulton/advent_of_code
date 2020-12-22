use std::collections::{HashMap, HashSet};
use crate::utils::file2vec;

pub fn day21(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> =  contents.iter().map(|x| x.to_owned().unwrap()).collect();

    let mut allergens:HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingredients = HashMap::new();
    for line in &contents{
        let mut ingredient = String::from("");
        let mut allergen = String::from("");
        let mut ingredients = HashSet::new();
        let mut in_allergens = false;
        for ch in line.chars(){
            match ch {
                ' ' => {
                    if  !in_allergens{
                        ingredients.insert(ingredient.to_owned());
                        let counter = all_ingredients.entry(ingredient.to_owned()).or_insert(0);
                        *counter += 1;
                    }
                    ingredient.clear();
                    allergen.clear();
                },
                '(' => {
                    in_allergens = true;
                },
                ',' | ')' => {
                    if allergens.contains_key(&allergen){
                        allergens
                        .entry(allergen.clone())
                        .and_modify(|e|{
                            e.retain(|i| ingredients.contains(i));   
                        });
                    } else {
                        allergens.insert(allergen.clone(), ingredients.clone());
                    }
                    allergen.clear();
                },
                _ => {
                    if in_allergens{
                        allergen.push(ch);
                    } else {
                        ingredient.push(ch);
                    }
                }
            }
        }
    }

    let mut canonical_dangerous_ingredients = HashMap::new();
    let mut part1_ans = 0;
    let mut ingredient_map = HashMap::new();
    let mut keys= HashSet::new();
    all_ingredients.retain(|ingredient, count| {
        let mut allergen_count = 0;
        for allergen in allergens.keys(){

            let ingredients = allergens.get(allergen).unwrap();
            if ingredients.contains(ingredient){
                let i = canonical_dangerous_ingredients.entry(ingredient.clone()).or_insert(Vec::new());
                keys.insert(ingredient.clone());
                i.push(allergen);
                allergen_count+=1;
            }
        }
        part1_ans += *count;
        if allergen_count == 1{
            ingredient_map.insert(canonical_dangerous_ingredients.get(ingredient).unwrap()[0].clone(),ingredient.clone());
        }
        allergen_count == 0
    });

    let mut allergens_again = HashSet::new();
    while ingredient_map.len() < allergens.len(){
        for k in &keys{
            let c = canonical_dangerous_ingredients.get_mut(k).unwrap();
            if c.len() > 1 {
                c.retain(|e| {
                    !ingredient_map.contains_key(*e)
                } )
            }
            if c.len() == 1{
                ingredient_map.insert(c[0].to_owned(), k.to_owned());
                allergens_again.insert(c[0].to_owned());
                }
            }
        }
    
    let mut allergens_again: Vec<String> = allergens_again.into_iter().collect();
    allergens_again.sort();
 
    
    let mut part2_ans = String::from("");
    for allergen in &allergens_again{
        part2_ans.push_str(ingredient_map.get(allergen).unwrap());
        part2_ans.push(',');
    }
    part2_ans.pop();

    println!("part 1 ans: {:?}", part1_ans);
    println!("part 2 ans: {}", part2_ans);
}