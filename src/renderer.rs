use crate::*;
use chrono::Datelike;

pub trait Renderer {
    fn render(&self, prescription: &Prescription) -> String;
    fn render_all(&self, prescriptions: &[Prescription]) -> String {
        let mut rendered = String::from("");
        for p in prescriptions {
            rendered.push_str(&self.render(p));
        }
        rendered
    }
}

pub struct SimpleTextRenderer;

impl Renderer for SimpleTextRenderer {
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

pub struct KTextRenderer;
impl Renderer for KTextRenderer {
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
