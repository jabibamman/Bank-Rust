use crate::bank::Operation;

#[derive(Debug, PartialEq)]
pub struct Compte {
    pub numero: String,
    pub solde: f64,
    pub operations: Vec<Operation>,
}

impl Compte {
    pub fn new(numero: String, solde: f64) -> Self {
        Compte {
            numero,
            solde,
            operations: Vec::new(),
        }
    }

    pub fn get_numero(&self) -> &String {
        &self.numero
    }

    pub fn get_solde(&self) -> f64 {
        self.solde
    }

    pub fn deposit(&mut self, montant: f64) {
        if montant < 0.0 {
            println!("Le montant doit être positif");
        }
        self.solde += montant;
        self.ajouter_operation(Operation::new("Dépôt".to_string(), montant));
    }

    pub fn withdraw(&mut self, montant: f64) -> Result<(), String> {
        if montant > 0.0 {
            if self.solde >= montant {
                self.solde -= montant;
                self.ajouter_operation(Operation::new("Retrait".to_string(), -montant));
                Ok(())
            } else {
                Err("Solde insuffisant.".to_string())
            }
        } else {
            Err("Le montant doit être positif".to_string())
        }
    }

    pub fn calculer_solde(&self) -> f64 {
        let mut solde = self.solde;
        for operation in &self.operations {
            solde += operation.montant;
        }
        solde
    }

    pub fn ajouter_operation(&mut self, operation: Operation) {
        self.operations.push(operation);
    }

    pub fn get_operations(&self) -> &[Operation] {
        &self.operations
    }

}