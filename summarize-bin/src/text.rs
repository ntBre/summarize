use summarize::Summary;

use crate::{format::Format, impl_display};

pub(crate) struct Text(pub Vec<Summary>);

impl_display!(Text);

impl Format for Text {
    max_fields! {
        harm => max_harms,
        corr => max_corrs,
        rots => max_rots,
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> IntoIterator for &'a Text {
    type Item = &'a Summary;

    type IntoIter = std::slice::Iter<'a, Summary>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
