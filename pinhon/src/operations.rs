pub struct Relation {
    pub(crate) plate: Vec<String>,
    pub(crate) pinhon: Vec<String>,
}

pub trait Calculo {
    fn new_pinhon(&mut self, pinhon_arg: String);
    fn new_plate(&mut self, plate_arg: String);
    fn show_relation(&self);
}

impl Calculo for Relation {
    fn new_pinhon(&mut self, pinhon_arg: String) {
        self.pinhon.push(pinhon_arg);
    }

    fn new_plate(&mut self, plate_arg: String) {
        self.plate.push(plate_arg);
    }

    fn show_relation(&self) {
        println!(
            "relación plato-pinhon : {:?} : {:?}",
            self.plate, self.pinhon
        )
    }
}
