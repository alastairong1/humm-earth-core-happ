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

export type EncryptedContentResponse = {
  encrypted_content: any;
  hash: ActionHash;
};

export enum AclRole {
  Owner = "Owner",
  Admin = "Admin",
  Writer = "Writer",
  Reader = "Reader",
}

export function sampleAcl() {
  return {
    owner: {
      id: "test-entity-acl-id",
      entity_type: "test-entity-acl-type",
    },
    admin: [],
    writer: [],
    reader: [],
  };
}

export function sampleEncryptedContent(partialEncryptedContent = {}) {
  return {
    bytes: Buffer.from("test-bytes"),
    ...partialEncryptedContent,
    header: {
      id: "test-id",
      hive_id: "test-hive-id",
      content_type: "test-content-type",
      acl: sampleAcl(),
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
  partialEncryptedContent = {},
  dynamicLinks = []
) {
  const sample = sampleEncryptedContent(partialEncryptedContent);
  return {
    id: sample.header.id,
    hive_id: sample.header.hive_id,
    content_type: sample.header.content_type,
    bytes: sample.bytes,
    acl: sample.header.acl,
    public_key_acl: sample.header.public_key_acl,
    dynamic_links: dynamicLinks,
  };
}

export async function createEncryptedContent(
  cell: CallableCell,
  createEncryptedContentInput = undefined
): Promise<EncryptedContentResponse> {
  const content =
    createEncryptedContentInput || (await sampleCreateEncryptedContentInput());
  return cell.callZome({
    zome_name: "content",
    fn_name: "create_encrypted_content",
    payload: content,
  });
}
