use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

mod imp {
    use std::cell::Cell;

    use super::*;
    use glib::{subclass::InitializingObject, Properties};
    use gtk::prelude::*;

    #[derive(Debug, Default, CompositeTemplate, Properties)]
    #[template(resource = "/moe/tsukimi/action_row.ui")]
    #[properties(wrapper_type = super::AActionRow)]
    pub struct AActionRow {
        #[template_child]
        pub secondary_label: TemplateChild<gtk::Label>,
        #[property(get, set, default_value = true)]
        pub show_arrow: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AActionRow {
        const NAME: &'static str = "AActionRow";
        type Type = super::AActionRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for AActionRow {}

    impl WidgetImpl for AActionRow {}
    impl ListBoxRowImpl for AActionRow {}
    impl PreferencesRowImpl for AActionRow {}
    impl ActionRowImpl for AActionRow {}
}

glib::wrapper! {
    pub struct AActionRow(ObjectSubclass<imp::AActionRow>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::ActionRow, @implements gtk::Accessible;
}

impl AActionRow {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
