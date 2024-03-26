use std::io;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, Sink, OutputStreamHandle, OutputStream};

mod exercice1;
mod exercice2;
mod exercice3;
mod exercice4;
mod exercice5;
mod exercice6;

fn play_sound(file_path: &str, stream: &OutputStreamHandle) {
    // Charger le fichier audio
    let file = File::open(file_path).expect("Erreur lors de l'ouverture du fichier audio");
    let source = Decoder::new(BufReader::new(file)).expect("Erreur lors de la création du décodeur audio");

    // Création d'un lecteur audio pour jouer le son
    let sink = Sink::try_new(stream).expect("Erreur lors de la création du lecteur audio");
    sink.append(source);
    sink.sleep_until_end();
}

fn main() {
    // Obtenez le gestionnaire de sortie audio par défaut
    let (_stream, handle) = OutputStream::try_default().expect("Impossible d'obtenir le flux de sortie par défaut");

    // Affichage de la pluie de petits canards
    println!("Préparez-vous pour une pluie de canards !");
    for _ in 0..5 {
        println!("    _");
        println!(" __(.)<");
        println!(" \\___)");
        println!();
        thread::sleep(Duration::from_millis(200)); // Pause entre chaque canard
    }
    println!(); // Nouvelle ligne après la pluie de canards

    // Affichage du menu
    println!("Bienvenue !");
    println!("Choisissez un exercice à exécuter :");
    println!("1. Exercice 1 ");
    println!("2. Exercice 2");
    println!("3. Exercice 3");
    println!("4. Exercice 4");
    println!("5. Exercice 5");
    println!("6. Exercice 6");

    // Lecture du choix de l'utilisateur
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Erreur de lecture de la ligne");

    // Traitement du choix de l'utilisateur
    match choix.trim() {
        "1" => {
            // Jouer le son du canard
            play_sound("src/canard1.mp3", &handle);
            exercice1::main();
        }
        "2" => {
            // Jouer le son du canard
            play_sound("src/canard2.mp3", &handle);
            exercice2::main();
        }
        "3" => {
            // Jouer le son du canard
            play_sound("src/canard3.mp3", &handle);
            exercice3::main();
        }
        "4" => {
            // Jouer le son du canard
            play_sound("src/canard4.mp3", &handle);
            exercice4::main();
        }
        "5" => {
            // Jouer le son du canard
            play_sound("src/canard5.mp3", &handle);
            exercice5::main();
        }
        "6" => {
            // Jouer le son du canard
            play_sound("src/canard6.mp3", &handle);
            exercice6::main();
        }
        _ => println!("Choix invalide"),
    }
}
