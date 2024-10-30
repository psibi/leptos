;;; Directory Local Variables            -*- no-byte-compile: t -*-
;;; For more information see (info "(emacs) Directory Variables")

((rustic-mode .
	   ((lsp-rust-analyzer-rustfmt-override-command . '["leptosfmt", "./**/*.rs"])
	    (lsp-rust-analyzer-cargo-target . "wasm32-unknown-unknown")
	    (lsp-rust-sysroot . "discover")
	    (lsp-rust-analyzer-server-command . ("rustup" "run" "nightly-2024-10-27" "rust-analyzer")))))

;; 	    (lsp-rust-sysroot . "discover")
;; )))
