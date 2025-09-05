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

---@param response_payload string
---@return JsonRpcResponse|nil
M.ParseJsonRpcResponse = function(response_payload)
	return vim.json.decode(response_payload)
end

return M
