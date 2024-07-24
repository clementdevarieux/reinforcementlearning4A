
mod Env;
mod secret_env;
mod run_all_and_save;
use std::fs::File;
use std::io::{self, Read};
use crate::Env::GridWorld::GridWorld;
fn get_vector(chemin: &str) -> Vec<i32> {
    // Ouvrir le fichier
    let mut file = File::open(chemin).expect("Erreur lors de l'ouverture du fichier");

    // Lire le contenu du fichier dans une chaîne de caractères
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Erreur lors de la lecture du fichier");

    // Nettoyer et parser le contenu
    let contents = contents.trim(); // Supprime les espaces blancs autour
    let contents = &contents[1..contents.len()-1]; // Supprime les crochets '[' et ']'

    // Convertir la chaîne en vecteur de f64
    let pi_vector: Vec<i32> = contents
        .split(',')
        .map(|s| s.trim().parse().expect("Erreur lors du parsing d'un nombre"))
        .collect();

    pi_vector
}

fn main() {
    // run_all_and_save::run_all_and_save();

    let vector = get_vector("./results/Pi_values/6d9acd28-70a5-4bd0-a130-3fffbcac9173_2024-07-24_20-09-23_GridWorld_policy_iteration_2");

    let mut grid = Env::GridWorld::GridWorld::init();
    grid.run_game_vec(vector);
}