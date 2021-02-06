use chrono::Datelike;

use crate::renderer::Renderer;
use crate::*;

pub struct KText;
impl Renderer for KText {
    fn render(&self, prescription: &Prescription) -> String {
        let mut rendered = format!("{}\t", prescription.medication.name);
        let doses = prescription
            .doses
            .iter()
            .map(|(dosage, date, _)| {
                let date_string = format!("{}/{}-", date.month(), date.day());
                let dose_string = match dosage {
                    Dosage::BeforeMeal([a, b, c, d]) => format!("({}-{}-{}-{}mg)", a, b, c, d),
                    Dosage::AfterMeal([a, b, c, d]) => format!("({}-{}-{}-{}mg)", a, b, c, d),
                    Dosage::Pause => "pause".to_string(),
                };
                [date_string, dose_string].concat()
            })
            .collect::<Vec<_>>()
            .join(" → ");
        rendered.push_str(&doses);
        rendered
    }
}
