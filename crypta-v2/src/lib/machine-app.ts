import { setup } from "xstate";

export const machine = setup({
	types: {
		context: {} as {},
		events: {} as
			| { type: "Save" }
			| { type: "Check" }
			| { type: "Cancel" }
			| { type: "Failed" }
			| { type: "Succeded" }
			| { type: "Succeeded" }
			| { type: "OpenFilter" }
			| { type: "FailedAction" }
			| { type: "OpenSettings" }
			| { type: "SucceededAction" }
			| { type: "Retry" }
			| { type: "Input" }
			| { type: "Clear" }
			| { type: "Retried" }
			| { type: "Loaded" },
	},
	guards: {
		RemoteAvailable: function ({ context, event }) {
			// Add your guard condition here
			return true;
		},
	},
}).createMachine({
	context: {},
	id: "Crypta V2",
	initial: "Migrating",
	states: {
		Migrating: {
			type: "parallel",
			states: {
				MigrationLocal: {
					initial: "Doing",
					states: {
						Doing: {
							on: {
								Succeeded: {
									target: "#Crypta V2.DataLoading",
								},
								Failed: {
									target: "#Crypta V2.DataError",
								},
							},
						},
					},
				},
				MigrationRemote: {
					initial: "Doing",
					states: {
						Doing: {
							on: {
								Succeeded: {
									target: "Done",
								},
								Failed: {
									target: "Warning",
								},
							},
						},
						Done: {},
						Warning: {},
					},
				},
			},
		},
		DataLoading: {
			type: "parallel",
			on: {
				OpenFilter: {
					target: "Filter",
				},
				OpenSettings: {
					target: "Settings",
				},
			},
			states: {
				Remote: {
					initial: "Transient",
					states: {
						Transient: {
							on: {
								Check: [
									{
										target: "Loading",
										guard: {
											type: "RemoteAvailable",
										},
									},
									{
										target: "Final",
									},
								],
							},
						},
						Loading: {
							type: "parallel",
							on: {
								Succeded: {
									target: "Done",
								},
								Failed: {
									target: "#Crypta V2.DataError",
								},
							},
							states: {
								First: {},
								More: {},
							},
						},
						Final: {
							type: "final",
						},
						Done: {
							type: "parallel",
							on: {
								FailedAction: {
									target: "Warning",
								},
								SucceededAction: {
									target: "Done",
								},
								Loaded: {
									target: "LoadingMore",
								},
							},
						},
						Warning: {
							on: {
								Retried: {
									target: "Retrying",
								},
							},
						},
						LoadingMore: {},
						Retrying: {
							on: {
								SucceededAction: {
									target: "Done",
								},
								FailedAction: {
									target: "Warning",
								},
							},
						},
					},
				},
				Local: {
					initial: "Loading",
					states: {
						Loading: {
							type: "parallel",
							on: {
								Succeeded: {
									target: "Done",
								},
								Failed: {
									target: "#Crypta V2.DataError",
								},
							},
							states: {
								First: {},
								More: {},
							},
						},
						Done: {
							type: "parallel",
							on: {
								FailedAction: {
									target: "#Crypta V2.DataError",
								},
								SucceededAction: {
									target: "Done",
								},
							},
						},
					},
				},
			},
		},
		Filter: {
			on: {
				Cancel: {
					target: "DataLoading",
				},
			},
		},
		Settings: {
			initial: "URLEmpty",
			on: {
				Save: {
					target: "Migrating",
				},
				Cancel: {
					target: "DataLoading",
				},
			},
			states: {
				URLEmpty: {
					on: {
						Input: {
							target: "URLFilled",
						},
					},
				},
				URLFilled: {
					initial: "Checking",
					on: {
						Clear: {
							target: "URLEmpty",
						},
					},
					states: {
						Checking: {
							on: {
								Succeeded: {
									target: "Done",
								},
							},
						},
						Done: {
							on: {
								Failed: {
									target: "Error",
								},
								Retry: {
									target: "Checking",
								},
							},
						},
						Error: {
							on: {
								Retry: {
									target: "Checking",
								},
							},
						},
					},
				},
			},
		},
		DataError: {},
	},
});
