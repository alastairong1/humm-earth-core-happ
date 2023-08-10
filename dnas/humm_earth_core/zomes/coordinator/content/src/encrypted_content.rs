use hdk::prelude::*;
use content_integrity::*;
#[hdk_extern]
pub fn create_encrypted_content(
    encrypted_content: EncryptedContent,
) -> ExternResult<Record> {
    let encrypted_content_hash = create_entry(
        &EntryTypes::EncryptedContent(encrypted_content.clone()),
    )?;
    let record = get(encrypted_content_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created EncryptedContent"))
            ),
        )?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(
        my_agent_pub_key,
        encrypted_content_hash.clone(),
        LinkTypes::AllEncryptedContentByAuthor,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_encrypted_content(
    original_encrypted_content_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_encrypted_content_hash.clone(),
        LinkTypes::EncryptedContentUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_encrypted_content_hash = match latest_link {
        Some(link) => {
            link
                .target
                .clone()
                .into_action_hash()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest(String::from("No action hash associated with link"))
                    ),
                )?
        }
        None => original_encrypted_content_hash.clone(),
    };
    get(latest_encrypted_content_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEncryptedContentInput {
    pub original_encrypted_content_hash: ActionHash,
    pub previous_encrypted_content_hash: ActionHash,
    pub updated_encrypted_content: EncryptedContent,
}
#[hdk_extern]
pub fn update_encrypted_content(
    input: UpdateEncryptedContentInput,
) -> ExternResult<Record> {
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
    let record = get(updated_encrypted_content_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated EncryptedContent"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_encrypted_content(
    original_encrypted_content_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_encrypted_content_hash)
}
