use chrono::NaiveDate;
use prescriptrack::renderer::Renderer;
use prescriptrack::*;
use renderer::SimpleTextRenderer;

fn main() {
    let meda = Medication {
        name: "お薬そのいち".to_string(),
    };
    let mut p = Prescription::new(meda);
    p.add_usage(
        Dosage::AfterMeal([0.0, 0.25, 0.25, 0.0]),
        NaiveDate::from_ymd(2021, 1, 1),
        "".to_string(),
    );
    p.add_usage(
        Dosage::AfterMeal([0.0, 0.25, 0.0, 0.25]),
        NaiveDate::from_ymd(2021, 1, 7),
        "晩に回した".to_string(),
    );
    p.add_usage(
        Dosage::AfterMeal([0.0, 0.125, 0.0, 0.125]),
        NaiveDate::from_ymd(2021, 1, 17),
        "へらした".to_string(),
    );
    println!("{}", SimpleTextRenderer.render(&p));
}

fn try_format() {
    let a = 1.0;
    let b = 0.25;
    let c = 0.3;
    let d = 0.4;
    let e = 0.25 / 16.0;
    println!("{} {} {} {} {}", a, b, c, d, e);
    println!("{:.2} {:.2} {:.2} {:.2} {:.2}", a, b, c, d, e);
}
