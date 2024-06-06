import { TriplitClient } from '@triplit/client';
import { schema } from '../data/schema-triplit';

export const globalClient = new TriplitClient({ schema });
