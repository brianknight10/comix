use crate::BuildArgs;

use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Result;
use csv::Reader;
use geojson::{Feature, GeoJson, Geometry, JsonObject, Value};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Point {
    #[serde(rename = "POINT")]
    id: String,
    #[serde(rename = "LAT_DECIMAL")]
    latitude: f64,
    #[serde(rename = "LONG_DECIMAL")]
    longitude: f64,
    #[serde(rename = "POINT_TYPE")]
    kind: String,
}

pub fn write_points(args: BuildArgs) -> Result<()> {
    let mut points: Vec<Point> = vec![];
    let mut reader = Reader::from_path(args.input)?;

    for result in reader.deserialize() {
        let point: Point = result?;
        points.push(point);
    }

    let output = File::create(args.output)?;
    let mut writer = BufWriter::new(output);

    let geojson = GeoJson::FeatureCollection(
        points
            .iter()
            .map(|point| {
                let geometry = Geometry::new(Value::Point(vec![
                    point.longitude,
                    point.latitude,
                ]));
                let properties = {
                    let mut properties = JsonObject::new();
                    properties.insert("id".to_string(), point.id.clone().into());
                    properties.insert("kind".to_string(), point.kind.clone().into());
                    properties
                };
                Feature {
                    bbox: None,
                    geometry: Some(geometry),
                    id: None,
                    properties: Some(properties),
                    foreign_members: None,
                }
            })
            .collect(),
    );

    writer.write_all(geojson.to_string().as_bytes())?;

    Ok(())
}
