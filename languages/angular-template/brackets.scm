; Standard HTML and expression brackets
("{" @open "}" @close)
("(" @open ")" @close)
("[" @open "]" @close)

; Interpolation delimiters
("{{" @open "}}" @close)

; Two-way binding delimiters
("[(" @open ")]" @close)

; HTML tag angle brackets — excluded from rainbow coloring because they
; represent document structure rather than expression nesting.
("<" @open ">" @close (#set! rainbow.exclude))
