use std::io;

// Structure pour représenter un livre
struct Livre {
    titre: String,
    annee_publication: u32,
}

// Fonction pour vérifier si un livre est un classique
fn est_classique(livre: &Livre) -> bool {
    let annee_actuelle = 2024; // On suppose que l'année actuelle est 2024
    let annee_limite = annee_actuelle - 50;

    // Vérifier si le livre a été publié il y a plus de 50 ans et est toujours lu et apprécié aujourd'hui
    livre.annee_publication <= annee_limite
}

pub fn main() {
    // Création de quelques livres pour notre base de données
    let livres = vec![
        Livre { titre: "Guerre et Paix".to_string(), annee_publication: 1869 },
        Livre { titre: "Orgueil et Préjugés".to_string(), annee_publication: 1813 },
        Livre { titre: "1984".to_string(), annee_publication: 1949 },
        Livre { titre: "Le Seigneur des Anneaux".to_string(), annee_publication: 1954 },
        Livre { titre: "Les Misérables".to_string(), annee_publication: 1862 },
        Livre { titre: "Le Petit Prince".to_string(), annee_publication: 1943 },
        Livre { titre: "Le Vieil Homme et la Mer".to_string(), annee_publication: 1952 },
        Livre { titre: "Harry Potter et la Pierre Philosophale".to_string(), annee_publication: 1997 },
        Livre { titre: "Le Da Vinci Code".to_string(), annee_publication: 2003 },
        Livre { titre: "Millénium, Tome 1 : Les Hommes qui n'aimaient pas les femmes".to_string(), annee_publication: 2005 },
        Livre { titre: "Cinquante nuances de Grey".to_string(), annee_publication: 2011 },
        Livre { titre: "Twilight, Tome 1 : Fascination".to_string(), annee_publication: 2005 },
    ];

    // Affichage de la liste des livres disponibles
    println!("Liste des livres disponibles :");
    for (index, livre) in livres.iter().enumerate() {
        println!("{}. {}", index + 1, livre.titre);
    }

    // Demander à l'utilisateur de choisir un livre par son numéro
    println!("Entrez le numéro du livre que vous souhaitez vérifier :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index: usize = input.trim().parse().expect("Invalid input");

    // Vérifier si l'index est valide
    if index > 0 && index <= livres.len() {
        let livre = &livres[index - 1];

        // Vérifier si le livre est un classique et afficher le résultat
        if est_classique(livre) {
            println!("{} est un classique!", livre.titre);
        } else {
            println!("{} n'est pas un classique.", livre.titre);
        }
    } else {
        println!("Numéro de livre invalide.");
    }
}
