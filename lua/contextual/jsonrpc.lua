local M = {}
---@class JsonRpcRequest
---@field jsonrpc string
---@field id integer
---@field method string
---@field params table

---@class JsonRpcResponse
---@field jsonrpc string
---@field id integer
---@field result table|nil
---@field error JsonRpcResponseError|nil

---@class JsonRpcResponseError
---@field code integer
---@field message string

---@param id integer
---@param method string
---@param params table
---@return JsonRpcRequest
M.NewJsonRpcRequest = function(id, method, params)
	return {
		jsonrpc = "2.0",
		id = id,
		method = method,
		params = params,
	}
end

local parse_header = function(response)
	local content_length = response:match("Content%-Length:%s*(%d+)")
	if not content_length then
		vim.notify("No Content-Length found", vim.log.levels.WARN)
		return nil
	end

	content_length = tonumber(content_length)
	local header_end = response:find("\r\n\r\n", 1, true)
	if not header_end then
		vim.notify("No header terminator found", vim.log.levels.WARN)
	end

	local body = response:sub(header_end + 4, header_end + 3 + content_length)

	return content_length, body
end

---@return JsonRpcResponse|nil
M.ParseJsonRpcResponse = function(response_payload)
	return vim.json.decode(response_payload)
end

return M
