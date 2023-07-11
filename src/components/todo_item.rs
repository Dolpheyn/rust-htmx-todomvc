use leptos::*;

struct Style<'a> {
    rules: Vec<bool>,
    styles: Vec<&'a str>,
}

impl<'a> Style<'a> {
    fn new() -> Self {
        Self {
            rules: Vec::new(),
            styles: Vec::new(),
        }
    }

    fn with_rule(mut self, rule: bool, style: &'a str) -> Self {
        self.rules.push(rule);
        self.styles.push(style);

        self
    }

    fn build(self) -> String {
        self.rules
            .iter()
            .zip(self.styles)
            .map(|(rule, style)| {
                if *rule {
                    style.to_string()
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[component]
pub fn TodoItem(cx: Scope, id: usize, text: String, completed: bool) -> impl IntoView {
    let put_url = format!("/todos/{id}", id = { id });
    let style = Style::new()
        .with_rule(completed, "text-decoration: line-through")
        .with_rule(id % 2 == 0, "background-color: darkgrey")
        .build();
    let element_id = format!("todo-item-{id}", id = { id });

    view! { cx,
        <li id=element_id hx-put=put_url style=style hx-swap="outerHTML">
            {text}
        </li>
    }
}
