import { CallableCell } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
  fakeDnaHash,
} from "@holochain/client";

export function sampleEncryptedContent(partialEncryptedContent = {}) {
  return {
    bytes: Buffer.from("test-bytes"),
    ...partialEncryptedContent,
    header: {
      id: "test-id",
      hive_id: "test-hive-id",
      content_type: "test-content-type",
      entity_acl: {
        owner: {
          id: "test-entity-acl-id",
          entity_type: "test-entity-acl-type",
        },
        admin: [],
        writer: [],
        reader: [],
      },
      public_key_acl: {
        owner: "test-entity-acl-public-key",
        admin: [],
        writer: [],
        reader: [],
      },
      ...((partialEncryptedContent as any).header || {}),
    },
  };
}

export async function sampleCreateEncryptedContentInput(
  cell: CallableCell,
  partialEncryptedContent = {}
) {
  const sample = sampleEncryptedContent(partialEncryptedContent);
  return {
    id: sample.header.id,
    hive_id: sample.header.hive_id,
    content_type: sample.header.content_type,
    bytes: sample.bytes,
    entity_acl: sample.header.entity_acl,
    public_key_acl: sample.header.public_key_acl,
    dynamic_links: [],
  };
}

export async function createEncryptedContent(
  cell: CallableCell,
  encryptedContent = undefined
): Promise<Record> {
  const content =
    encryptedContent || (await sampleCreateEncryptedContentInput(cell));
  return cell.callZome({
    zome_name: "content",
    fn_name: "create_encrypted_content",
    payload: content,
  });
}
