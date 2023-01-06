use seedwing_policy_engine::value::{InputValue, Rationale, RationaleResult};
use std::borrow::Borrow;

pub struct Rationalizer {
    value: RationaleResult,
}

impl Rationalizer {
    pub fn new(value: RationaleResult) -> Self {
        Self { value }
    }

    pub fn rationale(&self) -> String {
        let mut html = String::new();
        html.push_str("<div>");
        match &self.value {
            RationaleResult::None => {
                html.push_str("failed");
            }
            RationaleResult::Same(value) => {
                let locked_value = (**value).borrow();
                Self::rationale_inner(&mut html, locked_value);
            }
            RationaleResult::Transform(value) => {
                let locked_value = (**value).borrow();
                Self::rationale_inner(&mut html, locked_value);
            }
        }

        html.push_str("<div>");
        html
    }

    pub fn rationale_inner<'h>(html: &'h mut String, value: &'h InputValue) {
        let rationale = value.get_rationale();
        if rationale.is_empty() {
            return;
        }
        let value_json = value.as_json();
        let value_json = serde_json::to_string_pretty(&value_json).unwrap();
        let value_json = value_json.replace('<', "&lt;");
        let value_json = value_json.replace('>', "&gt;");
        html.push_str("<div>");
        html.push_str("<h2>Input:</h2>");
        html.push_str("<pre class='input-value'>");
        html.push_str(value_json.as_str());
        html.push_str("</pre>");
        for (k, v) in rationale.iter().rev() {
            match v {
                RationaleResult::None => {
                    if let Some(description) = k.description() {
                        html.push_str("<div class='entry no-match'>");
                        html.push_str(format!("did not match {}\n", description).as_str());
                        html.push_str("</div>");
                    }
                }
                RationaleResult::Same(inner) => {
                    if let Some(description) = k.description() {
                        html.push_str("<div class='entry match'>");
                        html.push_str(
                            format!("<h2><code>{}</code> matched</h2>", description).as_str(),
                        );
                        if let Some(list) = inner.try_get_list() {
                            for item in list {
                                Self::rationale_inner(html, (**item).borrow());
                            }
                        }
                        if let Some(object) = inner.try_get_object() {
                            for (field_name, v) in object.iter() {
                                html.push_str("<div>");
                                html.push_str(
                                    format!("<b>field <code>{}</code></b>", field_name).as_str(),
                                );
                                Self::rationale_inner(html, (**v).borrow());
                                html.push_str("</div>");
                            }
                        }
                        html.push_str("</div>");
                    }
                }
                RationaleResult::Transform(transform) => {
                    if let Some(description) = k.description() {
                        html.push_str("<div class='entry match transform'>");
                        match k {
                            Rationale::Type(_) => {
                                html.push_str(
                                    format!(
                                        "<b><code>{}</code> produced a value</b>\n",
                                        description
                                    )
                                    .as_str(),
                                );
                                Self::rationale_inner(html, (*transform).borrow());
                                if let Some(list) = transform.try_get_list() {
                                    for item in list {
                                        Self::rationale_inner(html, (**item).borrow());
                                    }
                                }
                                if let Some(object) = transform.try_get_object() {
                                    for (field_name, v) in object.iter() {
                                        html.push_str("<div>");
                                        html.push_str(
                                            format!("<b>field <code>{}</code></b>", field_name)
                                                .as_str(),
                                        );
                                        Self::rationale_inner(html, (**v).borrow());
                                        html.push_str("</div>");
                                    }
                                }
                            }
                            Rationale::Field(_) => {
                                html.push_str(
                                    format!("<b>field <code>{}</code> matched</b>\n", description)
                                        .as_str(),
                                );
                            }
                            Rationale::Expr(_) => {}
                        }
                        html.push_str("</div>")
                    }
                }
            }
        }

        html.push_str("</div>");
    }
}