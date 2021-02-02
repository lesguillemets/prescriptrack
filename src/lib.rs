use chrono::NaiveDate;
pub fn hello() {
    println!("Hello, world");
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Medication {
    name: String,
}

type Dose = f64;
type Comment = String;

#[derive(Clone, Debug, PartialEq)]
enum Dosage {
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

#[derive(Debug)]
pub struct Prescription {
    medication: Medication,
    doses: Vec<(Dosage, NaiveDate, Comment)>,
}

impl Prescription {
    fn add_usage(&mut self, dosage: Dosage, date: NaiveDate, comment: Comment) {
        self.doses.push((dosage, date, comment));
    }
}
