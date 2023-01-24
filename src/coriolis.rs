use std::collections::HashMap;

use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Default, Debug, PartialEq)]
pub struct Coriol {
    pub data: HashMap<(usize, usize), Vec<usize>>,
}

/// simpler representation for almost trivially implementing Serialize and
/// Deserialize
#[derive(Deserialize)]
struct DummyCoriol {
    modes: Vec<(usize, usize)>,
    axes: Vec<Vec<usize>>,
}

impl From<DummyCoriol> for Coriol {
    fn from(value: DummyCoriol) -> Self {
        Self {
            data: value.modes.into_iter().zip(value.axes).collect(),
        }
    }
}

impl<'de> Deserialize<'de> for Coriol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = DummyCoriol::deserialize(deserializer)?;
        Ok(s.into())
    }
}

impl Serialize for Coriol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Coriol", 2)?;
        let mut modes = Vec::new();
        let mut axes = Vec::new();
        let mut keys: Vec<_> = self.data.keys().collect();
        keys.sort();
        for key in keys {
            modes.push(key);
            axes.push(self.data[key].clone());
        }
        s.serialize_field("modes", &modes)?;
        s.serialize_field("axes", &axes)?;
        s.end()
    }
}
