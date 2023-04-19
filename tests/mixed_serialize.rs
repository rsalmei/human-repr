#[cfg(feature = "serde")]
mod tests {
    use human_repr::{
        HumanCount, HumanCountData, HumanDuration, HumanDurationData, HumanThroughput,
        HumanThroughputData,
    };
    use serde::{Deserialize, Serialize};

    #[test]
    fn mixed_serialize() -> Result<(), serde_json::Error> {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(bound(deserialize = "'de: 'a"))]
        enum Data<'a> {
            Count(HumanCountData<'a>),
            Duration(HumanDurationData),
            Throughput(HumanThroughputData<'a>),
        }
        let list = [
            Data::Count(123456.human_count("C")),
            Data::Duration(123456.human_duration()),
            Data::Throughput(123456.human_throughput("T")),
        ];
        let ser = serde_json::to_string(&list)?;
        assert_eq!(
            r#"[{"Count":{"val":123456.0,"unit":"C"}},{"Duration":{"val":123456.0}},{"Throughput":{"val":123456.0,"unit":"T"}}]"#,
            &ser
        );
        let list2 = serde_json::from_str::<Vec<Data>>(&ser)?;
        assert_eq!(list, list2.as_slice());
        Ok(())
    }
}
