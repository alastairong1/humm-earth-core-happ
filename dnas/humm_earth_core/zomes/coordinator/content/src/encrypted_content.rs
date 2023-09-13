use content_integrity::*;
use hdk::{hash_path::path::Component, prelude::*};
use zome_utils::*;

use crate::indexing::*;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct CreateEncryptedContentInput {
    pub id: String,
    pub hive_id: String,
    pub content_type: String,
    pub bytes: SerializedBytes,
    pub acl: Acl,
}

#[hdk_extern]
pub fn create_encrypted_content(input: CreateEncryptedContentInput) -> ExternResult<Record> {
    let encrypted_content = EncryptedContent {
        header: EncryptedContentHeader {
            id: input.id,
            hive_id: input.hive_id,
            content_type: input.content_type,
        },
        bytes: input.bytes,
        acl: input.acl,
    };
    let encrypted_content_hash =
        create_entry(&EntryTypes::EncryptedContent(encrypted_content.clone()))?;
    let record = get(encrypted_content_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from(
            "Could not find the newly created EncryptedContent"
        ))
    ))?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    // let link_tag = LinkTag::new(String::from(encrypted_content.content_type.clone()));
    let author_path = Path::from(vec![
        Component::from(my_agent_pub_key.to_string()),
        Component::from(encrypted_content.header.content_type.clone()),
    ]);
    let hive_path = Path::from(vec![
        Component::from(encrypted_content.header.hive_id),
        Component::from(encrypted_content.header.content_type.clone()),
    ]);
    create_link(
        author_path.path_entry_hash()?,
        encrypted_content_hash.clone(),
        LinkTypes::AllEncryptedContent,
        (),
    )?;
    create_link(
        hive_path.path_entry_hash()?,
        encrypted_content_hash.clone(),
        LinkTypes::AllEncryptedContent,
        (),
    )?;

    let time = get(encrypted_content_hash.clone(), GetOptions::content())?
        .unwrap()
        .action()
        .timestamp();
    let index = index_encrypted_content(
        encrypted_content_hash.clone(),
        &encrypted_content.header.content_type,
        time,
    );

    if let Err(e) = index {
        return Err(e);
    }

    Ok(record)
}

#[hdk_extern]
pub fn get_encrypted_content(
    content_hash: ActionHash,
) -> ExternResult<(Timestamp, AgentPubKey, EncryptedContent)> {
    let res = match get(content_hash.clone(), GetOptions::content())? {
        Some(record) => {
            let action = record.action().clone();
            let Ok(typed) = get_typed_from_record::<EncryptedContent>(record)
      else { return zome_error!("get_encrypted_content(): Entry not EncryptedContent") };

            Ok((action.timestamp(), action.author().to_owned(), typed))
        }
        None => zome_error!("get_encrypted_content(): Entry not found"),
    };
    res
}

#[hdk_extern]
pub fn get_many_encrypted_content(
    ahs: Vec<ActionHash>,
) -> ExternResult<Vec<(Timestamp, AgentPubKey, EncryptedContent)>> {
    return ahs
        .into_iter()
        .map(|ah| get_encrypted_content(ah))
        .collect();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetEncryptedContentByTimeAndAuthorInput {
    author: AgentPubKey,
    content_type: String,
    start_time: Option<Timestamp>,
    end_time: Option<Timestamp>,
    limit: Option<usize>,
}

#[hdk_extern]
pub fn get_encrypted_content_by_time_and_author(
    input: GetEncryptedContentByTimeAndAuthorInput,
) -> ExternResult<Vec<(Timestamp, AgentPubKey, EncryptedContent)>> {
    let res = get_encrypted_content_time_index_links(
        input.author,
        &input.content_type,
        input.start_time,
        input.end_time,
        input.limit,
    )?;
    let hashes: Vec<ActionHash> = res
        .1
        .into_iter()
        .map(|(_, link)| link.target.into_action_hash())
        .filter_map(|x| x)
        .collect();
    get_many_encrypted_content(hashes)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEncryptedContentInput {
    pub original_encrypted_content_hash: ActionHash,
    pub previous_encrypted_content_hash: ActionHash,
    pub updated_encrypted_content: EncryptedContent,
}

#[hdk_extern]
pub fn update_encrypted_content(input: UpdateEncryptedContentInput) -> ExternResult<Record> {
    let updated_encrypted_content_hash = update_entry(
        input.previous_encrypted_content_hash.clone(),
        &input.updated_encrypted_content,
    )?;
    create_link(
        input.original_encrypted_content_hash.clone(),
        updated_encrypted_content_hash.clone(),
        LinkTypes::EncryptedContentUpdates,
        (),
    )?;
    // TODO: create time link. get rid of default links and update links?
    let record = get(
        updated_encrypted_content_hash.clone(),
        GetOptions::default(),
    )?
    .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
        "Could not find the newly updated EncryptedContent"
    ))))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_encrypted_content(
    original_encrypted_content_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_encrypted_content_hash)
    // TODO: delete links
}
