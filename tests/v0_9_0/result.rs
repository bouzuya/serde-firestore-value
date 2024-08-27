#[test]
fn test() {
    use serde::de::Error as _;
    use serde_firestore_value::{Error, Result};
    #[derive(serde::Serialize)]
    struct T;
    let _ = Result::<T>::Ok(T);
    let _ = Result::<T>::Err(Error::custom("test"));
}
