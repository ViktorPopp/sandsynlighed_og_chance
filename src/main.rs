use std::collections::HashMap;

/// Sorterer listen fra lavest til største værdi
pub fn sorter_liste(liste: &mut Vec<i32>) {
    for i in 1..liste.len() {
        let key = liste[i];
        let mut j = i as isize - 1;

        // Flyt elementer, der er større end key, en position frem
        while j >= 0 && liste[j as usize] > key {
            liste[(j + 1) as usize] = liste[j as usize];
            j -= 1;
        }

        liste[(j + 1) as usize] = key;
    }
}

/// Finder det største element i listen
pub fn find_storste(liste: &Vec<i32>) -> i32 {
    *liste.last().unwrap()
}

/// Finder det mindste element i listen
pub fn find_mindste(liste: &Vec<i32>) -> i32 {
    *liste.first().unwrap()
}

/// Finder typetallet i listen
///
/// Typetallet er den værdi der optræder flest gange i datasættet
pub fn find_typetal(liste: &Vec<i32>) -> i32 {
    let mut frekvenser = HashMap::new();
    for &nummer in liste {
        *frekvenser.entry(nummer).or_insert(0) += 1;
    }

    let max_frekvens = *frekvenser.values().max().unwrap();
    // Find et tal med den højeste frekvens (det første, der opstår med den største frekvens)
    *frekvenser
        .iter()
        .find(|&(_, &v)| v == max_frekvens)
        .map(|(k, _)| k)
        .unwrap()
}

/// Finder middeltallet i listen
///
/// Middeltallet er den værdi der findes i midten af listen
pub fn find_middeltal(liste: &Vec<i32>) -> f64 {
    let sum: i32 = liste.iter().sum();
    sum as f64 / liste.len() as f64
}

/// Finder medianen i listen
///
/// Medianen er også kaldet gennensnittet. Den findes ved at plusse alle værdierne fra
/// datasættet sammen og derefter dividere med længden af datasættet
pub fn find_median(liste: &Vec<i32>) -> f64 {
    let mid_index = liste.len() / 2;
    if liste.len() % 2 == 0 {
        (liste[mid_index - 1] + liste[mid_index]) as f64 / 2.0
    } else {
        liste[mid_index] as f64
    }
}

/// Finder variationsbredden i listen
///
/// Variationsbredden er den største værdi minus den mindste værdi
pub fn find_variationsbredde(liste: &Vec<i32>) -> i32 {
    find_storste(liste) - find_mindste(liste)
}

fn main() {
    let mut liste = vec![37, 42, 39, 39, 39, 42, 39, 44, 38, 41, 43, 37];

    // Sorter listen for at finde statistikker korrekt
    sorter_liste(&mut liste);

    println!("Største værdi:    {}", find_storste(&liste));
    println!("Mindste værdi:    {}", find_mindste(&liste));
    println!("Typetal:          {:?}", find_typetal(&liste));
    println!("Middeltal:        {:.2}", find_middeltal(&liste));
    println!("Median:           {:.2}", find_median(&liste));
    println!("Variationsbredde: {}", find_variationsbredde(&liste));
}
