use chrono::NaiveDate;
pub fn hello() {
    println!("Hello, world");
}

struct Medication {
    name: String,
}

type Dose = f64;
type Comment = String;

enum Dosage {
    BeforeMeal([Dose; 4]), // breakfast-lunch-dinner-before_sleep
    AfterMeal([Dose; 4]),
}

impl Dosage {
    fn daily_dose(&self) -> Dose {
        match self {
            Dosage::BeforeMeal(doses) => doses.iter().sum(),
            Dosage::AfterMeal(doses) => doses.iter().sum(),
        }
    }
}

struct Prescription {
    medication: Medication,
    doses: Vec<(Dosage, NaiveDate, Comment)>,
}
