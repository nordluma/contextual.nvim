local jsonrpc = require("contextual.jsonrpc")

local M = {}

M.setup = function(opts)
	-- TODO: define user configurable options
	opts = opts or {}
end

--- Retrieve the lines and columns of visually highlighted text.
---
--- Neovim automatically sets the `'<` and `'>` marks while in visual selection
--- so we'll use them to get the precise selection.
---@return integer: Start line number
---@return integer: Start column number
---@return integer: End line number
---@return integer: End column number
local get_selection_positions = function()
	local start_pos = vim.fn.getpos("'<")
	local end_pos = vim.fn.getpos("'>")
	return start_pos[2], start_pos[3], end_pos[2], end_pos[3]
end

local get_text_selection = function(opts)
	if opts.range > 0 then
		local start_line, start_col, end_line, end_col = get_selection_positions()
		local lines = vim.api.nvim_buf_get_lines(0, start_line - 1, end_line, false)

		-- handle selection base on line count
		if #lines == 1 then
			-- single-line selection
			lines[1] = lines[1]:sub(start_col, end_col)
		else
			-- multi line selection
			lines[1] = lines[1]:sub(start_col)
			lines[#lines] = lines[#lines]:sub(1, end_col)
		end
		return table.concat(lines, "\n")
	end

	-- not in visual mode, get current line
	return vim.fn.getline(".")
end

local capture_note_ctx = function(opts)
	local project_directory = vim.lsp.client.root_dir or vim.fn.getcwd()

	return {
		text_selection = get_text_selection(opts),
		filename = vim.fn.expand("%:p"),
		project_directory = project_directory,
	}
end

local parse_rg_output = function(output)
	local todos = {}
	for line in output:gmatch("[^\r\n]+") do
		local file, line_num, content = line:match("([^:]+):(%d+):(.*)")
		if file and line_num and content then
			table.insert(todos, {
				file_path = file,
				line_number = tonumber(line_num),
				content = content:gsub("^%s+", ""),
			})
		end
	end

	return todos
end

local sync_scan_todos = function()
	local result = vim.system(
		{ "rg", "TODO|FIXME|HACK", "--line-number", "--with-filename", "--no-heading" },
		{ text = true }
	)
		:wait()
	if result.code == 0 then
		return parse_rg_output(result.stdout)
	elseif result.code == 1 then
		-- ripgrep returns 1 when no matches where found (not an error)
		return {}
	else
		vim.notify("ripgrep error:" .. (result.stderr or "unknown"), vim.log.levels.ERROR)
		return {}
	end
end

local send_tcp = function(client, payload)
	local payload_body = vim.json.encode(payload)
	local payload_len = string.len(payload_body)
	local json_payload = string.format("Content-Length: %d\r\n\r\n", payload_len)
	vim.uv.write(client, json_payload, function(err)
		if err then
			vim.notify("error sending message header: " .. err, vim.log.levels.WARN)
		end
	end)

	vim.uv.write(client, payload_body, function(err)
		if err then
			vim.notify("error sending message content: " .. err, vim.log.levels.WARN)
		end
	end)
end

local connect_to_backend = function(req, opts)
	local addr = opts.addr or "127.0.0.1"
	local port = opts.port or 8080
	local client = vim.uv.new_tcp()
	if not client then
		return nil
	end

	vim.uv.tcp_connect(client, addr, port, function(err)
		if err then
			vim.notify("error connecting to contextual backend: " .. err, vim.log.levels.WARN)
		end

		send_tcp(client, req)

		vim.uv.read_start(client, function(err, data)
			-- vim.print("received: ", err, data)
			if err then
				vim.notify("error reading response: " .. err, vim.log.levels.WARN)
			elseif data then
				vim.print(data)
				vim.uv.shutdown(client)
			else
				vim.uv.close(client)
			end
		end)
	end)

	return client
end

M.new_note = function(opts)
	local note_ctx = capture_note_ctx(opts)
	print(note_ctx)
end

M.sync_todos = function(opts)
	local result = sync_scan_todos()
	local req = jsonrpc.NewJsonRpcRequest(5, "contextual/newTodo", result)
	local client = connect_to_backend(req, {})
	if not client then
		vim.notify("failed to create tcp client", vim.log.levels.WARN)
		return
	end
end

return M
