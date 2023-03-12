use libheif_rs::LibHeif;

#[test]
fn get_version() {
    let lib_heif = LibHeif::new();
    let version = lib_heif.version();
    assert!(version[0] >= 1);
    assert!(version[1] >= 14);
}

#[test]
fn get_encoder_descriptors() {
    let lib_heif = LibHeif::new();
    let descriptors = lib_heif.encoder_descriptors(100, None, None);
    assert!(descriptors.len() >= 2);
}

#[test]
fn get_encoder() {
    let lib_heif = LibHeif::new();
    let descriptors = lib_heif.encoder_descriptors(1, None, None);
    assert!(!descriptors.is_empty());
    let encoder = lib_heif.encoder(descriptors[0]).unwrap();
    assert_eq!(encoder.name(), descriptors[0].name());
}

#[test]
fn get_encoder_for_format() {
    let lib_heif = LibHeif::new();
    let descriptors = lib_heif.encoder_descriptors(100, None, None);
    for descriptor in descriptors {
        let encoder = lib_heif
            .encoder_for_format(descriptor.compression_format())
            .unwrap();
        assert_eq!(encoder.name(), descriptor.name());
    }
}
