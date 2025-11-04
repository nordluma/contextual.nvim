vim.api.nvim_create_user_command("NewNote", function(opts)
	require("contextual").create_note(opts)
end, { range = true })

vim.api.nvim_create_user_command("SyncTodos", function(opts)
	require("contextual").sync_todos(opts)
end, {})
