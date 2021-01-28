use crate::model::{TfData, TfLayerEnum};
use nom::branch::alt;
use nom::combinator::map;

use nom::multi::many1;

use super::{
    base_parser::tf_comment, color_parser::*, contact_parser::*, layer_dt_parser::*,
    layer_parser::*, rule_parser::*, stipple_parser::*, technology_parser::*, tile_parser::*,
};

use nom::sequence::tuple;
use nom::IResult;
pub fn tf_parser(input: &str) -> IResult<&str, TfData> {
    let (input, data) = tuple((
        many1(tf_comment),
        technology_parser,
        many1(color_parser),
        many1(stipple_parser),
        tile_parser,
        many1(alt((
            map(layer_parser, |x| TfLayerEnum::Layer(x)),
            map(layer_dt_parser, |x| TfLayerEnum::DataType(x)),
        ))),
        many1(contact_parser),
        many1(designrule_parser),
        pr_rule_parser,
        many1(density_rule_parser),
    ))(input)?;
    Ok((
        input,
        TfData {
            basic: data.1,
            color: data.2,
            stipple: data.3,
            tile: data.4,
            layer: data.5,
            contact: data.6,
            designrule: data.7,
            prrule: data.8,
            densityrule: data.9,
        },
    ))
}