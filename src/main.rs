use std::collections::HashMap;

/// Sorterer listen fra laveste til største værdi
pub fn sorter_liste(liste: &mut Vec<i32>) {
    liste.sort(); // Rusts indbyggede sortering er mere effektiv
}

/// Finder det største element i listen (hvis listen ikke er tom)
pub fn find_storste(liste: &Vec<i32>) -> Option<i32> {
    liste.last().copied()
}

/// Finder det mindste element i listen (hvis listen ikke er tom)
pub fn find_mindste(liste: &Vec<i32>) -> Option<i32> {
    liste.first().copied()
}

/// Finder typetallet i listen (mest forekommende tal)
pub fn find_typetal(liste: &Vec<i32>) -> Option<i32> {
    let mut frekvenser = HashMap::new();
    for &nummer in liste {
        *frekvenser.entry(nummer).or_insert(0) += 1;
    }

    frekvenser
        .into_iter()
        .max_by_key(|&(_, v)| v)
        .map(|(k, _)| k) // Returnerer tallet med højeste frekvens
}

/// Finder gennemsnittet af listen
pub fn find_middeltal(liste: &Vec<i32>) -> Option<f64> {
    if liste.is_empty() {
        return None;
    }
    let sum: i32 = liste.iter().sum();
    Some(sum as f64 / liste.len() as f64)
}

/// Finder medianen i listen (midterste værdi i en sorteret liste)
pub fn find_median(liste: &Vec<i32>) -> Option<f64> {
    if liste.is_empty() {
        return None;
    }

    let mid_index = liste.len() / 2;
    Some(if liste.len() % 2 == 0 {
        (liste[mid_index - 1] + liste[mid_index]) as f64 / 2.0
    } else {
        liste[mid_index] as f64
    })
}

/// Finder variationsbredden (største - mindste værdi)
pub fn find_variationsbredde(liste: &Vec<i32>) -> Option<i32> {
    Some(find_storste(liste)? - find_mindste(liste)?)
}

/// Finder hyppighed og frekvens af hvert tal i listen
pub fn find_hyppighed_frekvens(liste: &Vec<i32>) -> Vec<(i32, usize, f64)> {
    let mut frekvenser = HashMap::new();
    let total = liste.len() as f64;
    
    for &nummer in liste {
        *frekvenser.entry(nummer).or_insert(0) += 1;
    }
    
    let mut resultat: Vec<_> = frekvenser.into_iter()
        .map(|(tal, hyppighed)| (tal, hyppighed, hyppighed as f64 / total))
        .collect();
    
    resultat.sort_by_key(|&(tal, _, _)| tal);
    resultat
}

fn main() {
    let mut liste = vec![5, 5, 6, 4, 5, 4, 5, 3, 7, 6, 7, 3, 3, 2, 6, 5];
    println!("Datasæt:          {:?}", liste);

    sorter_liste(&mut liste);
    println!("Sorteret liste:   {:?}", liste);
    println!("Største værdi:    {}", find_storste(&liste).unwrap_or(0));
    println!("Mindste værdi:    {}", find_mindste(&liste).unwrap_or(0));
    println!("Typetal:          {}", find_typetal(&liste).unwrap_or(0));
    println!("Middeltal:        {:.2}", find_middeltal(&liste).unwrap_or(0.0));
    println!("Median:           {:.2}", find_median(&liste).unwrap_or(0.0));
    println!("Variationsbredde: {}", find_variationsbredde(&liste).unwrap_or(0));
    
    let hyppighed_frekvens = find_hyppighed_frekvens(&liste);
    println!("\nHyppighed og frekvens:");
    for (tal, hyppighed, frekvens) in &hyppighed_frekvens {
        println!("Tal: {:<2}  | Hyppighed: {:<2}  | Frekvens: {:.2}", tal, hyppighed, frekvens * 100.0);
    }
}
