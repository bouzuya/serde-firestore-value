#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_serde_json_bool() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&true)?, "true");
        Ok(())
    }

    #[test]
    fn test_serde_json_u8() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&0_u8)?, "0");
        Ok(())
    }

    #[test]
    fn test_serde_json_u64() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&u64::MAX)?, "18446744073709551615");
        Ok(())
    }

    #[test]
    fn test_serde_json_char() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&'a')?, r#""a""#);
        Ok(())
    }

    #[test]
    fn test_serde_json_none() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&None::<u8>)?, "null");
        Ok(())
    }

    #[test]
    fn test_serde_json_some() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&Some(1_u8))?, "1");
        Ok(())
    }

    #[test]
    fn test_serde_json_unit() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&())?, "null");
        Ok(())
    }

    #[test]
    fn test_serde_json_unit_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Unit;
        assert_eq!(serde_json::to_string(&Unit)?, "null");
        Ok(())
    }

    #[test]
    fn test_serde_json_unit_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            A,
            B,
        }
        assert_eq!(serde_json::to_string(&E::A)?, r#""A""#);
        assert_eq!(serde_json::to_string(&E::B)?, r#""B""#);
        Ok(())
    }

    #[test]
    fn test_serde_json_newtype_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Millimeters(u8);
        assert_eq!(serde_json::to_string(&Millimeters(1))?, r#"1"#);
        Ok(())
    }

    #[test]
    fn test_serde_json_newtype_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            N(u8),
        }
        assert_eq!(serde_json::to_string(&E::N(1))?, r#"{"N":1}"#);
        Ok(())
    }

    #[test]
    fn test_serde_json_seq() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&vec![
                serde_json::Value::Bool(true),
                serde_json::Value::String('c'.to_string())
            ])?,
            r#"[true,"c"]"#
        );
        Ok(())
    }

    #[test]
    fn test_serde_json_tuple_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct Rgb(u8, u8, u8);
        assert_eq!(serde_json::to_string(&Rgb(1, 2, 3))?, r#"[1,2,3]"#);
        Ok(())
    }

    #[test]
    fn test_serde_json_tuple_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            T(u8, u8),
        }
        assert_eq!(serde_json::to_string(&E::T(1, 2))?, r#"{"T":[1,2]}"#);
        Ok(())
    }

    #[test]
    fn test_serde_json_struct() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        struct S {
            r: u8,
            g: u8,
            b: u8,
        }
        assert_eq!(
            serde_json::to_string(&S { r: 1, g: 2, b: 3 })?,
            r#"{"r":1,"g":2,"b":3}"#
        );
        Ok(())
    }

    #[test]
    fn test_serde_json_struct_variant() -> anyhow::Result<()> {
        #[derive(serde::Serialize)]
        enum E {
            S { r: u8, g: u8, b: u8 },
        }
        assert_eq!(
            serde_json::to_string(&E::S { r: 1, g: 2, b: 3 })?,
            r#"{"S":{"r":1,"g":2,"b":3}}"#
        );
        Ok(())
    }

    #[test]
    fn test_serde_json_error() -> anyhow::Result<()> {
        let mut map = HashMap::new();
        map.insert((), 1_u8);
        assert_eq!(
            serde_json::to_string(&map).unwrap_err().to_string(),
            "key must be a string"
        );
        Ok(())
    }

    #[test]
    fn test_serde_json_error2() -> anyhow::Result<()> {
        #[allow(dead_code)]
        #[derive(Debug, serde::Deserialize)]
        enum T {
            A(i64),
        }
        assert!(serde_json::from_str::<'_, T>(r#"{"B":1}"#)
            .unwrap_err()
            .to_string()
            .starts_with("unknown variant `B`, expected `A`"));
        Ok(())
    }
}
