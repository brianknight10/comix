use crate::BuildArgs;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::vec;

use anyhow::Result;
use csv::Reader;
use geojson::{feature, Feature, FeatureCollection, Geometry, JsonObject, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RoutePoint {
    #[serde(rename = "AMENDMENT_NO")]
    amendment_number: String,
    #[serde(rename = "LAT_DECIMAL")]
    latitude: f64,
    #[serde(rename = "LONG_DECIMAL")]
    longitude: f64,
    #[serde(rename = "POINT_SEQ")]
    point_sequence: i16,
    #[serde(rename = "PROCEDURE_NAME")]
    procedure_name: String,
    #[serde(rename = "RNAV_FLAG")]
    rnav_flag: String,
}

pub fn write_lines(args: BuildArgs) -> Result<()> {
    let mut points: Vec<RoutePoint> = vec![];
    let mut reader = Reader::from_path(args.input)?;

    for result in reader.deserialize() {
        let point: RoutePoint = result?;
        points.push(point);
    }

    let output = File::create(args.output)?;
    let mut writer = BufWriter::new(output);

    let mut p = points.iter().peekable();
    let mut features: Vec<Feature> = vec![];
    let mut lines: Option<Vec<Vec<Vec<f64>>>> = None;
    let mut verts: Option<Vec<Vec<f64>>> = None;

    while let Some(point) = p.next() {
        let next_point = p.peek();

        match verts {
            None => {
                verts = Some(vec![vec![point.longitude, point.latitude]]);
            }
            Some(ref mut v) => {
                v.push(vec![point.longitude, point.latitude]);
            }
        }

        if next_point.is_none() || next_point.unwrap().point_sequence == 10 {
            match lines {
                None => {
                    lines = Some(vec![verts.unwrap()]);
                }
                Some(ref mut l) => {
                    l.push(verts.unwrap());
                }
            }
            verts = None;

            if next_point.is_some() && next_point.unwrap().procedure_name == point.procedure_name {
                continue;
            }

            features.push(Feature {
                bbox: None,
                geometry: Some(Geometry::new(Value::MultiLineString(lines.unwrap()))),
                id: Some(feature::Id::String(point.procedure_name.clone())),
                properties: Some({
                    let mut properties = JsonObject::new();
                    properties.insert(
                        "name".to_string(),
                        procedure_name(&point.procedure_name, &point.amendment_number).into(),
                    );
                    properties.insert("rnav".to_string(), point.rnav_flag.clone().into());
                    properties
                }),
                foreign_members: None,
            });
            lines = None;
        }
    }

    let geojson = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };

    writer.write_all(geojson.to_string().as_bytes())?;

    Ok(())
}

fn procedure_name(name: &str, number: &str) -> String {
    format!("{} {}", name, number)
}
