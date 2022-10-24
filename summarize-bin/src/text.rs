use summarize::Summary;

use crate::{
    format::{Format, TableType},
    impl_display,
};

pub(crate) struct Text(pub Vec<Summary>);

impl_display!(Text);

impl Format for Text {
    const SEP: &'static str = "";
    const END: &'static str = "";

    fn len(&self) -> usize {
        self.0.len()
    }

    fn line(width: usize) -> String {
        (vec!["-"; width]).join("")
    }

    max_fields! {
        harm => max_harms,
        corr => max_corrs,
        rots => max_rots,
    }

    fn omega(&self, idx: usize) -> String {
        format!(" w{:<2}", idx)
    }

    fn nu(&self, idx: usize) -> String {
        format!(" v{:<2}", idx)
    }

    fn pre_table(&self, typ: TableType, _: usize) -> String {
        match typ {
            TableType::Vib => String::from("Vibrational Frequencies (cm-1):\n"),
        }
    }

    fn post_table(&self) -> &'static str {
        ""
    }
}

impl<'a> IntoIterator for &'a Text {
    type Item = &'a Summary;

    type IntoIter = std::slice::Iter<'a, Summary>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
