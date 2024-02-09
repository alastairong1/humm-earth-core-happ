use content_integrity::{EncryptedContent, EntityAclEntry, LinkTypes};
use hdk::{hash_path::path::Component, prelude::*};

// TODO: use the public key acl instead of the entity acl

pub fn create_acl_links(
    encrypted_content: EncryptedContent,
    action_hash: ActionHash,
) -> ExternResult<Vec<ActionHash>> {
    // add a link for each author based on the ACK admin and write fields
    let mut acl_link_action_hashes: Vec<ActionHash> = vec![];
    let owner = encrypted_content.header.entity_acl.owner;
    let admins: Vec<EntityAclEntry> = encrypted_content.header.entity_acl.admin.clone();
    let writers: Vec<EntityAclEntry> = encrypted_content
        .header
        .entity_acl
        .admin
        .iter()
        .chain(encrypted_content.header.entity_acl.writer.clone().iter())
        .cloned()
        .collect();
    let readers: Vec<EntityAclEntry> = writers
        .iter()
        .chain(encrypted_content.header.entity_acl.reader.clone().iter())
        .cloned()
        .collect();

    // owner
    let owner_path = Path::from(vec![
        Component::from(encrypted_content.header.hive_id.clone()),
        Component::from(encrypted_content.header.content_type.clone()),
        Component::from(owner.id.to_string()),
    ]);

    let owner_ah = create_link(
        owner_path
            .path_entry_hash()
            .expect(format!("could not get path entry hash for owner: '{}'", owner.id).as_str()),
        action_hash.clone(),
        LinkTypes::HummContentOwner,
        (),
    );
    acl_link_action_hashes
        .push(owner_ah.expect(format!("could not create link for owner: '{}'", owner.id).as_str()));

    admins.iter().for_each(|acl_entry| {
        let path = Path::from(vec![
            Component::from(encrypted_content.header.hive_id.clone()),
            Component::from(encrypted_content.header.content_type.clone()),
            Component::from(acl_entry.id.to_string()),
        ]);

        let ah = create_link(
            path.path_entry_hash().expect(
                format!(
                    "could not get path entry hash for admin: '{}'",
                    acl_entry.id
                )
                .as_str(),
            ),
            action_hash.clone(),
            LinkTypes::HummContentAdmin,
            (),
        );
        acl_link_action_hashes.push(
            ah.expect(format!("could not create link for admin: '{}'", acl_entry.id).as_str()),
        );
    });

    writers.iter().for_each(|acl_entry| {
        let path = Path::from(vec![
            Component::from(encrypted_content.header.hive_id.clone()),
            Component::from(encrypted_content.header.content_type.clone()),
            Component::from(acl_entry.id.to_string()),
        ]);

        let ah = create_link(
            path.path_entry_hash().expect(
                format!(
                    "could not get path entry hash for writer: '{}'",
                    acl_entry.id
                )
                .as_str(),
            ),
            action_hash.clone(),
            LinkTypes::HummContentWriter,
            (),
        );
        acl_link_action_hashes.push(
            ah.expect(format!("could not create link for writer: '{}'", acl_entry.id).as_str()),
        );
    });

    readers.iter().for_each(|acl_entry| {
        let path = Path::from(vec![
            Component::from(encrypted_content.header.hive_id.clone()),
            Component::from(encrypted_content.header.content_type.clone()),
            Component::from(acl_entry.id.to_string()),
        ]);

        let ah = create_link(
            path.path_entry_hash().expect(
                format!(
                    "could not get path entry hash for reader: '{}'",
                    acl_entry.id
                )
                .as_str(),
            ),
            action_hash.clone(),
            LinkTypes::HummContentReader,
            (),
        );
        acl_link_action_hashes.push(
            ah.expect(format!("could not create link for reader: '{}'", acl_entry.id).as_str()),
        );
    });

    Ok(acl_link_action_hashes)
}
