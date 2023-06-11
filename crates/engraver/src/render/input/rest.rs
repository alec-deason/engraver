use smufl::{Metadata, StaffSpaces};

use super::Duration;
use crate::{
    render::{
        context::Context,
        ir::{Coord, Element, Symbol},
        metadata_extensions::MetadataExtensions,
        Output, Render,
    },
    Result,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Rest {
    pub duration: Duration,
    pub id: Option<String>,
}

impl Render for Rest {
    fn render(
        &self,
        x: StaffSpaces,
        _context: &mut Context,
        metadata: &Metadata,
    ) -> Result<Output> {
        let glyph = self.duration.value.rest_glyph();

        let element = Element::Symbol(Symbol {
            origin: Coord {
                x,
                y: StaffSpaces(2.0),
            },
            value: glyph.codepoint(),
        });
        let width = metadata.width_of(glyph)?;

        Ok(Output {
            elements: vec![Element::Group {
                id: self.id.clone(),
                elements: vec![element],
            }],
            width,
        })
    }
}
