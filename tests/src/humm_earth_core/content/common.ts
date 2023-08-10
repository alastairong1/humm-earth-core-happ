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

export async function sampleEncryptedContent(
  cell: CallableCell,
  partialEncryptedContent = {}
) {
  return {
    ...{
      id: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      content_type: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      bytes: [10],
    },
    ...partialEncryptedContent,
  };
}

export async function createEncryptedContent(
  cell: CallableCell,
  encryptedContent = undefined
): Promise<Record> {
  return cell.callZome({
    zome_name: "content",
    fn_name: "create_encrypted_content",
    payload: encryptedContent || (await sampleEncryptedContent(cell)),
  });
}
