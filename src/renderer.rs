use crate::Prescription;

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
