import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from "@holochain/tryorama";
import {
  NewEntryAction,
  ActionHash,
  Record,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeEntryHash,
} from "@holochain/client";
import { decode } from "@msgpack/msgpack";

import { createEncryptedContent } from "./common.js";

test("create a EncryptedContent and get all encrypted content by author", async () => {
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

    // Bob gets all encrypted content by author
    let collectionOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "content",
      fn_name: "get_all_encrypted_content",
      payload: alice.agentPubKey,
    });
    assert.equal(collectionOutput.length, 0);

    // Alice creates a EncryptedContent
    const createdRecord: Record = await createEncryptedContent(alice.cells[0]);
    assert.ok(createdRecord);

    await pause(1200);

    // Bob gets all encrypted content by author again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "content",
      fn_name: "get_all_encrypted_content",
      payload: alice.agentPubKey,
    });
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(createdRecord, collectionOutput[0]);
  });
});
