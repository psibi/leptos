;;; Directory Local Variables            -*- no-byte-compile: t -*-
;;; For more information see (info "(emacs) Directory Variables")

((rustic-mode .
	   ((lsp-rust-analyzer-rustfmt-override-command . '["leptosfmt", "./**/*.rs"])
	    (lsp-rust-analyzer-cargo-target "wasm32-unknown-unknown"))))
