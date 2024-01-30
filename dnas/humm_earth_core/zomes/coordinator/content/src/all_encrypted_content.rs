use content_integrity::*;
use hdk::{hash_path::path::Component, prelude::*};

///////////////// split this into multiple different links?

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllEncryptedContentAndContentTypeInput {
    pub author: String,
    pub content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllEncryptedContentByHiveAndContentTypeInput {
    pub hive_id: String,
    pub content_type: String,
}

#[hdk_extern]
pub fn get_all_encrypted_content_by_author_and_content_type(
    input: GetAllEncryptedContentAndContentTypeInput,
) -> ExternResult<Vec<Record>> {
    let path = Path::from(vec![
        Component::from(input.author.to_string()),
        Component::from(input.content_type),
    ]);

    let links = get_links(path.path_entry_hash()?, LinkTypes::HummContentOwner, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| {
            Ok(GetInput::new(
                link.target
                    .into_action_hash()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                        "No action hash associated with link"
                    ))))?
                    .into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}

#[hdk_extern]
pub fn get_all_encrypted_content_by_hive_and_content_type(
    input: GetAllEncryptedContentByHiveAndContentTypeInput,
) -> ExternResult<Vec<Record>> {
    let path = Path::from(vec![
        Component::from(input.hive_id),
        Component::from(input.content_type),
    ]);
    let links = get_links(path.path_entry_hash()?, LinkTypes::Hive, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| {
            Ok(GetInput::new(
                link.target
                    .into_action_hash()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                        "No action hash associated with link"
                    ))))?
                    .into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}
