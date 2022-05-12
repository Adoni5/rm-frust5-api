use std::collections::HashMap;

use hdf5;
use hdf5::types::VarLenAscii;

use crate::ChannelInfo;

fn _set_string_attributes(
    group: hdf5::Group,
    data: &HashMap<&str, &str>,
) -> Result<(), hdf5::Error> {
    for (key, value) in data.into_iter() {
        let attr = VarLenAscii::from_ascii(value).unwrap();
        group
            .new_attr::<VarLenAscii>()
            .create(*key)?
            .write_scalar(&attr)?;
    }
    Ok(())
}

fn _set_float_attributes(group: hdf5::Group, data: ChannelInfo) -> Result<(), hdf5::Error> {
    for (key, value) in data {
        group
            .new_attr::<f64>()
            .create(key.as_str())?
            .write_scalar(&value)?;
    }
    Ok(())
}

pub fn add_tracking_info(
    group: hdf5::Group,
    data: &HashMap<&str, &str>,
) -> Result<(), hdf5::Error> {
    _set_string_attributes(group, data)?;
    Ok(())
}

pub fn add_context_tags(group: hdf5::Group, data: &HashMap<&str, &str>) -> Result<(), hdf5::Error> {
    _set_string_attributes(group, data)?;
    Ok(())
}

pub fn add_channel_info(group: hdf5::Group, data: ChannelInfo) -> Result<(), hdf5::Error> {
    let attr = VarLenAscii::from_ascii(&data.channel_number).unwrap();
    group
        .new_attr::<VarLenAscii>()
        .create("channel_number")?
        .write_scalar(&attr)?;
    _set_float_attributes(group, data)?;
    Ok(())
}

pub fn add_raw_data(group: hdf5::Group, data: ChannelInfo) -> Result<(), hdf5::Error> {
    
    Ok(())
}
