use content_integrity::*;
use hdk::{hash_path::path::Component, prelude::*};
use zome_utils::*;

use crate::{
    dynamic_links::create_dynamic_links, hive_link::create_hive_link,
    humm_content_id_link::create_humm_content_id_link, linking::acl_links::create_acl_links,
    time_indexed_links::*,
};

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct CreateEncryptedContentInput {
    pub id: String,
    pub hive_id: String,
    pub content_type: String,
    pub bytes: SerializedBytes,
    pub entity_acl: EntityAcl,
    pub public_key_acl: PublicKeyAcl,
    pub dynamic_links: Option<Vec<String>>,
}

#[hdk_extern]
pub fn create_encrypted_content(input: CreateEncryptedContentInput) -> ExternResult<Record> {
    let encrypted_content = EncryptedContent {
        header: EncryptedContentHeader {
            id: input.id,
            hive_id: input.hive_id,
            content_type: input.content_type,
            entity_acl: input.entity_acl,
            public_key_acl: input.public_key_acl,
        },
        bytes: input.bytes,
    };
    let action_hash = create_entry(&EntryTypes::EncryptedContent(encrypted_content.clone()))?;
    let record = get(action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from(
            "Could not find the newly created EncryptedContent"
        ))
    ))?;

    // author links
    let res = create_acl_links(encrypted_content.clone(), action_hash.clone());
    if let Err(e) = res {
        return Err(e);
    }

    // hive link
    let res = create_hive_link(encrypted_content.clone(), action_hash.clone());
    if let Err(e) = res {
        return Err(e);
    }

    // content ID link
    let res = create_humm_content_id_link(encrypted_content.clone(), action_hash.clone());
    if let Err(e) = res {
        return Err(e);
    }

    // dynamic links
    if let Some(dynamic_links) = input.dynamic_links {
        let res = create_dynamic_links(
            encrypted_content.clone(),
            action_hash.clone(),
            dynamic_links,
        );
        if let Err(e) = res {
            return Err(e);
        }
    }

    // time indexing links
    let time_index =
        time_index_encrypted_content(action_hash.clone(), &encrypted_content.header.content_type);
    if let Err(e) = time_index {
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
pub struct GetByDynamicLinkInput {
    pub hive_id: String,
    pub content_type: String,
    pub dynamic_link: String,
}

#[hdk_extern]
pub fn get_by_dynamic_link(input: GetByDynamicLinkInput) -> ExternResult<Record> {
    let path = Path::from(vec![
        Component::from(input.hive_id),
        Component::from(input.content_type),
        Component::from(input.dynamic_link.clone()),
    ]);

    let links = get_links(path.path_entry_hash()?, LinkTypes::Hive, None)?;
    if links.is_empty() {
        return Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Could not find the EncryptedContent at dynamic link {0}",
            input.dynamic_link
        ))));
    }
    let ah = links[0]
        .clone()
        .target
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "No action hash associated with link"
        ))))?;
    let record =
        get(ah, GetOptions::default())?.ok_or(wasm_error!(WasmErrorInner::Guest(format!(
            "Could not find the EncryptedContent at dynamic link {0}",
            input.dynamic_link
        ))))?;
    Ok(record)
}

#[hdk_extern]
pub fn list_by_dynamic_link(input: GetByDynamicLinkInput) -> ExternResult<Vec<Record>> {
    let path = Path::from(vec![
        Component::from(input.hive_id),
        Component::from(input.content_type),
        Component::from(input.dynamic_link.clone()),
    ]);
    let links = get_links(path.path_entry_hash()?, LinkTypes::Hive, None)?;
    let records: Vec<Record> = links
        .into_iter()
        .map(|link| {
            let ah = link
                .target
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                    "No action hash associated with link"
                ))));

            if let Err(e) = ah {
                return Err(e);
            }

            let record = get(ah.unwrap(), GetOptions::default())?.ok_or(wasm_error!(
                WasmErrorInner::Guest(format!(
                    "Could not find the EncryptedContent at dynamic link {0}",
                    input.dynamic_link
                ))
            ));

            if let Err(e) = record {
                return Err(e);
            }

            record
        })
        .filter_map(|x| x.ok())
        .collect();

    Ok(records)
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
