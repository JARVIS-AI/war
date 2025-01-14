use crate::{darksiders1::gfc, utils::parsing::derailed};
use byteordered::{ByteOrdered, Endianness};
use failure::Error;
use std::{
    io::{Read, Seek},
    rc::Rc,
};

pub use visscript::draw_all_visscripts_as_graphviz;

mod visscript;

pub fn read(file: impl Read + Seek) -> Result<Vec<(String, gfc::Object)>, Error> {
    let mut files = Vec::new();

    let mut world_package = ByteOrdered::new(file, Endianness::Big);
    let world_files = gfc::WorldFactory::read_header(&mut world_package)?;

    for world_info in &world_files {
        let world_path = world_info.name.clone();
        let world = gfc::WorldFactory::read_world(
            &mut world_package,
            &world_files,
            &world_info.name,
        )?;
        let world = Rc::new(world);
        files.push((world_path, world.clone()));

        let region_data = match world.get_property("RegionData") {
            Some(x) => x.as_array().ok_or_else(derailed)?,
            None => &[],
        };
        for region_data in region_data {
            let region_data = region_data.as_object().ok_or_else(derailed)?;
            let region_id = region_data
                .get_property("ID")
                .and_then(gfc::Value::as_int)
                .ok_or_else(derailed)?;
            let region_name = region_data
                .get_property("Name")
                .and_then(gfc::Value::as_hstring)
                .ok_or_else(derailed)?;
            let region_layers = region_data
                .get_property("Layers")
                .and_then(gfc::Value::as_array)
                .ok_or_else(derailed)?;

            let region_path = format!("{}/{}", world_info.name, region_name);
            let region = gfc::WorldFactory::read_region(
                &mut world_package,
                &world_files,
                &world,
                region_id,
            )?;
            let region = Rc::new(region);
            files.push((region_path, region.clone()));

            for layer_data in region_layers {
                let layer_data = layer_data.as_object().ok_or_else(derailed)?;
                let layer_id = layer_data
                    .get_property("ID")
                    .and_then(gfc::Value::as_int)
                    .ok_or_else(derailed)?;
                let layer_name = layer_data
                    .get_property("Name")
                    .and_then(gfc::Value::as_hstring)
                    .ok_or_else(derailed)?;

                let layer_path =
                    format!("{}/{}/{}", world_info.name, region_name, layer_name);
                let layer = gfc::WorldFactory::read_layer(
                    &mut world_package,
                    &world_files,
                    &world,
                    region_data,
                    layer_id,
                )?;
                files.push((layer_path, Rc::new(layer)));
            }
        }
    }

    Ok(files
        .into_iter()
        // Unwrap the `Rc`s which we used to appease the borrow checker.
        .map(|(p, o)| (p, Rc::try_unwrap(o).unwrap()))
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::{darksiders1::gfc, utils::parsing::derailed, worlds};
    use failure::Error;
    use std::{
        fs,
        io::{self, Read, Seek},
    };

    #[test]
    fn smoke_test() -> Result<(), Error> {
        let worlds = open_fixture()?;
        let files = worlds::read(worlds)?;
        let (world_name, world) = &files[0];
        assert_eq!(world_name, "overworld");
        assert_eq!(
            world
                .get_property("RegionData")
                .and_then(gfc::Value::as_array)
                .ok_or_else(derailed)?
                .len(),
            227,
        );
        Ok(())
    }

    fn open_fixture() -> io::Result<impl Read + Seek> {
        let root = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/src/worlds/fixtures/worlds.mnfst", root);
        let file = fs::File::open(&path)?;
        Ok(io::BufReader::new(file))
    }
}
