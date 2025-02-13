require("lspconfig")["rust_analyzer"].setup({
	settings = {
		["rust-analyzer"] = {
			-- diagnostics = { disabled = { 'inactive-code' } },
			cargo = { features = "all", target = "x86_64-unknown-linux-gnu" },
		},
	},
})

require("lspconfig")
