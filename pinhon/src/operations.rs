struct Dientes {
    plate: Vec<String>,
    pinhon: Vec<String>,
}

pub trait Relation {
    fn new(plate: String, pinhon: String) -> Self;
    fn show_relation(&self);
}

impl Relation for Dientes {
    fn new(plate_arg: String, pinhon_arg: String) -> Self {
        let mut new_relation = Dientes {
            plate: Vec::new(),
            pinhon: Vec::new(),
        };
        new_relation.plate.push(plate_arg);
        new_relation.pinhon.push(pinhon_arg);
        new_relation
    }

    fn show_relation(&self) {
        println!("{:?} : {:?}", self.plate, self.pinhon)
    }
}
