import { assert, expect, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from "@holochain/tryorama";
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
import { decode, encode } from "@msgpack/msgpack";

import {
  AclRole,
  EncryptedContentResponse,
  createEncryptedContent,
  sampleCreateEncryptedContentInput,
  sampleEncryptedContent,
} from "../common.js";

test("create and read EncryptedContent using acl owner link", async () => {
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
    const sampleContent = sampleEncryptedContent();
    const sampleInput = await sampleCreateEncryptedContentInput(sampleContent);
    const record = await createEncryptedContent(alice.cells[0], sampleInput);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created EncryptedContent
    const listInput = {
      hive_id: sampleContent.header.hive_id,
      content_type: sampleContent.header.content_type,
      acl_role: AclRole.Owner,
      entity_id: sampleContent.header.acl.owner.id,
    };
    const createReadOutput: EncryptedContentResponse[] =
      await bob.cells[0].callZome({
        zome_name: "content",
        fn_name: "list_by_acl_link",
        payload: listInput,
      });

    assert.deepEqual(sampleContent, createReadOutput[0].encrypted_content);
  });
});

test("create and read EncryptedContent using acl admin link", async () => {
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
    const sampleContent = sampleEncryptedContent();
    sampleContent.header.acl.admin.push({
      id: "test-admin-id",
      entity_type: "test-entity-acl-type",
    });
    const sampleInput = await sampleCreateEncryptedContentInput(sampleContent);
    const record = await createEncryptedContent(alice.cells[0], sampleInput);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created EncryptedContent
    const listInput = {
      hive_id: sampleContent.header.hive_id,
      content_type: sampleContent.header.content_type,
      acl_role: AclRole.Admin,
      entity_id: sampleContent.header.acl.admin[0].id,
    };
    const createReadOutput: EncryptedContentResponse[] =
      await bob.cells[0].callZome({
        zome_name: "content",
        fn_name: "list_by_acl_link",
        payload: listInput,
      });
    console.log(createReadOutput);

    assert.deepEqual(sampleContent, createReadOutput[0].encrypted_content);
  });
});

test("create and read EncryptedContent using acl writer link", async () => {
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
    const sampleContent = sampleEncryptedContent();
    sampleContent.header.acl.writer.push({
      id: "test-writer-id",
      entity_type: "test-entity-acl-type",
    });
    const sampleInput = await sampleCreateEncryptedContentInput(sampleContent);
    const record = await createEncryptedContent(alice.cells[0], sampleInput);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created EncryptedContent
    const listInput = {
      hive_id: sampleContent.header.hive_id,
      content_type: sampleContent.header.content_type,
      acl_role: AclRole.Writer,
      entity_id: sampleContent.header.acl.writer[0].id,
    };
    const createReadOutput: EncryptedContentResponse[] =
      await bob.cells[0].callZome({
        zome_name: "content",
        fn_name: "list_by_acl_link",
        payload: listInput,
      });
    console.log(createReadOutput);

    assert.deepEqual(sampleContent, createReadOutput[0].encrypted_content);
  });
});

test("create and read EncryptedContent using acl reader link", async () => {
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
    const sampleContent = sampleEncryptedContent();
    sampleContent.header.acl.reader.push({
      id: "test-reader-id",
      entity_type: "test-entity-acl-type",
    });
    const sampleInput = await sampleCreateEncryptedContentInput(sampleContent);
    const record = await createEncryptedContent(alice.cells[0], sampleInput);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created EncryptedContent
    const listInput = {
      hive_id: sampleContent.header.hive_id,
      content_type: sampleContent.header.content_type,
      acl_role: AclRole.Reader,
      entity_id: sampleContent.header.acl.reader[0].id,
    };
    const createReadOutput: EncryptedContentResponse[] =
      await bob.cells[0].callZome({
        zome_name: "content",
        fn_name: "list_by_acl_link",
        payload: listInput,
      });

    assert.deepEqual(sampleContent, createReadOutput[0].encrypted_content);
  });
});
