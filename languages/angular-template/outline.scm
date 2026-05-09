;; --- Documentation ---

; Annotation: <!-- ... -->
;             ^^^^^^^^^^^^
((comment) @annotation)



;; --- Elements ---

; HTML Element: <button>...</button>
;               ^^^^^^^^
((element
    (start_tag
        (tag_name) @name)) @item)

; Self-Closing Tag: <input />
;                   ^^^^^
((self_closing_tag
    (tag_name) @name) @item)



;; --- Control Flows ---

; If Statement: @if (...) { }
;               ^^^
((if_statement
    "@" @name
    (control_keyword) @name) @item)

; Else If Statement: @else if (...) { }
;                    ^^^^^^^
((else_if_statement
    "@" @name
    (control_keyword) @name
    (control_keyword) @name) @item)

; Else Statement: @else { }
;                 ^^^^
((else_statement
    "@" @name
    (control_keyword) @name) @item)

; For Statement: @for (...) { }
;                ^^^^
((for_statement
    "@" @name
    (control_keyword) @name) @item)

; Empty Statement: @empty { }
;                  ^^^^^^
((empty_statement
    "@" @name
    (control_keyword) @name) @item)

; Switch Statement: @switch (...) { }
;                   ^^^^^^^
((switch_statement
    "@" @name
    (control_keyword) @name) @item)

; Case Statement: @case (...) { }
;                 ^^^^^
((case_statement
    "@" @name
    (control_keyword) @name) @item)

; Default Statement: @default { }
;                    ^^^^^^^^
((default_statement
    "@" @name
    (control_keyword) @name) @item)



;; --- Defer Blocks ---

; Defer Statement: @defer { }
;                  ^^^^^^
((defer_statement
    "@" @name
    (control_keyword) @name) @item)

; Placeholder Statement: @placeholder { }
;                        ^^^^^^^^^^^^
((placeholder_statement
    "@" @name
    (control_keyword) @name) @item)

; Loading Statement: @loading { }
;                    ^^^^^^^^
((loading_statement
    "@" @name
    (control_keyword) @name) @item)

; Error Statement: @error { }
;                  ^^^^^^
((error_statement
    "@" @name
    (control_keyword) @name) @item)



;; --- Directives ---

; Structural: *ngIf="condition"
;             ^^^^^
((structural_directive
    (identifier) @name) @item)



;; --- Variables ---

; Template Reference: <input #username />
;                            ^^^^^^^^^
((attribute
  (attribute_name) @name (#match? @name "^#")) @item)
