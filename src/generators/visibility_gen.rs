use rustdoc_types::Visibility;

pub struct VisibilityGenerator;

impl VisibilityGenerator {
    pub fn generate_visibility(visibility: &Visibility) -> String {
        let mut visibility_string = String::new();
        match visibility {
            Visibility::Public => visibility_string.push_str("pub "),
            Visibility::Default => {}
            Visibility::Crate => visibility_string.push_str("pub(crate) "),
            Visibility::Restricted { parent, path } => {
                // TODO: Utilize parent
                visibility_string.push_str("pub(");
                visibility_string.push_str(path);
                visibility_string.push_str(") ");
            }
        }
        visibility_string
    }
}
