// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

#[test]
fn test_array_aggregate() {
    use std::sync::Arc;

    use super::*;

    #[allow(dead_code)]
    struct ArrayTest {
        name: &'static str,
        args: Vec<DataArrayRef>,
        expect: Vec<DataValue>,
        error: Vec<&'static str>,
        op: DataValueAggregateOperator,
    }

    let tests = vec![
        ArrayTest {
            name: "min-passed",
            args: vec![
                Arc::new(StringArray::from(vec!["x1", "x2"])),
                Arc::new(Int8Array::from(vec![1, 2, 3, 4])),
                Arc::new(Int16Array::from(vec![4, 3, 2, 1])),
                Arc::new(Int32Array::from(vec![4, 3, 2, 1])),
                Arc::new(Int64Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt8Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt16Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt32Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt64Array::from(vec![4, 3, 2, 1])),
                Arc::new(Float32Array::from(vec![4.0, 3.0, 2.0, 1.0])),
                Arc::new(Float64Array::from(vec![4.0, 3.0, 2.0, 1.0])),
            ],
            op: DataValueAggregateOperator::Min,
            expect: vec![
                DataValue::String(Some("x1".to_string())),
                DataValue::Int8(Some(1)),
                DataValue::Int16(Some(1)),
                DataValue::Int32(Some(1)),
                DataValue::Int64(Some(1)),
                DataValue::UInt8(Some(1)),
                DataValue::UInt16(Some(1)),
                DataValue::UInt32(Some(1)),
                DataValue::UInt64(Some(1)),
                DataValue::Float32(Some(1.0)),
                DataValue::Float64(Some(1.0)),
            ],
            error: vec![""],
        },
        ArrayTest {
            name: "max-passed",
            args: vec![
                Arc::new(StringArray::from(vec!["x1", "x2"])),
                Arc::new(Int8Array::from(vec![1, 2, 3, 4])),
                Arc::new(Int16Array::from(vec![4, 3, 2, 1])),
                Arc::new(Int32Array::from(vec![4, 3, 2, 1])),
                Arc::new(Int64Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt8Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt16Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt32Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt64Array::from(vec![4, 3, 2, 1])),
                Arc::new(Float32Array::from(vec![4.0, 3.0, 2.0, 1.0])),
                Arc::new(Float64Array::from(vec![4.0, 3.0, 2.0, 1.0])),
            ],
            op: DataValueAggregateOperator::Max,
            expect: vec![
                DataValue::String(Some("x2".to_string())),
                DataValue::Int8(Some(4)),
                DataValue::Int16(Some(4)),
                DataValue::Int32(Some(4)),
                DataValue::Int64(Some(4)),
                DataValue::UInt8(Some(4)),
                DataValue::UInt16(Some(4)),
                DataValue::UInt32(Some(4)),
                DataValue::UInt64(Some(4)),
                DataValue::Float32(Some(4.0)),
                DataValue::Float64(Some(4.0)),
            ],
            error: vec![""],
        },
        ArrayTest {
            name: "sum-passed",
            args: vec![
                Arc::new(StringArray::from(vec!["xx"])),
                Arc::new(Int8Array::from(vec![1, 2, 3, 4])),
                Arc::new(Int16Array::from(vec![4, 3, 2, 1])),
                Arc::new(Int32Array::from(vec![4, 3, 2, 1])),
                Arc::new(Int64Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt8Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt16Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt32Array::from(vec![4, 3, 2, 1])),
                Arc::new(UInt64Array::from(vec![4, 3, 2, 1])),
                Arc::new(Float32Array::from(vec![4.0, 3.0, 2.0, 1.0])),
                Arc::new(Float64Array::from(vec![4.0, 3.0, 2.0, 1.0])),
            ],
            op: DataValueAggregateOperator::Sum,
            expect: vec![
                DataValue::String(Some("xx".to_string())),
                DataValue::Int8(Some(10)),
                DataValue::Int16(Some(10)),
                DataValue::Int32(Some(10)),
                DataValue::Int64(Some(10)),
                DataValue::UInt8(Some(10)),
                DataValue::UInt16(Some(10)),
                DataValue::UInt32(Some(10)),
                DataValue::UInt64(Some(10)),
                DataValue::Float32(Some(10.0)),
                DataValue::Float64(Some(10.0)),
            ],
            error: vec!["Internal Error: Unsupported data_array_sum for data type: Utf8"],
        },
    ];

    for t in tests {
        for (i, args) in t.args.iter().enumerate() {
            let result = data_array_aggregate_op(t.op.clone(), args.clone());
            match result {
                Ok(ref v) => {
                    // Result check.
                    if *v != t.expect[i] {
                        println!("{}, expect:\n{:?} \nactual:\n{:?}", t.name, t.expect[i], v);
                        assert!(false);
                    }
                }
                Err(e) => assert_eq!(t.error[i], e.to_string()),
            }
        }
    }
}