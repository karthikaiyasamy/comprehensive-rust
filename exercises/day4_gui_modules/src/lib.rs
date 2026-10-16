pub mod gui {
    pub trait Widget {
        fn title(&self) -> &str;
    }

    pub mod widgets {
        use super::Widget;

        pub struct Button {
            pub label: String,
        }

        impl Widget for Button {
            fn title(&self) -> &str {
                &self.label
            }
        }
    }

    pub struct Window {
        pub widgets: Vec<Box<dyn Widget>>,
    }

    impl Window {
        pub fn new() -> Self {
            Window { widgets: Vec::new() }
        }

        pub fn add_widget(&mut self, w: Box<dyn Widget>) {
            self.widgets.push(w);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::gui::widgets::Button;
    use super::gui::Window;

    #[test]
    fn test_gui() {
        let mut win = Window::new();
        win.add_widget(Box::new(Button { label: "Submit".to_string() }));
        assert_eq!(win.widgets.len(), 1);
        assert_eq!(win.widgets[0].title(), "Submit");
    }
}
