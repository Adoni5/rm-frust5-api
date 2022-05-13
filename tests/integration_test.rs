use std::collections::HashMap;

use frust5_api::{self, ChannelInfo, RawAttrsOpts};

#[test]
fn write_test_fast5() {
    let context_tags = HashMap::from([
        ("barcoding_enabled", "0"),
        ("experiment_duration_set", "4800"),
        ("experiment_type", "genomic_dna"),
        ("local_basecalling", "0"),
        ("package", "bream4"),
        ("package_version", "6.3.5"),
        ("sample_frequency", "4000"),
        ("sequencing_kit", "sqk-lsk109"),
    ]);
    let tracking_id = HashMap::from([
        ("asic_id", "817405089"),
        ("asic_id_eeprom", "5661715"),
        ("asic_temp", "29.357218"),
        ("asic_version", "IA02D"),
        ("auto_update", "0"),
        (
            "auto_update_source",
            "https,//mirror.oxfordnanoportal.com/software/MinKNOW/",
        ),
        ("bream_is_standard", "0"),
        ("configuration_version", "4.4.13"),
        ("device_id", "Bantersaurus"),
        ("device_type", "gridion"),
        ("distribution_status", "stable"),
        ("distribution_version", "21.10.8"),
        (
            "exp_script_name",
            "sequencing/sequencing_MIN106_DNA,FLO-MIN106,SQK-LSK109",
        ),
        ("exp_script_purpose", "sequencing_run"),
        ("exp_start_time", "2021-12-17T16,54,04.325472+00,00"),
        ("flow_cell_id", "TEST0001"),
        ("flow_cell_product_code", "FLO-MIN106"),
        ("guppy_version", "5.0.17+99baa5b"),
        ("heatsink_temp", "34.066406"),
        ("host_product_code", "GRD-X5B003"),
        ("host_product_serial_number", "NOTFOUND"),
        ("hostname", "master"),
        ("installation_type", "nc"),
        ("local_firmware_file", "1"),
        ("operating_system", "ubuntu 16.04"),
        ("protocol_group_id", "test_exp"),
        ("protocol_run_id", "SYNTHETIC_RUN"),
        ("protocol_start_time", "2021-12-17T16,49,35.705717+00,00"),
        ("protocols_version", "6.3.5"),
        ("run_id", "Runnin"),
        ("sample_id", "fetid_lake_water_coffee"),
        ("usb_config", "fx3_1.2.4#fpga_1.2.1#bulk#USB300"),
        ("version", "4.4.3"),
    ]);
    let raw_attrs = HashMap::from([
        ("duration", RawAttrsOpts::Duration(100)),
        ("end_reason", RawAttrsOpts::EndReason(1)),
        ("median_before", RawAttrsOpts::MedianBefore(100.0)),
        ("read_id", RawAttrsOpts::ReadId("Hi there")),
        ("read_number", RawAttrsOpts::EndReason(100)),
        ("start_mux", RawAttrsOpts::StartMux(10)),
        ("start_time", RawAttrsOpts::StartTime(1000000)),
    ]);

    let channel_info = ChannelInfo::new(8192_f64, 6.0, 1500.0, 4000.0, String::from("241"));
    let mut multi =
        frust5_api::MultiFast5File::new("test2.fast5".to_string(), frust5_api::OpenMode::Append);
    multi
        .create_empty_read(
            "get_your_tsaty_read_right_here".to_string(),
            "5e83140edbb3559206940d49ecd665888d7da5f9".to_string(),
            &tracking_id,
            &context_tags,
            channel_info,
            &raw_attrs
        )
        .unwrap();
    let channel_info = ChannelInfo::new(8192_f64, 6.0, 1500.0, 4000.0, String::from("241"));

    multi
        .create_empty_read(
            "get_your_tsaty_read_right_here2".to_string(),
            "5e83140edbb3559206940d49ecd665888d7da5f9".to_string(),
            &tracking_id,
            &context_tags,
            channel_info,
            &raw_attrs
        )
        .unwrap();
}

#[test]
fn test_iter() {
    let x = frust5_api::ChannelInfo::new(1.0, 2.0, 3.0, 4.0, String::from("Hi"));
    for (k, v) in x {
        println!("{}/{}", k, v)
    }
}
