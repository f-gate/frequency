use crate::{avro, types::*};
use apache_avro::types::Record;
use std::collections::HashMap;

const PRIMITIVE_EXAMPLE: &[(&str, bool)] = &[
	(r#""null""#, true),
	(r#"{"type": "null"}"#, true),
	(r#""boolean""#, true),
	(r#"{"type": "boolean"}"#, true),
	(r#""string""#, true),
	(r#"{"type": "string"}"#, true),
	(r#""bytes""#, true),
	(r#"{"type": "bytes"}"#, true),
	(r#""int""#, true),
	(r#"{"type": "int"}"#, true),
	(r#""long""#, true),
	(r#"{"type": "long"}"#, true),
	(r#""float""#, true),
	(r#"{"type": "float"}"#, true),
	(r#""double""#, true),
	(r#"{"type": "double"}"#, true),
	(r#""true""#, false),
	(r#"true"#, false),
	(r#"{"no_type": "test"}"#, false),
	(r#"{"type": "panther"}"#, false),
];

const VALID_EXAMPLES: &[(&str, bool)] = &[
	(r#"{"type": "fixed", "name": "Test", "size": 1}"#, true),
	(
		r#"{
                "type": "fixed",
                "name": "MyFixed",
                "namespace": "org.apache.hadoop.avro",
                "size": 1
            }"#,
		true,
	),
];

const INVALID_EXAMPLES: &[(&str, bool)] = &[
	(r#"{"type": "fixed", "name": "MissingSize"}"#, false),
	(r#"{"type": "fixed", "size": 314}"#, false),
];
#[test]
fn test_fingerprint_raw() {
	for (raw_schema, expected) in PRIMITIVE_EXAMPLE {
		let schema_result = avro::fingerprint_raw_schema(raw_schema);
		if *expected {
			assert!(
				schema_result.is_ok(),
				"schema {} was supposed to be valid; error: {:?}",
				raw_schema,
				schema_result.err()
			);
		} else {
			assert!(
				schema_result.is_err(),
				"schema {} was supposed to be invalid; error: {:?}",
				raw_schema,
				schema_result.err()
			);
		}
	}
}

#[test]
/// Test that the string generated by an Avro Schema object is, in fact, a valid Avro schema.
fn test_valid_cast_to_string_after_parse() {
	for (raw_schema, expected) in VALID_EXAMPLES {
		let schema_result = avro::fingerprint_raw_schema(raw_schema);
		if *expected {
			assert!(
				schema_result.is_ok(),
				"schema {} was supposed to be valid; error: {:?}",
				raw_schema,
				schema_result.err()
			);
			let schema_res = schema_result.unwrap();
			let translate_schema = avro::translate_schema(schema_res.1);
			assert!(
				translate_schema.is_ok(),
				"schema {} was supposed to be valid; error: {:?}",
				raw_schema,
				translate_schema.err()
			);
			let translated_schema = translate_schema.unwrap();
			assert_eq!(translated_schema, schema_res.0);
		} else {
			assert!(
				schema_result.is_err(),
				"schema {} was supposed to be invalid; error: {:?}",
				raw_schema,
				schema_result.err()
			);
		}
	}
}

#[test]
fn test_get_writer_with_schema() {
	let schema_result = avro::fingerprint_raw_schema(r#"{"type": "int"}"#);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
}

#[test]
fn test_get_writer_with_data() {
	let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
    "#;
	let schema_result = avro::fingerprint_raw_schema(raw_schema);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let mut writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
	// the Record type models our Record schema
	let mut record = Record::new(writer.schema()).unwrap();
	record.put("a", 27i64);
	record.put("b", "foo");
	let result_write = writer.append(record);
	assert!(result_write.is_ok());
}

#[test]
fn test_set_writer_with_data() {
	let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
    "#;
	let schema_result = avro::fingerprint_raw_schema(raw_schema);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let mut writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
	// the Record type models our Record schema
	let mut record = Record::new(writer.schema()).unwrap();
	record.put("a", 27i64);
	record.put("b", "foo");
	let result_write = writer.append(record);
	assert!(result_write.is_ok());
}

#[test]
fn test_populate_data_records() {
	let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
    "#;
	let schema_result = avro::fingerprint_raw_schema(raw_schema);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
	// hashmap to store the data
	let mut data_map = HashMap::new();
	// the Record type models our Record schema
	data_map.insert("a".to_string(), SchemaValue::Long(27i64));
	data_map.insert("b".to_string(), SchemaValue::String("foo".to_string()));

	let result_write = avro::populate_schema_and_serialize(&translated_schema, &data_map);
	assert!(result_write.is_ok());
}

#[test]
fn test_invalid_cast_to_string_after_parse() {
	for (raw_schema, _expected) in INVALID_EXAMPLES {
		let schema_result = avro::fingerprint_raw_schema(raw_schema);
		assert!(
			schema_result.is_err(),
			"schema {} was supposed to be invalid; error: {:?}",
			raw_schema,
			schema_result.err()
		);
	}
}

#[test]
fn test_invalid_translation() {
	let bad_schema = "{\"something\": \"nothing\"}";
	let bad_bytes = bad_schema.as_bytes().to_vec();
	let schema_result = avro::translate_schema(bad_bytes);
	assert!(
		schema_result.is_err(),
		"schema {} was supposed to be invalid; error: {:?}",
		bad_schema,
		schema_result.err()
	);
}

#[test]
fn test_populate_data_serialized() {
	let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
    "#;
	let schema_result = avro::fingerprint_raw_schema(raw_schema);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
	// hashmap to store the data
	let mut data_map = HashMap::new();
	// the Record type models our Record schema
	data_map.insert("a".to_string(), SchemaValue::Long(27i64));
	data_map.insert("b".to_string(), SchemaValue::String("foo".to_string()));

	let result_write = avro::populate_schema_and_serialize(&translated_schema, &data_map);
	assert!(result_write.is_ok());
}

#[test]
fn test_reader_schema_with_data() {
	let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
    "#;
	let schema_result = avro::fingerprint_raw_schema(raw_schema);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
	// hashmap to store the data
	let mut data_map = HashMap::new();
	// the Record type models our Record schema
	data_map.insert("a".to_string(), SchemaValue::Long(27i64));
	data_map.insert("b".to_string(), SchemaValue::String("foo".to_string()));

	let result_write = avro::populate_schema_and_serialize(&translated_schema, &data_map);
	assert!(result_write.is_ok());
	let serialized_result = result_write.unwrap();
	let reader_res = avro::get_schema_data_map(&serialized_result, &translated_schema);
	assert!(reader_res.is_ok());
}

#[test]
fn test_end_to_end_flow() {
	// create a schema
	let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
    "#;
	let schema_result = avro::fingerprint_raw_schema(raw_schema);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
	// hashmap to store the data
	let mut data_map = HashMap::new();
	// the Record type models our Record schema
	data_map.insert("a".to_string(), SchemaValue::Long(27i64));
	data_map.insert("b".to_string(), SchemaValue::String("foo".to_string()));
	// write the data
	let result_write = avro::populate_schema_and_serialize(&translated_schema, &data_map);
	assert!(result_write.is_ok());
	let serialized_result = result_write.unwrap();
	// read the data
	let reader_res = avro::get_schema_data_map(&serialized_result, &translated_schema);
	assert!(reader_res.is_ok());
}

#[test]
fn test_end_to_end_flow_map() {
	// create a schema
	let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
    "#;
	let schema_result = avro::fingerprint_raw_schema(raw_schema);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
	// hashmap to store the data
	let mut data_map = HashMap::new();
	// the Record type models our Record schema
	data_map.insert("a".to_string(), SchemaValue::Long(27i64));
	data_map.insert("b".to_string(), SchemaValue::String("foo".to_string()));
	// write the data
	let result_write = avro::populate_schema_and_serialize(&translated_schema, &data_map);
	assert!(result_write.is_ok());
	let serialized_result = result_write.unwrap();
	let reader_res = avro::get_schema_data_map(&serialized_result, &translated_schema);
	assert!(reader_res.is_ok());
	let reader = reader_res.unwrap();
	assert_eq!(reader["a"], SchemaValue::Long(27i64));
	assert_eq!(reader["b"], SchemaValue::String("foo".to_string()));
}

#[test]
fn test_bad_records() {
	// create a schema
	let raw_schema = r#"
    {
        "type": "record",
        "name": "test",
        "fields": [
            {"name": "a", "type": "long", "default": 42},
            {"name": "b", "type": "string"}
        ]
    }
    "#;
	let schema_result = avro::fingerprint_raw_schema(raw_schema);
	assert!(schema_result.is_ok());
	let schema_res = schema_result.unwrap();
	let translate_schema = avro::translate_schema(schema_res.1);
	assert!(translate_schema.is_ok());
	let translated_schema = translate_schema.unwrap();
	let writer = avro::get_schema_data_writer(&translated_schema);
	assert_eq!(writer.schema(), &translated_schema);
	let mut serialized_result = Vec::new();
	// populated serialized_result
	serialized_result.push(0x0);
	let reader_res = avro::get_schema_data_map(&serialized_result, &translated_schema);
	assert!(reader_res.is_err());
}