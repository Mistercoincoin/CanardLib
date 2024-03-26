use std::io;

// Structure pour représenter un livre
struct Livre {
    titre: String,
    genre: String,
    annee_publication: u32,
}

pub fn main() {
    // Création de quelques livres pour notre base de données
    let livres = vec![
        Livre { titre: "Guerre et Paix".to_string(), genre: "Roman".to_string(), annee_publication: 1869 },
        Livre { titre: "Orgueil et Préjugés".to_string(), genre: "Roman".to_string(), annee_publication: 1813 },
        Livre { titre: "1984".to_string(), genre: "Science-Fiction".to_string(), annee_publication: 1949 },
        Livre { titre: "Le Seigneur des Anneaux".to_string(), genre: "Fantasy".to_string(), annee_publication: 1954 },
        Livre { titre: "Les Misérables".to_string(), genre: "Roman".to_string(), annee_publication: 1862 },
        Livre { titre: "Le Petit Prince".to_string(), genre: "Fantasy".to_string(), annee_publication: 1943 },
        Livre { titre: "Le Vieil Homme et la Mer".to_string(), genre: "Roman".to_string(), annee_publication: 1952 },
        Livre { titre: "Da Vinci Code".to_string(), genre: "Thriller".to_string(), annee_publication: 2003 },
        Livre { titre: "Harry Potter à l'école des sorciers".to_string(), genre: "Fantasy".to_string(), annee_publication: 1997 },
        Livre { titre: "Jurassic Park".to_string(), genre: "Science-Fiction".to_string(), annee_publication: 1990 },
        Livre { titre: "Anges et Démons".to_string(), genre: "Thriller".to_string(), annee_publication: 2000 },
        Livre { titre: "Le Lion, la Sorcière blanche et l'Armoire magique".to_string(), genre: "Fantasy".to_string(), annee_publication: 1950 },
    ];

    // Extraire la liste des genres disponibles à partir des livres
    let genres: Vec<String> = livres.iter().map(|livre| livre.genre.clone()).collect();

    // Supprimer les doublons de la liste des genres
    let mut genres_uniques: Vec<String> = Vec::new();
    for genre in genres {
        if !genres_uniques.contains(&genre) {
            genres_uniques.push(genre);
        }
    }

    // Demander à l'utilisateur de choisir un genre parmi les genres disponibles
    println!("Choisissez un genre parmi les suivants :");
    for (index, genre) in genres_uniques.iter().enumerate() {
        println!("{}. {}", index + 1, genre);
    }

    // Lire l'entrée de l'utilisateur pour choisir le genre
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Convertir l'entrée en nombre entier
    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrée invalide.");
            return;
        }
    };

    // Vérifier si l'index est valide et afficher les livres correspondants
    match index {
        1.. => {
            let genre_recherche = &genres_uniques[index - 1];

            // Afficher la liste des livres du genre recherché, triés par année de publication
            println!("Liste des livres de genre {} :", genre_recherche);
            for livre in livres.iter().filter(|&livre| livre.genre == *genre_recherche) {
                println!("{} ({})", livre.titre, livre.annee_publication);
            }
        },
        _ => {
            println!("Genre invalide.");
        }
    }
}
