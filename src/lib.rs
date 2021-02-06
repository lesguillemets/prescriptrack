use chrono::NaiveDate;
pub mod format;
pub mod renderer;

pub fn hello() {
    println!("Hello, world");
}

#[derive(Debug)]
pub struct Prescription {
    medication: Medication,
    doses: Vec<(Dosage, NaiveDate, Comment)>,
}

impl Prescription {
    pub fn new(med: Medication) -> Self {
        Prescription {
            medication: med,
            doses: vec![],
        }
    }
    pub fn add_usage(&mut self, dosage: Dosage, date: NaiveDate, comment: Comment) {
        self.doses.push((dosage, date, comment));
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Medication {
    pub name: String,
}

pub type Dose = f64;
pub type Comment = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Dosage {
    BeforeMeal([Dose; 4]), // breakfast-lunch-dinner-before_sleep
    AfterMeal([Dose; 4]),
    Pause,
}

impl Dosage {
    fn daily_dose(&self) -> Dose {
        match self {
            Dosage::BeforeMeal(doses) => doses.iter().sum(),
            Dosage::AfterMeal(doses) => doses.iter().sum(),
            Dosage::Pause => 0.0,
        }
    }
}
