pub mod firestore {
    pub mod v1 {
        tonic::include_proto!("google.firestore.v1");
    }
}
pub mod r#type {
    tonic::include_proto!("google.r#type");
}
pub mod rpc {
    tonic::include_proto!("google.rpc");
}
