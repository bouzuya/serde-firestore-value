use std::collections::HashMap;

#[test]
fn test() {
    let _ = google::firestore::v1::Document {
        name: "name".to_string(),
        fields: HashMap::default(),
        create_time: None,
        update_time: None,
    };
    let _ = google::r#type::LatLng {
        latitude: 0_f64,
        longitude: 0_f64,
    };
    let _ = google::rpc::Status {
        code: 0_i32,
        message: "message".to_string(),
        details: Vec::default(),
    };
}
