// Code to make GSUB tables easier to work with
use skrifa::{
    raw::{
        tables::{
            gsub::{
                AlternateSubst, AlternateSubstFormat1, AlternateSubstFormat2,
                ExtensionSubstFormat1, LigatureSubst, LigatureSubstFormat1, LigatureSubstFormat2,
                MultipleSubst, MultipleSubstFormat1, MultipleSubstFormat2, ReverseChainSingleSubst,
                ReverseChainSingleSubstFormat1, ReverseChainSingleSubstFormat2, SingleSubst,
                SingleSubstFormat1, SingleSubstFormat2, SingleSubstFormat3, SingleSubstFormat4,
                SubstitutionSubtables,
            },
            layout::Subtables,
        },
        ReadError,
    },
    GlyphId,
};

/// A map of substitutions, input glyphs on the left, output glyphs on the right
pub type SubstitutionMap = Vec<(Vec<GlyphId>, Vec<GlyphId>)>;

/// A trait to get a substitution map from a GSUB subtable
pub trait GetSubstitutionMap {
    /// Get the substitution map from the subtable
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError>;
}

/// This type isn't public in `read-fonts` so copy it in
type SubSubtables<'a, T> = Subtables<'a, T, ExtensionSubstFormat1<'a, T>>;

impl<'a> GetSubstitutionMap for SingleSubstFormat1<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let delta = self.delta_glyph_id() as i32;
        Ok(self
            .coverage()?
            .iter()
            .map(|old_gid| {
                let new_gid = GlyphId::from((old_gid.to_u32() as i32 + delta) as u16);
                (vec![old_gid], vec![new_gid])
            })
            .collect())
    }
}

impl<'a> GetSubstitutionMap for SingleSubstFormat2<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        Ok(self
            .coverage()?
            .iter()
            .zip(self.substitute_glyph_ids().iter().map(|gid| gid.get()))
            .map(|(old_gid, new_gid)| (vec![old_gid], vec![GlyphId::from(new_gid)]))
            .collect())
    }
}

impl<'a> GetSubstitutionMap for SingleSubstFormat3<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let delta = self.delta_glyph_id().to_i32();
        Ok(self
            .coverage()?
            .iter()
            .map(|old_gid| {
                let new_gid = GlyphId::from((old_gid.to_u32() as i32 + delta) as u16);
                (vec![old_gid], vec![new_gid])
            })
            .collect())
    }
}

impl<'a> GetSubstitutionMap for SingleSubstFormat4<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        Ok(self
            .coverage()?
            .iter()
            .zip(self.substitute_glyph_ids().iter().map(|gid| gid.get()))
            .map(|(old_gid, new_gid)| (vec![old_gid], vec![GlyphId::from(new_gid)]))
            .collect())
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, SingleSubst<'a>> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                SingleSubst::Format1(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
                SingleSubst::Format2(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
                SingleSubst::Format3(single_subst_format3) => {
                    result.extend(single_subst_format3.substitutions()?);
                }
                SingleSubst::Format4(single_subst_format4) => {
                    result.extend(single_subst_format4.substitutions()?);
                }
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, MultipleSubst<'a>> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                MultipleSubst::Format1(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
                MultipleSubst::Format2(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for MultipleSubstFormat1<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        Ok(self
            .coverage()?
            .iter()
            .zip(self.sequences().iter().flatten())
            .map(|(old_gid, sequence)| {
                let new_gids = sequence
                    .substitute_glyph_ids()
                    .iter()
                    .map(|gid| GlyphId::from(gid.get()))
                    .collect();
                (vec![old_gid], new_gids)
            })
            .collect())
    }
}

impl<'a> GetSubstitutionMap for MultipleSubstFormat2<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        Ok(self
            .coverage()?
            .iter()
            .zip(self.sequences().iter().flatten())
            .map(|(old_gid, sequence)| {
                let new_gids = sequence
                    .substitute_glyph_ids()
                    .iter()
                    .map(|gid| GlyphId::from(gid.get()))
                    .collect();
                (vec![old_gid], new_gids)
            })
            .collect())
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, AlternateSubst<'a>> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                AlternateSubst::Format1(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
                AlternateSubst::Format2(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for AlternateSubstFormat1<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        Ok(self
            .coverage()?
            .iter()
            .zip(self.alternate_sets().iter().flatten())
            .map(|(old_gid, sequence)| {
                let new_gids = sequence
                    .alternate_glyph_ids()
                    .iter()
                    .map(|gid| GlyphId::from(gid.get()))
                    .collect();
                (vec![old_gid], new_gids)
            })
            .collect::<Vec<_>>())
    }
}

impl<'a> GetSubstitutionMap for AlternateSubstFormat2<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        Ok(self
            .coverage()?
            .iter()
            .zip(self.alternate_sets().iter().flatten())
            .map(|(old_gid, sequence)| {
                let new_gids = sequence
                    .alternate_glyph_ids()
                    .iter()
                    .map(|gid| GlyphId::from(gid.get()))
                    .collect();
                (vec![old_gid], new_gids)
            })
            .collect::<Vec<_>>())
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, LigatureSubst<'a>> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                LigatureSubst::Format1(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
                LigatureSubst::Format2(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for LigatureSubstFormat1<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let mut result = vec![];
        for (first_gid, sequence) in self
            .coverage()?
            .iter()
            .zip(self.ligature_sets().iter().flatten())
        {
            for ligature in sequence.ligatures().iter().flatten() {
                let remaining_gids = ligature
                    .component_glyph_ids()
                    .iter()
                    .map(|gid| GlyphId::from(gid.get()));
                let new_gid = GlyphId::from(ligature.ligature_glyph());
                let mut lhs = vec![first_gid];
                lhs.extend(remaining_gids);
                result.push((lhs, vec![new_gid]));
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for LigatureSubstFormat2<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let mut result = vec![];
        for (first_gid, sequence) in self
            .coverage()?
            .iter()
            .zip(self.ligature_sets().iter().flatten())
        {
            for ligature in sequence.ligatures().iter().flatten() {
                let remaining_gids = ligature
                    .component_glyph_ids()
                    .iter()
                    .map(|gid| GlyphId::from(gid.get()));
                let new_gid = GlyphId::from(ligature.ligature_glyph());
                let mut lhs = vec![first_gid];
                lhs.extend(remaining_gids);
                result.push((lhs, vec![new_gid]));
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, ReverseChainSingleSubst<'a>> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                ReverseChainSingleSubst::Format1(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
                ReverseChainSingleSubst::Format2(table_ref) => {
                    result.extend(table_ref.substitutions()?);
                }
            }
        }
        Ok(result)
    }
}
impl<'a> GetSubstitutionMap for ReverseChainSingleSubstFormat1<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        Ok(self
            .coverage()?
            .iter()
            .zip(self.substitute_glyph_ids().iter())
            .map(|(lgid, rgid)| (vec![lgid], vec![GlyphId::from(rgid.get())]))
            .collect::<Vec<_>>())
    }
}

impl<'a> GetSubstitutionMap for ReverseChainSingleSubstFormat2<'a> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        Ok(self
            .coverage()?
            .iter()
            .zip(self.substitute_glyph_ids().iter())
            .map(|(lgid, rgid)| (vec![lgid], vec![GlyphId::from(rgid.get())]))
            .collect::<Vec<_>>())
    }
}
impl GetSubstitutionMap for SubstitutionSubtables<'_> {
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError> {
        match self {
            SubstitutionSubtables::Single(st) => st.substitutions(),
            SubstitutionSubtables::Multiple(st) => st.substitutions(),
            SubstitutionSubtables::Alternate(st) => st.substitutions(),
            SubstitutionSubtables::Ligature(st) => st.substitutions(),
            SubstitutionSubtables::Reverse(st) => st.substitutions(),
            SubstitutionSubtables::Contextual(_)
            | SubstitutionSubtables::ChainContextual(_)
            | SubstitutionSubtables::EmptyExtension => Ok(vec![]),
        }
    }
}
