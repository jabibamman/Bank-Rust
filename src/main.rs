use bank_rust::bank::{Compte, Operation};

fn main() {

    let mut compte = Compte::new("123456789".to_string(), 1000.0);

    println!("Compte: {:?}", compte);
    println!("Solde initial: {}", compte.get_solde());

    compte.deposit(100.0);
    println!("Solde après dépôt de 100.0: {}", compte.get_solde());

    match compte.withdraw(50.0) {
        Ok(_) => println!("Retrait réussi."),
        Err(e) => println!("{}", e),
    }

    println!("Solde après retrait de 50.0: {}", compte.get_solde());

    match compte.withdraw(100000.0) {
        Ok(_) => println!("Retrait réussi."),
        Err(e) => println!("{}", e),
    }

    println!("Historique des opérations :");
    for (index, operation) in compte.get_operations().iter().enumerate() {
        println!("{}: {:?}", index + 1, operation);
    }


}
