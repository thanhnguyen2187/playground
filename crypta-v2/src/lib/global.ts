import { TriplitClient } from '@triplit/client';
import { schema } from '../data/schema-triplit';
import { createActor } from 'xstate';
import { machine as globalMachine } from '$lib/machine-app';

export const globalClient = new TriplitClient({ schema, storage: "indexeddb" });
export const globalActor = createActor(globalMachine);
globalActor.start();
