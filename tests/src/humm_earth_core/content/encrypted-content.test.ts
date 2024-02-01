import { assert, expect, test } from "vitest";

import { runScenario, pause, CallableCell } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeDnaHash,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
} from "@holochain/client";
import { decode } from "@msgpack/msgpack";

import {
  createEncryptedContent,
  sampleCreateEncryptedContentInput,
  sampleEncryptedContent,
} from "./common.js";

test("create EncryptedContent", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/humm-earth-core-happ.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a EncryptedContent
    const record: Record = await createEncryptedContent(alice.cells[0]);
    assert.ok(record);
  });
});

test("create and read EncryptedContent", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/humm-earth-core-happ.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const sampleContent = sampleEncryptedContent();
    const sampleInput = await sampleCreateEncryptedContentInput(
      alice.cells[0],
      sampleContent
    );

    // Alice creates a EncryptedContent
    const record: Record = await createEncryptedContent(
      alice.cells[0],
      sampleInput
    );
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created EncryptedContent
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "content",
      fn_name: "get_encrypted_content",
      payload: record.signed_action.hashed.hash,
    });
    console.log(createReadOutput);
    assert.deepEqual(sampleContent, createReadOutput[2]);
  });
});

test("create and update EncryptedContent", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/humm-earth-core-happ.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a EncryptedContent
    const record: Record = await createEncryptedContent(alice.cells[0]);
    assert.ok(record);

    const originalActionHash = record.signed_action.hashed.hash;

    // Alice updates the EncryptedContent
    const contentUpdate = sampleEncryptedContent({
      bytes: Buffer.from("test-bytes-2"),
    });
    let updateInput = {
      original_encrypted_content_hash: originalActionHash,
      previous_encrypted_content_hash: originalActionHash,
      updated_encrypted_content: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "content",
      fn_name: "update_encrypted_content",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the updated EncryptedContent
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "content",
      fn_name: "get_encrypted_content",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, readUpdatedOutput0[2]);

    // Alice updates the EncryptedContent again
    const contentUpdate2 = sampleEncryptedContent({
      bytes: Buffer.from("test-bytes-3"),
    });

    updateInput = {
      original_encrypted_content_hash: originalActionHash,
      previous_encrypted_content_hash: updatedRecord.signed_action.hashed.hash,
      updated_encrypted_content: contentUpdate2,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "content",
      fn_name: "update_encrypted_content",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the updated EncryptedContent
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "content",
      fn_name: "get_encrypted_content",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate2, readUpdatedOutput1[2]);
  });
});

test("create and delete EncryptedContent", async () => {
  await runScenario(async (scenario) => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + "/../workdir/humm-earth-core-happ.happ";

    // Set up the app to be installed
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([
      appSource,
      appSource,
    ]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Alice creates a EncryptedContent
    const record: Record = await createEncryptedContent(alice.cells[0]);
    assert.ok(record);

    // Alice deletes the EncryptedContent
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "content",
      fn_name: "delete_encrypted_content",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);

    // Bob tries to get the deleted EncryptedContent
    await expect(
      async () =>
        await bob.cells[0].callZome({
          zome_name: "content",
          fn_name: "get_encrypted_content",
          payload: record.signed_action.hashed.hash,
        })
    ).rejects.toThrow();
  });
});
