import { assign, setup } from "xstate";
import type { NoteDisplay } from "../data/schema-triplit";
import { createEmptyNoteDisplay } from '../data/data-transformation';

export const machine = setup({
	types: {
		context: {} as {
			notes: NoteDisplay[]
			note: NoteDisplay
		},
		events: {} as
			| { type: "Save" }
			| { type: "Check" }
			| { type: "Cancel" }
			| { type: "Failed" }
			| { type: "FailedData" }
			| { type: "Succeeded" }
			| { type: "OpenFilter" }
			| { type: "FailedAction" }
			| { type: "OpenSettings" }
			| { type: "SucceededAction" }
			| { type: "Retry" }
			| { type: "Input" }
			| { type: "Clear" }
			| { type: "Retried" }
			| { type: "Error" }
			| { type: "Loaded"; notes: NoteDisplay[] }
			| { type: "Reload" }
			| { type: "ModalOpenNote"; note: NoteDisplay }
			| { type: "ModalCancel" },
	},
	guards: {
		IsNotesEmpty: ({ context }) => context.notes.length === 0,
	},
}).createMachine({
	context: {
		notes: [],
		note: createEmptyNoteDisplay(),
	},
	id: "AppState",
	initial: "Functioning",
	states: {
		Functioning: {
			initial: "Loading",
			on: {
				FailedData: "DataError",
			},
			states: {
				Loading: {
					on: {
						Loaded: {
							target: "Idling",
							actions: assign({
								notes: ({event}) => event.notes,
							})
						},
						Error: "..DataError",
					},
				},
				Idling: {
					type: "parallel",
					on: {
						ModalOpenNote: {
							target: ".Modal.Note",
							actions: assign({
								note: ({event}) => event.note,
							})
						},
						Reload: {
							target: "Loading",
						}
					},
					states: {
						Items: {
							initial: "Transient",
							states: {
								Transient: {
									always: [
										{
											guard: "IsNotesEmpty",
											target: "Blank",
										},
										{
											target: "Filled",
										},
									],
								},
								Blank: {},
								Filled: {},
							},
						},
						Toaster: {},
						LoadingMore: {
							initial: "Idling",
							states: {
								Idling: {},
								Working: {},
							},
						},
						Modal: {
							initial: "None",
							on: {
								ModalCancel: {},
							},
							states: {
								None: {},
								Encryption: {},
								Settings: {},
								Note: {},
							},
						},
					},
				},
			},
		},
		DataError: {},
	},
});
