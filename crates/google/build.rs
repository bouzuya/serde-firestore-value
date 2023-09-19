use prost_build::Config;

fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.disable_comments([
        "google.firestore.v1.BloomFilter",
        "google.firestore.v1.StructuredQuery.start_at",
        "google.firestore.v1.StructuredAggregationQuery.Aggregation.alias",
        "google.firestore.v1.StructuredAggregationQuery.Aggregation.Count.up_to",
    ]);
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .disable_comments("google.firestore.v1.Firestore")
        .disable_comments("google.firestore.v1.Firestore.RunAggregationQuery")
        .compile_with_config(
            config,
            &["proto/googleapis/google/firestore/v1/firestore.proto"],
            &["proto/googleapis"],
        )?;
    Ok(())
}
