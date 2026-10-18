    pub fn load<R: Read>(reader: R) -> io::Result<Self> {
        let value: serde_json::Value = serde_json::from_reader(reader)?;
        match value {
            serde_json::Value::Object(mut obj) => {
                let ver = serde_json::from_value(
                    obj.get("version")
                        .ok_or(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Expected a version entry",
                        ))?
                        .clone(),
                )?;
                match ver {
                    SerdeVersion {
                        major: 3,
                        minor: 0..=0,
                    } => {
                        obj.insert(
                            "version".into(),
                            serde_json::Value::Number(ver.minor.into()),
                        );
                        Ok(serde_json::from_value(serde_json::Value::Object(obj))?)
                    }
                    SerdeVersion { major, minor } => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unhandled version: {major}.{minor}"),
                    )),
                }
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected a JSON object",
            )),
        }
    }

    /// Intended for testing only. This method enables fully controlling the inputs passed to
