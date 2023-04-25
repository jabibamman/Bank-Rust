#[derive(Debug, PartialEq)]
pub struct Operation {
    pub libelle: String,
    pub montant: f64,
}

impl Operation {
    pub fn new(libelle: String, montant: f64) -> Self {
        Operation { libelle, montant }
    }

    pub fn get_libelle(&self) -> &String {
        &self.libelle
    }

    pub fn get_montant(&self) -> f64 {
        self.montant
    }

}