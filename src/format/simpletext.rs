use chrono::Datelike;

use crate::renderer::Renderer;
use crate::*;

pub struct SimpleText;
impl Renderer for SimpleText {
    // todo specify formatting
    fn render(&self, prescription: &Prescription) -> String {
        let mut rendered = format!("{}", prescription.medication.name);
        for (dosage, date, com) in prescription.doses.iter() {
            let dose_str = match dosage {
                Dosage::BeforeMeal([a, b, c, d]) => format!("\t{}-{}-{}-{}", a, b, c, d),
                Dosage::AfterMeal([a, b, c, d]) => format!("\t{}-{}-{}-{}", a, b, c, d),
                Dosage::Pause => format!("\t-pause"),
            };
            rendered.push_str(&dose_str);
            rendered.push_str(&format!(" [{}/{}-", date.month(), date.day()));
            if !com.is_empty() {
                rendered.push_str(&format!(" ({})]\n", com));
            } else {
                rendered.push_str("]\n");
            }
        }
        rendered
    }
}
