mod utils;

#[cfg(feature = "blosc")]
use hdf5::filters::blosc_set_nthreads;
use hdf5::types::VarLenAscii;
use hdf5::{Error, File, Group, Result};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn read_fast5() -> Result<Vec<Group>, hdf5::Error> {
    let file = File::open("FAL37440_pass_5e83140e_100.fast5")?; // open for reading
    let gs = file.groups()?; // open the dataset;
    Ok(gs.clone())
}

pub struct MultiFast5File {
    filename: String,
    mode: OpenMode,
    handle: File,
    _run_id_map: HashMap<String, String>,
}

pub struct ChannelInfo {
    digitisation: f64,
    offset: f64,
    range: f64,
    sampling_rate: f64,
    channel_number: String,
}
impl IntoIterator for ChannelInfo {
    type Item = (String, f64);
    type IntoIter = std::array::IntoIter<(String, f64), 4>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([
            ("digitisation".to_string(), self.digitisation),
            ("offset".to_string(), self.offset),
            ("range".to_string(), self.range),
            ("sampling_rate".to_string(), self.sampling_rate),
        ])
    }
}
impl ChannelInfo {
    pub fn new(
        digitisation: f64,
        offset: f64,
        range: f64,
        sampling_rate: f64,
        channel_number: String,
    ) -> ChannelInfo {
        ChannelInfo {
            digitisation,
            offset,
            range,
            sampling_rate,
            channel_number,
        }
    }
}

pub enum OpenMode {
    Append,
    Read,
}
const HARDLINK_GROUPS: [&str; 2] = ["context_tags", "tracking_id"];

impl MultiFast5File {
    pub fn new(filename: String, mode: OpenMode) -> MultiFast5File {
        let file = match mode {
            OpenMode::Append => {
                let file = File::with_options()
                    .with_fapl(|p| p.core().core_filebacked(true))
                    .append(&filename)
                    .unwrap();
                // default attributes for now
                let file_type = VarLenAscii::from_ascii("multi-read").unwrap();
                let file_version = VarLenAscii::from_ascii("2.2").unwrap();
                file.new_attr::<VarLenAscii>()
                    .create("file_type")
                    .unwrap()
                    .write_scalar(&file_type)
                    .unwrap();
                file.new_attr::<VarLenAscii>()
                    .create("file_version")
                    .unwrap()
                    .write_scalar(&file_version)
                    .unwrap();
                file
            }
            OpenMode::Read => {
                File::open("FAL37440_pass_5e83140e_100.fast5").unwrap() // open for reading
            }
        };
        MultiFast5File {
            filename: filename.clone(),
            mode,
            handle: file,
            _run_id_map: HashMap::new(),
        }
    }
    /// Diverged from ONT come back and straight up rework
    pub fn create_empty_read(
        &mut self,
        read_id: String,
        run_id: String,
        tracking_id: &HashMap<&str, &str>,
        context_tags: &HashMap<&str, &str>,
        channel_info: ChannelInfo,
    ) -> Result<Group, Error> {
        let group_name = format!("read_{}", read_id);
        let group = self.handle.create_group(&group_name).unwrap();
        let s = VarLenAscii::from_ascii(run_id.as_str()).unwrap();
        group
            .new_attr::<VarLenAscii>()
            .create("run_id")?
            .write_scalar(&s)?;
        // set the shared groups for every read - namely the contstant Dict attributes
        if self._run_id_map.contains_key(&run_id) {
            for shared_group in HARDLINK_GROUPS {
                self.handle
                    .link_hard(
                        format!("read_{}/{}", self._run_id_map[&run_id], shared_group).as_str(),
                        format!("{}/{}", group_name, shared_group).as_str(),
                    )
                    .expect(format!("{}/{}", self._run_id_map[&run_id], shared_group).as_str());
            }
        } else {
            self._run_id_map.insert(run_id, read_id);
            let context_group = group.create_group("context_tags")?;
            let tracking_group = group.create_group("tracking_id")?;
            let channel_group = group.create_group("channel_id")?;
            utils::add_tracking_info(tracking_group, &tracking_id)?;
            utils::add_context_tags(context_group, context_tags)?;
            utils::add_channel_info(channel_group, channel_info)?;
        }

        Ok(group)
    }

}
